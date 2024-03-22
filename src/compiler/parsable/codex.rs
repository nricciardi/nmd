pub mod parsing_rule;
pub mod codex_configuration;
pub mod modifier;

use std::collections::HashMap;
use std::sync::Arc;

pub use parsing_rule::ParsingRule;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use regex::{Captures, Regex};
use self::modifier::modifiers_bucket::ModifiersBucket;
use self::modifier::paragraph_modifier::{self, ParagraphModifier};
use self::modifier::text_modifier::TextModifier;
use self::modifier::ModifierIdentifier;
pub use self::modifier::{MAX_HEADING_LEVEL, Modifier};
use self::parsing_rule::html_extended_block_quote_rule::HtmlExtendedBlockQuoteRule;
use self::parsing_rule::html_list_rule::HtmlListRule;
use crate::compiler::dossier::document::chapter::paragraph::ParagraphError;
use crate::compiler::dossier::document::Paragraph;
use crate::compiler::output_format::OutputFormat;
use crate::compiler::parsable::codex::modifier::Mod;
use self::codex_configuration::CodexConfiguration;
use self::parsing_rule::html_image_rule::HtmlImageRule;
use self::parsing_rule::parsing_outcome::{ParsingError, ParsingOutcome};
use self::parsing_rule::replacement_rule::ReplacementRule;
use super::ParsingConfiguration;

pub const PARAGRAPH_SEPARATOR: &str = r"(?m:^\n[ \t]*){1}";

/// Ordered collection of rules
/// A **rule** is defined as the actual text transformation
pub struct Codex {
    configuration: CodexConfiguration,
    text_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>>,
    paragraph_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>>,
    chapter_rules: Vec<Box<dyn ParsingRule>>,
    document_rules: Vec<Box<dyn ParsingRule>>,
    
}

impl Codex {

    pub fn from(format: &OutputFormat, configuration: CodexConfiguration) -> Self {
        match format {
            OutputFormat::Html => Self::of_html(configuration)
        }
    }

    pub fn text_rules(&self) -> &HashMap<ModifierIdentifier, Box<dyn ParsingRule>> {
        &self.text_rules
    }

    pub fn paragraph_rules(&self) -> &HashMap<ModifierIdentifier, Box<dyn ParsingRule>> {
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

        let excluded_modifiers = parsing_configuration.modifiers_excluded().clone();

        self.parse_content_excluding_modifiers(content, Arc::clone(&parsing_configuration), excluded_modifiers)
    }

    pub fn parse_content_excluding_modifiers(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>, mut excluded_modifiers: ModifiersBucket) -> Result<ParsingOutcome, ParsingError> {

        log::debug!("start to parse content:\n{}\nexcluding: {:?}", content, excluded_modifiers);

        let mut outcome = ParsingOutcome::new(String::from(content));

        if excluded_modifiers == ModifiersBucket::All {
            log::debug!("parsing of content:\n{} is skipped are excluded all modifiers", content);
            
            return Ok(outcome)
        }

        for text_rule in self.text_rules() {

            if excluded_modifiers.contains(text_rule.modifier()) {

                log::debug!("{:?}: '{}' content search pattern is skipped", text_rule.modifier(), text_rule.modifier().search_pattern());
                continue;
            }

            if text_rule.is_match(content) {
                log::debug!("there is a match with {:?}: '{}' (content search pattern)", text_rule.modifier(), text_rule.modifier().search_pattern());

                outcome = text_rule.parse(outcome.parsed_content(), Arc::clone(&parsing_configuration))?;
    
                excluded_modifiers = excluded_modifiers + text_rule.incompatible_modifiers().clone();

                if excluded_modifiers == ModifiersBucket::All {
                    log::debug!("all next modifiers will be skipped because {:?} excludes {:?}", text_rule.modifier(), ModifiersBucket::All)
                }

            } else {
                log::debug!("no matches with {:?}: '{}' (content search pattern)", text_rule.modifier(), text_rule.modifier().search_pattern());
            }
            
        }

        Ok(outcome)
    }

