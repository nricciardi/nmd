pub mod parsing_rule;
pub mod codex_configuration;
pub mod modifier;

use std::sync::Arc;

pub use parsing_rule::ParsingRule;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use regex::{Captures, Regex};
use self::modifier::chapter_modifier::ChapterModifier;
use self::modifier::paragraph_modifier::ParagraphModifier;
use self::modifier::text_modifier::TextModifier;
use self::modifier::Modifiers;
use self::modifier::{MAX_HEADING_LEVEL, Modifier};
use self::parsing_rule::html_extended_block_quote_rule::HtmlExtendedBlockQuoteRule;
use self::parsing_rule::html_list_rule::HtmlListRule;
use crate::compiler::dossier::document::chapter::paragraph::ParagraphError;
use crate::compiler::dossier::document::Paragraph;
use crate::compiler::output_format::OutputFormat;
use self::codex_configuration::CodexConfiguration;
use self::parsing_rule::html_image_rule::HtmlImageRule;
use self::parsing_rule::parsing_outcome::{ParsingError, ParsingOutcome};
use self::parsing_rule::replacement_rule::ReplacementRule;
use super::ParsingConfiguration;


/// Ordered collection of rules
pub struct Codex {
    configuration: CodexConfiguration,
    text_rules: Vec<Box<dyn ParsingRule>>,
    paragraph_rules: Vec<Box<dyn ParsingRule>>,
    chapter_rules: Vec<Box<dyn ParsingRule>>,
    document_rules: Vec<Box<dyn ParsingRule>>,
}

impl Codex {

    pub fn from(format: &OutputFormat, configuration: CodexConfiguration) -> Self {
        match format {
            OutputFormat::Html => Self::of_html(configuration)
        }
    }

    pub fn content_rules(&self) -> &Vec<Box<dyn ParsingRule>> {
        &self.text_rules
    }

    pub fn paragraph_rules(&self) -> &Vec<Box<dyn ParsingRule>> {
        &self.paragraph_rules
    }

    pub fn create_id(s: &str) -> String {

        let allowed_chars = s.chars().filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == ' ').map(|c| {
            if c == ' ' {
                return '-';
            }

            c
        });