    pub fn parse_paragraph(&self, paragraph: &Paragraph, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {
        self.parse_paragraph_excluding_modifiers(paragraph, parsing_configuration, ModifiersBucket::None)
    }


    pub fn parse_paragraph_excluding_modifiers(&self, paragraph: &Paragraph, parsing_configuration: Arc<ParsingConfiguration>, mut excluded_modifiers: ModifiersBucket) -> Result<ParsingOutcome, ParsingError> {

        log::debug!("start to parse paragraph:\n'{}'\nexcluding: {:?}", paragraph, excluded_modifiers);

        let mut outcome = ParsingOutcome::new(String::from(paragraph.content()));

        if excluded_modifiers == ModifiersBucket::All {
            log::debug!("parsing of paragraph:\n{} is skipped are excluded all modifiers", paragraph);
            
            return Ok(outcome)
        }

        let paragraph_rule = self.paragraph_rules().get(paragraph.paragraph_type());

        if let Some(paragraph_rule) = paragraph_rule {
            let search_pattern = paragraph_rule.search_pattern();

            log::debug!("'{}' paragraph search pattern that is about to be tested", search_pattern);

            outcome = paragraph_rule.parse(outcome.parsed_content(), Arc::clone(&parsing_configuration))?;

            excluded_modifiers = excluded_modifiers + paragraph_rule.incompatible_modifiers().clone();

        } else {

            log::warn!("there is NOT a paragraph rule for '{}' in codex", paragraph.paragraph_type());
        }

        // for paragraph_rule in self.paragraph_rules() {

        //     let search_pattern = paragraph_rule.modifier().search_pattern();

        //     log::debug!("{:?}: '{}' paragraph search pattern that is about to be tested", paragraph_rule.modifier(), search_pattern);

        //     if paragraph_rule.is_match(outcome.parsed_content()) {

        //         log::debug!("there is a match with '{}'", search_pattern);

        //         outcome = paragraph_rule.parse(outcome.parsed_content(), Arc::clone(&parsing_configuration))?;

        //         excluded_modifiers = excluded_modifiers + paragraph_rule.incompatible_modifiers().clone();

        //         break;      // ONLY ONE paragraph modifier
        //     } else {

        //         log::debug!("there is NOT a match with '{}'", search_pattern);
        //     }
        // }

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

        content = content.replace("\n\n", "\n\n\n");

        // work-around to fix paragraph matching end line
        while !content.ends_with("\n\n") {
            content.push_str("\n");
        }

        for paragraph_modifier in self.configuration.ordered_paragraph_modifiers() {

            let search_pattern = format!(r"{}{}{}", PARAGRAPH_SEPARATOR, paragraph_modifier.search_pattern(), PARAGRAPH_SEPARATOR);

            log::debug!("test {}", search_pattern);

            let regex = Regex::new(&search_pattern).unwrap();

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
                    log::debug!("discarded paragraph:\n{}\nbecause there is an overlap between {} and {} using pattern {:?}:\n{:#?}\n", m.as_str(), start, end, search_pattern, p);
                    return
                }

                log::debug!("found paragraph between {} and {}:\n{}\nusing {:?}", start, end, matched_str, &paragraph_modifier);

                let paragraph = Paragraph::new(matched_str, paragraph_modifier.identifier().clone());

                if !paragraph.contains_only_newlines() {
                    paragraphs.push((start, end, paragraph));
                }

            });
        }

        paragraphs.par_sort_by(|a, b| a.0.cmp(&b.1));

        Ok(paragraphs.iter().map(|p| p.2.to_owned()).collect())
    }

    fn new(configuration: CodexConfiguration, text_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>>, paragraph_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>>, chapter_rules: Vec<Box<dyn ParsingRule>>, document_rules: Vec<Box<dyn ParsingRule>>) -> Codex {

        // TODO: check if there are all necessary rules based on theirs type

        Codex {
            configuration,
            text_rules,
            paragraph_rules,
            chapter_rules,
            document_rules
        }
    }

    pub fn of_html(configuration: CodexConfiguration) -> Self {

        let mut content_rules: Vec<Box<dyn ParsingRule>> = Vec::new();

        for i in (1..=MAX_HEADING_LEVEL).rev() {
            content_rules.push(Box::new(ReplacementRule::new(Modifier::HeadingGeneralExtendedVersion(i), move |caps: &Captures| {
                let title = &caps[1];

                let id = Self::create_id(title);

                format!(r#"<h{} class="heading-{}" id="{}">{}</h{}>"#, i, i, id, title, i)
            })));

            content_rules.push(Box::new(ReplacementRule::new(Modifier::HeadingGeneralCompactVersion(i), |caps: &Captures| {
                let heading_lv = &caps[1];
                let title = &caps[2];

                let id = Self::create_id(title);

                format!(r#"<h{} class="heading-{}" id="{}">{}</h>"#, heading_lv, heading_lv, id, title)
            })));

            // content_rules.push(Box::new(ReplacementRule::new(Modifier::HeadingGeneralExtendedVersion(i), format!(r#"<h{} class="heading-{}">$1</h{}>"#, i, i, i))));
            // content_rules.push(Box::new(ReplacementRule::new(Modifier::HeadingGeneralCompactVersion(i), String::from(r#"<h${1} class="heading-${1}">$2</h$>"#))));
        }

        let text_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>> = HashMap::from([
            (
                TextModifier::Todo.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::Todo, String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))) as Box<dyn ParsingRule>,
            ),
            (
                TextModifier::AbridgedTodo.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::AbridgedTodo, String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))),
            ),
            (
                TextModifier::BookmarkWithId.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::BookmarkWithId, String::from(r#"<div class="bookmark" id="$2"><div class="bookmark-title">$1</div><div class="bookmark-description">$3</div></div>"#))),
            ),
            (
                TextModifier::Bookmark.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::Bookmark, String::from(r#"<div class="bookmark"><div class="bookmark-title">$1</div><div class="bookmark-description">$2</div></div>"#))),
            ),
            (
                TextModifier::AbridgedBookmarkWithId.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::AbridgedBookmarkWithId, String::from(r#"<div class="abridged-bookmark" id="$2"><div class="abridged-bookmark-title">$1</div></div>"#))),
            ),
            (
                TextModifier::AbridgedBookmark.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::AbridgedBookmark, String::from(r#"<div class="abridged-bookmark"><div class="abridged-bookmark-title">$1</div></div>"#))),
            ),
            (
                TextModifier::EmbeddedStyleWithId.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::EmbeddedStyleWithId, String::from(r#"<span class="identifier embedded-style" id="$2" style="$3">$1</span>"#))),
            ),
            (
                TextModifier::EmbeddedStyle.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::EmbeddedStyle, String::from(r#"<span class="embedded-style" style="$2">$1</span>"#))),
            ),
            (
                TextModifier::AbridgedEmbeddedStyleWithId.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::AbridgedEmbeddedStyleWithId, String::from(r#"<span class="identifier abridged-embedded-style" id="$2" style="color: $3; background-color: $4; font-family: $5;">$1</span>"#))),
            ),
            (
                TextModifier::AbridgedEmbeddedStyle.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::AbridgedEmbeddedStyle, String::from(r#"<span class="abridged-embedded-style" style="color: $2; background-color: $3; font-family: $4;">$1</span>"#))),
            ),
            (
                TextModifier::Identifier.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::Identifier, String::from(r#"<span class="identifier" id="$2">$1</span>"#))),
            ),
            (
                TextModifier::Highlight.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::Highlight, String::from(r#"<mark class="highlight">$1</mark>"#))),
            ),
            (
                TextModifier::InlineMath.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::InlineMath, String::from(r#"<span class="inline-math">$$${1}$$</span>"#))),
            ),
            (
                TextModifier::InlineCode.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::InlineCode, String::from(r#"<code class="language-markup inline-code">${1}</code>"#))),
            ),
            (
                TextModifier::BoldStarVersion.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::BoldStarVersion, String::from(r#"<strong class="bold">${1}</strong>"#))),
            ),
            (
                TextModifier::BoldUnderscoreVersion.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::BoldUnderscoreVersion, String::from(r#"<strong class="bold">${1}</strong>"#))),
            ),
            (
                TextModifier::ItalicStarVersion.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::ItalicStarVersion, String::from(r#"<em class="italic">${1}</em>"#))),
            ),
            (
                TextModifier::ItalicUnderscoreVersion.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::ItalicUnderscoreVersion, String::from(r#"<em class="italic">${1}</em>"#))),
            ),
            (
                TextModifier::Strikethrough.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::Strikethrough, String::from(r#"<del class="strikethrough">${1}</del>"#))),
            ),
            (
                TextModifier::Underlined.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::Underlined, String::from(r#"<u class="underlined">${1}</u>"#))),
            ),
            (
                TextModifier::Superscript.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::Superscript, String::from(r#"<sup class="superscript">${1}</sup>"#))),
            ),
            (
                TextModifier::Subscript.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::Subscript, String::from(r#"<sub class="subscript">${1}</sub>"#))),
            ),
            (
                TextModifier::Link.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::Link, String::from(r#"<a href=\"$2\" class="link">${1}</a>"#))),
            ),
            (
                TextModifier::Comment.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::Checkbox, String::from(r#"<div class="checkbox checkbox-unchecked"></div>"#))),
            ),
            (
                TextModifier::Checkbox.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::Todo, String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))) as Box<dyn ParsingRule>,
            ),
            (
                TextModifier::CheckboxChecked.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::CheckboxChecked, String::from(r#"<div class="checkbox checkbox-checked"></div>"#))),
            ),
            (
                TextModifier::Emoji.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::Emoji, String::from(r#"<i class="em-svg em-${1}" aria-role="presentation"></i>"#))),
            ),
        ]);

        // content_rules.append(&mut vec![
        //     Box::new(ReplacementRule::new(Modifier::Todo, String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))),
        //     Box::new(ReplacementRule::new(Modifier::AbridgedTodo, String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))),
        //     Box::new(ReplacementRule::new(Modifier::BookmarkWithId, String::from(r#"<div class="bookmark" id="$2"><div class="bookmark-title">$1</div><div class="bookmark-description">$3</div></div>"#))),
        //     Box::new(ReplacementRule::new(Modifier::Bookmark, String::from(r#"<div class="bookmark"><div class="bookmark-title">$1</div><div class="bookmark-description">$2</div></div>"#))),
        //     Box::new(ReplacementRule::new(Modifier::AbridgedBookmarkWithId, String::from(r#"<div class="abridged-bookmark" id="$2"><div class="abridged-bookmark-title">$1</div></div>"#))),
        //     Box::new(ReplacementRule::new(Modifier::AbridgedBookmark, String::from(r#"<div class="abridged-bookmark"><div class="abridged-bookmark-title">$1</div></div>"#))),
        //     Box::new(ReplacementRule::new(Modifier::EmbeddedStyleWithId, String::from(r#"<span class="identifier embedded-style" id="$2" style="$3">$1</span>"#))),
        //     Box::new(ReplacementRule::new(Modifier::EmbeddedStyle, String::from(r#"<span class="embedded-style" style="$2">$1</span>"#))),
        //     Box::new(ReplacementRule::new(Modifier::AbridgedEmbeddedStyleWithId, String::from(r#"<span class="identifier abridged-embedded-style" id="$2" style="color: $3; background-color: $4; font-family: $5;">$1</span>"#))),
        //     Box::new(ReplacementRule::new(Modifier::AbridgedEmbeddedStyle, String::from(r#"<span class="abridged-embedded-style" style="color: $2; background-color: $3; font-family: $4;">$1</span>"#))),
        //     Box::new(ReplacementRule::new(Modifier::Identifier, String::from(r#"<span class="identifier" id="$2">$1</span>"#))),
        //     Box::new(ReplacementRule::new(Modifier::Highlight, String::from(r#"<mark class="highlight">$1</mark>"#))),
        //     Box::new(ReplacementRule::new(Modifier::InlineMath, String::from(r#"<span class="inline-math">$$${1}$$</span>"#))),
        //     Box::new(ReplacementRule::new(Modifier::InlineCode, String::from(r#"<code class="language-markup inline-code">${1}</code>"#))),
        //     Box::new(ReplacementRule::new(Modifier::BoldStarVersion, String::from(r#"<strong class="bold">${1}</strong>"#))),
        //     Box::new(ReplacementRule::new(Modifier::BoldUnderscoreVersion, String::from(r#"<strong class="bold">${1}</strong>"#))),
        //     Box::new(ReplacementRule::new(Modifier::ItalicStarVersion, String::from(r#"<em class="italic">${1}</em>"#))),
        //     Box::new(ReplacementRule::new(Modifier::ItalicUnderscoreVersion, String::from(r#"<em class="italic">${1}</em>"#))),
        //     Box::new(ReplacementRule::new(Modifier::Strikethrough, String::from(r#"<del class="strikethrough">${1}</del>"#))),
        //     Box::new(ReplacementRule::new(Modifier::Underlined, String::from(r#"<u class="underlined">${1}</u>"#))),
        //     Box::new(ReplacementRule::new(Modifier::Superscript, String::from(r#"<sup class="superscript">${1}</sup>"#))),
        //     Box::new(ReplacementRule::new(Modifier::Subscript, String::from(r#"<sub class="subscript">${1}</sub>"#))),
        //     Box::new(ReplacementRule::new(Modifier::Link, String::from(r#"<a href=\"$2\" class="link">${1}</a>"#))),
        //     Box::new(ReplacementRule::new(Modifier::Comment, String::from(r#"<!-- ${1} -->"#))),
        //     Box::new(ReplacementRule::new(Modifier::Checkbox, String::from(r#"<div class="checkbox checkbox-unchecked"></div>"#))),
        //     Box::new(ReplacementRule::new(Modifier::CheckboxChecked, String::from(r#"<div class="checkbox checkbox-checked"></div>"#))),
        //     Box::new(ReplacementRule::new(Modifier::Emoji, String::from(r#"<i class="em-svg em-${1}" aria-role="presentation"></i>"#))),
        // ]);

        let paragraph_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>> = HashMap::from([
            (
                ParagraphModifier::PageBreak.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::PageBreak, String::from(r#"<div class="page-break"></div>"#))) as Box<dyn ParsingRule>
            ),
            (
                ParagraphModifier::EmbeddedParagraphStyleWithId.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::EmbeddedParagraphStyleWithId, String::from(r#"<div class="identifier embedded-paragraph-style" id="$2" style="$3">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                ParagraphModifier::EmbeddedParagraphStyle.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::EmbeddedParagraphStyle, String::from(r#"<div class="embedded-paragraph-style" style="$2">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                ParagraphModifier::AbridgedEmbeddedParagraphStyleWithId.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::AbridgedEmbeddedParagraphStyleWithId, String::from(r#"<div class="identifier abridged-embedded-paragraph-style" id="$2" style="color: $3; background-color: $4; font-family: $5;">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                ParagraphModifier::AbridgedEmbeddedParagraphStyle.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::AbridgedEmbeddedParagraphStyle, String::from(r#"<div class="abridged-embedded-paragraph-style" style="color: $2; background-color: $3; font-family: $4;">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                ParagraphModifier::ParagraphIdentifier.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::ParagraphIdentifier, String::from(r#"<span class="identifier" id="$2">$1</span>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                ParagraphModifier::ExtendedBlockQuote.identifier().clone(),
                Box::new(HtmlExtendedBlockQuoteRule::new()),
            ),
            (
                ParagraphModifier::MathBlock.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::MathBlock, String::from(r#"<p class="math-block">$$$$${1}$$$$</p>"#))),
            ),
            (
                ParagraphModifier::Image.identifier().clone(),
                Box::new(HtmlImageRule::new())
            ),
            (
                ParagraphModifier::CodeBlock.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::CodeBlock, String::from(r#"<pre><code class="language-${1} code-block">$2</code></pre>"#))),
            ),
            (
                ParagraphModifier::List.identifier().clone(),
                Box::new(HtmlListRule::new()),
            ),
            (
                ParagraphModifier::FocusBlock.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::FocusBlock, String::from(r#"<div class="focus-block focus-block-$1"><div class="focus-block-title focus-block-$1-title"></div><div class="focus-block-description focus-block-$1-description"">$2</div></div>"#)).with_newline_fix(r"<br>".to_string()))
            ),
            (
                ParagraphModifier::LineBreakDash.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::LineBreakDash, String::from(r#"<hr class="line-break line-break-dash">"#)))
            ),
            (
                ParagraphModifier::LineBreakStar.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::LineBreakStar, String::from(r#"<hr class="line-break line-break-star">"#)))
            ),
            (
                ParagraphModifier::LineBreakPlus.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::LineBreakPlus, String::from(r#"<hr class="line-break line-break-plus">"#)))
            ),
            (
                ParagraphModifier::CommonParagraph.identifier().clone(),
                Box::new(ReplacementRule::new(Modifier::CommonParagraph, String::from(r#"<p class="paragraph">${1}</p>"#)))
            ),
        ]);

        // let paragraph_rules: Vec<Box<dyn ParsingRule>> = vec![
        //     Box::new(ReplacementRule::new(Modifier::PageBreak, String::from(r#"<div class="page-break"></div>"#))),
        //     Box::new(ReplacementRule::new(Modifier::EmbeddedParagraphStyleWithId, String::from(r#"<div class="identifier embedded-paragraph-style" id="$2" style="$3">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
        //     Box::new(ReplacementRule::new(Modifier::EmbeddedParagraphStyle, String::from(r#"<div class="embedded-paragraph-style" style="$2">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
        //     Box::new(ReplacementRule::new(Modifier::AbridgedEmbeddedParagraphStyleWithId, String::from(r#"<div class="identifier abridged-embedded-paragraph-style" id="$2" style="color: $3; background-color: $4; font-family: $5;">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
        //     Box::new(ReplacementRule::new(Modifier::AbridgedEmbeddedParagraphStyle, String::from(r#"<div class="abridged-embedded-paragraph-style" style="color: $2; background-color: $3; font-family: $4;">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
        //     Box::new(ReplacementRule::new(Modifier::ParagraphIdentifier, String::from(r#"<span class="identifier" id="$2">$1</span>"#)).with_newline_fix(r"<br>".to_string())),
        //     Box::new(HtmlExtendedBlockQuoteRule::new()),
        //     Box::new(ReplacementRule::new(Modifier::MathBlock, String::from(r#"<p class="math-block">$$$$${1}$$$$</p>"#))),
        //     Box::new(HtmlImageRule::new()),
        //     Box::new(ReplacementRule::new(Modifier::CodeBlock, String::from(r#"<pre><code class="language-${1} code-block">$2</code></pre>"#))),
        //     Box::new(HtmlListRule::new()),
        //     Box::new(ReplacementRule::new(Modifier::FocusBlock, String::from(r#"<div class="focus-block focus-block-$1"><div class="focus-block-title focus-block-$1-title"></div><div class="focus-block-description focus-block-$1-description"">$2</div></div>"#)).with_newline_fix(r"<br>".to_string())),
        //     Box::new(ReplacementRule::new(Modifier::LineBreakDash, String::from(r#"<hr class="line-break line-break-dash">"#))),
        //     Box::new(ReplacementRule::new(Modifier::LineBreakStar, String::from(r#"<hr class="line-break line-break-star">"#))),
        //     Box::new(ReplacementRule::new(Modifier::LineBreakPlus, String::from(r#"<hr class="line-break line-break-plus">"#))),
        //     Box::new(ReplacementRule::new(Modifier::CommonParagraph, String::from(r#"<p class="paragraph">${1}</p>"#))),
        // ];

        Self::new(configuration, text_rules, paragraph_rules, vec![], vec![])
    }

    pub fn heading_rules(&self) -> Vec<&Box<dyn ParsingRule>> {
        
        self.text_rules.iter().filter(|&rules| {
            Modifier::str_is_heading(content)
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

        for rule in codex.text_rules() {
            let result = rule.parse(parsing_result.as_str(), Arc::clone(&parsing_configuration)).unwrap();

            parsing_result = result.parsed_content().clone()
        }

        assert_eq!(parsing_result, expected_result);

        let nmd_text = "This is a simple *nmd* text for test";
        let expected_result = "This is a simple <em>nmd</em> text for test";
        let mut parsing_result = String::from(nmd_text);

        for rule in codex.text_rules() {
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

        for rule in codex.text_rules() {
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