        allowed_chars.collect()
    }

    pub fn parse_content(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {

        let excluded_modifiers = parsing_configuration.excluded_modifiers().clone();

        self.parse_content_excluding_modifiers(content, Arc::clone(&parsing_configuration), excluded_modifiers)
    }

    pub fn parse_content_excluding_modifiers(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>, mut excluded_modifiers: Modifiers) -> Result<ParsingOutcome, ParsingError> {

        log::debug!("start to parse content:\n{}\nexcluding: {:?}", content, excluded_modifiers);

        let mut outcome = ParsingOutcome::new(String::from(content));

        if excluded_modifiers == Modifiers::All {
            log::debug!("parsing of content:\n{} is skipped are excluded all modifiers", content);
            
            return Ok(outcome)
        }

        for content_rule in self.content_rules() {

            if excluded_modifiers.contains(content_rule.modifier()) {

                log::debug!("{:?}: '{}' content search pattern is skipped", content_rule.modifier(), content_rule.modifier().search_pattern());
                continue;
            }

            if content_rule.is_match(content) {
                log::debug!("there is a match with {:?}: '{}' (content search pattern)", content_rule.modifier(), content_rule.modifier().search_pattern());

                outcome = content_rule.parse(outcome.parsed_content(), Arc::clone(&parsing_configuration))?;
    
                excluded_modifiers = excluded_modifiers + content_rule.incompatible_modifiers().clone();

                if excluded_modifiers == Modifiers::All {
                    log::debug!("all next modifiers will be skipped because {:?} excludes {:?}", content_rule.modifier(), Modifiers::All)
                }

            } else {
                log::debug!("no matches with {:?}: '{}' (content search pattern)", content_rule.modifier(), content_rule.modifier().search_pattern());
            }
            
        }

        Ok(outcome)
    }

    pub fn parse_paragraph(&self, paragraph: &Paragraph, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {
        self.parse_paragraph_excluding_modifiers(paragraph, parsing_configuration, Modifiers::None)
    }


    pub fn parse_paragraph_excluding_modifiers(&self, paragraph: &Paragraph, parsing_configuration: Arc<ParsingConfiguration>, mut excluded_modifiers: Modifiers) -> Result<ParsingOutcome, ParsingError> {

        log::debug!("start to parse paragraph:\n{}\nexcluding: {:?}", paragraph, excluded_modifiers);

        let mut outcome = ParsingOutcome::new(String::from(paragraph.content()));

        if excluded_modifiers == Modifiers::All {
            log::debug!("parsing of paragraph:\n{} is skipped are excluded all modifiers", paragraph);
            
            return Ok(outcome)
        }

        for paragraph_rule in self.paragraph_rules() {

            let search_pattern = paragraph_rule.modifier().search_pattern();

            log::debug!("{:?}: '{}' paragraph search pattern that is about to be tested", paragraph_rule.modifier(), search_pattern);

            if paragraph_rule.is_match(outcome.parsed_content()) {

                log::debug!("there is a match with '{}'", search_pattern);

                outcome = paragraph_rule.parse(outcome.parsed_content(), Arc::clone(&parsing_configuration))?;

                excluded_modifiers = excluded_modifiers + paragraph_rule.incompatible_modifiers().clone();

                break;      // ONLY ONE paragraph modifier
            } else {

                log::debug!("there is NOT a match with '{}'", search_pattern);
            }
        }

        outcome = self.parse_content_excluding_modifiers(outcome.parsed_content(), Arc::clone(&parsing_configuration), excluded_modifiers)?;

        Ok(outcome)
    }

    fn count_newlines_at_start(s: &str) -> usize {
        s.bytes().take_while(|&b| b == b'\n').count()
    }

    fn count_newlines_at_end(s: &str) -> usize {
        s.bytes().rev().take_while(|&b| b == b'\n').count()
    }

    /// Split a string in the corresponding vector of paragraphs
    pub fn split_str_in_paragraphs(&self, content: &str) -> Result<Vec<Paragraph>, ParagraphError> {

        let mut paragraphs: Vec<(usize, usize, Paragraph)> = Vec::new();
        let mut content = String::from(content);

        // work-around to fix paragraph matching end line
        while !content.ends_with("\n\n") {
            content.push_str("\n");
        }

        for modifier in ParagraphModifier::ordered_paragraph_modifiers() {

            log::debug!("test {:?}", modifier);

            let regex = Regex::new(&modifier.search_pattern()).unwrap();

            regex.find_iter(content.clone().as_str()).for_each(|m| {

                let matched_str = m.as_str().to_string();

                let start = m.start() + Self::count_newlines_at_start(&matched_str);
                let end = m.end() - Self::count_newlines_at_end(&matched_str) - 1;

                let overlap_paragraph = paragraphs.par_iter().find_any(|p| {
                    (p.0 >= start && p.1 <= end) ||     // current paragraph contains p
                    (p.0 <= start && p.1 >= end) ||     // p contains current paragraph
                    (p.0 <= start && p.1 >= start && p.1 <= end) ||     // left overlap
                    (p.0 >= start && p.0 <= end && p.1 >= end)          // right overlap
                });

                if let Some(p) = overlap_paragraph {     // => overlap
                    log::debug!("discarded paragraph:\n{}\nbecause there is an overlap between {} and {} using pattern {:?}:\n{:#?}\n", m.as_str(), start, end, &modifier.search_pattern(), p);
                    return
                }

                log::debug!("found paragraph between {} and {}:\n{}\nusing {:?}", start, end, matched_str, &modifier);

                let paragraph = Paragraph::from(matched_str);

                if !paragraph.contains_only_newlines() {
                    paragraphs.push((start, end, paragraph));
                }

            });
        }

        paragraphs.par_sort_by(|a, b| a.0.cmp(&b.1));

        Ok(paragraphs.iter().map(|p| p.2.to_owned()).collect())
    }

    fn new(configuration: CodexConfiguration, content_rules: Vec<Box<dyn ParsingRule>>, paragraph_rules: Vec<Box<dyn ParsingRule>>, chapter_rules: Vec<Box<dyn ParsingRule>>, document_rules: Vec<Box<dyn ParsingRule>>) -> Codex {

        // TODO: check if there are all necessary rules based on theirs type

        Codex {
            configuration,
            text_rules: content_rules,
            paragraph_rules,
            chapter_rules,
            document_rules
        }
    }

    pub fn of_html(configuration: CodexConfiguration) -> Self {

        let mut content_rules: Vec<Box<dyn ParsingRule>> = Vec::new();

        for i in (1..=MAX_HEADING_LEVEL).rev() {
            content_rules.push(Box::new(ReplacementRule::new(Box::new(ChapterModifier::HeadingGeneralExtendedVersion(i)), move |caps: &Captures| {
                let title = &caps[1];

                let id = Self::create_id(title);

                format!(r#"<h{} class="heading-{}" id="{}">{}</h{}>"#, i, i, id, title, i)
            })));

            content_rules.push(Box::new(ReplacementRule::new(Box::new(ChapterModifier::HeadingGeneralCompactVersion(i)), |caps: &Captures| {
                let heading_lv = &caps[1];
                let title = &caps[2];

                let id = Self::create_id(title);

                format!(r#"<h{} class="heading-{}" id="{}">{}</h>"#, heading_lv, heading_lv, id, title)
            })));

            // content_rules.push(Box::new(ReplacementRule::new(Modifier::HeadingGeneralExtendedVersion(i), format!(r#"<h{} class="heading-{}">$1</h{}>"#, i, i, i))));
            // content_rules.push(Box::new(ReplacementRule::new(Modifier::HeadingGeneralCompactVersion(i), String::from(r#"<h${1} class="heading-${1}">$2</h$>"#))));
        }

        content_rules.append(&mut vec![
            Box::new(ReplacementRule::new(Box::new(TextModifier::Todo), String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::BookmarkWithId), String::from(r#"<div class="bookmark" id="$2"><div class="bookmark-title">$1</div><div class="bookmark-description">$3</div></div>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::Bookmark), String::from(r#"<div class="bookmark"><div class="bookmark-title">$1</div><div class="bookmark-description">$2</div></div>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::AbridgedBookmarkWithId), String::from(r#"<div class="abridged-bookmark" id="$2"><div class="abridged-bookmark-title">$1</div></div>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::AbridgedBookmark), String::from(r#"<div class="abridged-bookmark"><div class="abridged-bookmark-title">$1</div></div>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::EmbeddedStyleWithId), String::from(r#"<span class="identifier embedded-style" id="$2" style="$3">$1</span>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::EmbeddedStyle), String::from(r#"<span class="embedded-style" style="$2">$1</span>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::AbridgedEmbeddedStyleWithId), String::from(r#"<span class="identifier abridged-embedded-style" id="$2" style="color: $3; background-color: $4; font-family: $5;">$1</span>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::AbridgedEmbeddedStyle), String::from(r#"<span class="abridged-embedded-style" style="color: $2; background-color: $3; font-family: $4;">$1</span>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::Identifier), String::from(r#"<span class="identifier" id="$2">$1</span>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::Highlight), String::from(r#"<mark class="highlight">$1</mark>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::InlineMath), String::from(r#"<span class="inline-math">$$${1}$$</span>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::InlineCode), String::from(r#"<code class="language-markup inline-code">${1}</code>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::BoldStarVersion), String::from(r#"<strong class="bold">${1}</strong>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::BoldUnderscoreVersion), String::from(r#"<strong class="bold">${1}</strong>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::ItalicStarVersion), String::from(r#"<em class="italic">${1}</em>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::ItalicUnderscoreVersion), String::from(r#"<em class="italic">${1}</em>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::Strikethrough), String::from(r#"<del class="strikethrough">${1}</del>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::Underlined), String::from(r#"<u class="underlined">${1}</u>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::Superscript), String::from(r#"<sup class="superscript">${1}</sup>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::Subscript), String::from(r#"<sub class="subscript">${1}</sub>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::Link), String::from(r#"<a href=\"$2\" class="link">${1}</a>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::Comment), String::from(r#"<!-- ${1} -->"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::Checkbox), String::from(r#"<div class="checkbox checkbox-unchecked"></div>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::CheckboxChecked), String::from(r#"<div class="checkbox checkbox-checked"></div>"#))),
            Box::new(ReplacementRule::new(Box::new(TextModifier::Emoji), String::from(r#"<i class="em-svg em-${1}" aria-role="presentation"></i>"#))),
        ]);

        let paragraph_rules: Vec<Box<dyn ParsingRule>> = vec![
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::AbridgedTodo), String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::PageBreak), String::from(r#"<div class="page-break"></div>"#))),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::EmbeddedParagraphStyleWithId), String::from(r#"<div class="identifier embedded-paragraph-style" id="$2" style="$3">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::EmbeddedParagraphStyle), String::from(r#"<div class="embedded-paragraph-style" style="$2">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::AbridgedEmbeddedParagraphStyleWithId), String::from(r#"<div class="identifier abridged-embedded-paragraph-style" id="$2" style="color: $3; background-color: $4; font-family: $5;">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::AbridgedEmbeddedParagraphStyle), String::from(r#"<div class="abridged-embedded-paragraph-style" style="color: $2; background-color: $3; font-family: $4;">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::ParagraphIdentifier), String::from(r#"<span class="identifier" id="$2">$1</span>"#)).with_newline_fix(r"<br>".to_string())),
            Box::new(HtmlExtendedBlockQuoteRule::new()),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::MathBlock), String::from(r#"<p class="math-block">$$$$${1}$$$$</p>"#))),
            Box::new(HtmlImageRule::new()),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::CodeBlock), String::from(r#"<pre><code class="language-${1} code-block">$2</code></pre>"#))),
            Box::new(HtmlListRule::new()),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::FocusBlock), String::from(r#"<div class="focus-block focus-block-$1"><div class="focus-block-title focus-block-$1-title"></div><div class="focus-block-description focus-block-$1-description"">$2</div></div>"#)).with_newline_fix(r"<br>".to_string())),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::LineBreakDash), String::from(r#"<hr class="line-break line-break-dash">"#))),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::LineBreakStar), String::from(r#"<hr class="line-break line-break-star">"#))),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::LineBreakPlus), String::from(r#"<hr class="line-break line-break-plus">"#))),
            Box::new(ReplacementRule::new(Box::new(ParagraphModifier::CommonParagraph), String::from(r#"<p class="paragraph">${1}</p>"#))),
        ];

        Self::new(configuration, content_rules, paragraph_rules, vec![], vec![])
    }

    pub fn heading_rules(&self) -> Vec<&Box<dyn ParsingRule>> {
        
        self.text_rules.iter().filter(|&rules| {
            match rules.modifier() {
                ChapterModifier::HeadingGeneralCompactVersion(_) | ChapterModifier::HeadingGeneralExtendedVersion(_) => true,
                _ => false
            }
        }).collect()
    }
}






#[cfg(test)]
mod test {

    use std::sync::Arc;

    use crate::compiler::parsable::ParsingConfiguration;

    use super::*;

    #[test]
    fn html_multiple_uses() {
        let codex: &Codex = &Codex::of_html(CodexConfiguration::default());

        let nmd_text = "This is a simple **nmd** text for test";
        let expected_result = "This is a simple <strong>nmd</strong> text for test";
        let mut parsing_result = String::from(nmd_text);
        let parsing_configuration = Arc::new(ParsingConfiguration::default());

        for rule in codex.content_rules() {
            let result = rule.parse(parsing_result.as_str(), Arc::clone(&parsing_configuration)).unwrap();

            parsing_result = result.parsed_content().clone()
        }

        assert_eq!(parsing_result, expected_result);

        let nmd_text = "This is a simple *nmd* text for test";
        let expected_result = "This is a simple <em>nmd</em> text for test";
        let mut parsing_result = String::from(nmd_text);

        for rule in codex.content_rules() {
            let result = rule.parse(parsing_result.as_str(), Arc::clone(&parsing_configuration)).unwrap();

            parsing_result = result.parsed_content().clone()
        }

        assert_eq!(parsing_result, expected_result);
    }

    #[test]
    fn headings () {
        let codex: &Codex = &Codex::of_html(CodexConfiguration::default());

        let nmd_text = 
r#"
#1 title 1
## title 2
###### title 6
"#.trim();
        let expected_result = 
r#"
<h1 class="h1">title 1</h1>
<h2 class="h2">title 2</h2>
<h6 class="h6">title 6</h6>
"#.trim();

        let mut parsing_result = String::from(nmd_text);
        let parsing_configuration = Arc::new(ParsingConfiguration::default());

        for rule in codex.content_rules() {
            let result = rule.parse(parsing_result.as_str(), Arc::clone(&parsing_configuration)).unwrap();

            parsing_result = result.parsed_content().clone()
        }

        assert_eq!(parsing_result, expected_result);
    }


    #[test]
    fn split_str_in_paragraphs() {
        let codex: Codex = Codex::of_html(CodexConfiguration::default());

        let nmd_text = 
r#"
```python

print("hello world")

```

`print("hello world)`
"#.trim();

        let paragraphs = codex.split_str_in_paragraphs(nmd_text).unwrap();

        assert_eq!(paragraphs.len(), 2)
    }

    #[test]
    fn id() {
        let s = "my $string<-_778ks";

        let id = Codex::create_id(s);

        assert_eq!(id, "my-string-_778ks");
    }
}