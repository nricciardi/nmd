pub mod codex_configuration;
pub mod modifier;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use regex::{Captures, Regex};
use self::modifier::standard_chapter_modifier::StandardChapterModifier;
use self::modifier::modifiers_bucket::ModifiersBucket;
use self::modifier::standard_paragraph_modifier::{self, StandardParagraphModifier};
use self::modifier::standard_text_modifier::StandardTextModifier;
use self::modifier::{Modifier, ModifierIdentifier};
pub use self::modifier::MAX_HEADING_LEVEL;
use crate::compiler::dossier::document::chapter::heading::{Heading, HeadingLevel};
use crate::compiler::dossier::{Document, DocumentError};
use crate::compiler::output_format::OutputFormat;
use self::codex_configuration::CodexConfiguration;

use super::parser::parsing_rule::html_extended_block_quote_rule::HtmlExtendedBlockQuoteRule;
use super::parser::parsing_rule::html_image_rule::HtmlImageRule;
use super::parser::parsing_rule::html_list_rule::HtmlListRule;
use super::parser::parsing_rule::replacement_rule::ReplacementRule;
use super::parser::parsing_rule::ParsingRule;


/// Ordered collection of rules
/// A **rule** is defined as the actual text transformation
pub struct Codex {
    configuration: CodexConfiguration,
    text_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>>,
    paragraph_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>>,
    chapter_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>>,
    document_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>>,
    
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

    pub fn configuration(&self) -> &CodexConfiguration {
        &self.configuration
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

    // pub fn is_heading(&self, content: &str) -> bool {
    //     let chapter_modifiers = self.configuration.ordered_chapter_modifier();

    //     for chapter_modifier in chapter_modifiers {
    //         let regex = Regex::new(&chapter_modifier.search_pattern()).unwrap();

    //         if regex.is_match(content) {
    //             return true
    //         }
    //     }

    //     false
    // }
    
    fn new(configuration: CodexConfiguration, text_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>>, paragraph_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>>, chapter_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>>, document_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>>) -> Codex {

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

        let text_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>> = HashMap::from([
            (
                StandardTextModifier::Todo.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::Todo.modifier_pattern().clone(), String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))) as Box<dyn ParsingRule>,
            ),
            (
                StandardTextModifier::AbridgedTodo.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::AbridgedTodo.modifier_pattern().clone(), String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))),
            ),
            (
                StandardTextModifier::BookmarkWithId.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::BookmarkWithId.modifier_pattern().clone(), String::from(r#"<div class="bookmark" id="$2"><div class="bookmark-title">$1</div><div class="bookmark-description">$3</div></div>"#))),
            ),
            (
                StandardTextModifier::Bookmark.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::Bookmark.modifier_pattern().clone(), String::from(r#"<div class="bookmark"><div class="bookmark-title">$1</div><div class="bookmark-description">$2</div></div>"#))),
            ),
            (
                StandardTextModifier::AbridgedBookmarkWithId.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::AbridgedBookmarkWithId.modifier_pattern().clone(), String::from(r#"<div class="abridged-bookmark" id="$2"><div class="abridged-bookmark-title">$1</div></div>"#))),
            ),
            (
                StandardTextModifier::AbridgedBookmark.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::AbridgedBookmark.modifier_pattern().clone(), String::from(r#"<div class="abridged-bookmark"><div class="abridged-bookmark-title">$1</div></div>"#))),
            ),
            (
                StandardTextModifier::EmbeddedStyleWithId.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::EmbeddedStyleWithId.modifier_pattern().clone(), String::from(r#"<span class="identifier embedded-style" id="$2" style="$3">$1</span>"#))),
            ),
            (
                StandardTextModifier::EmbeddedStyle.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::EmbeddedStyle.modifier_pattern().clone(), String::from(r#"<span class="embedded-style" style="$2">$1</span>"#))),
            ),
            (
                StandardTextModifier::AbridgedEmbeddedStyleWithId.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::AbridgedEmbeddedStyleWithId.modifier_pattern().clone(), String::from(r#"<span class="identifier abridged-embedded-style" id="$2" style="color: $3; background-color: $4; font-family: $5;">$1</span>"#))),
            ),
            (
                StandardTextModifier::AbridgedEmbeddedStyle.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::AbridgedEmbeddedStyle.modifier_pattern().clone(), String::from(r#"<span class="abridged-embedded-style" style="color: $2; background-color: $3; font-family: $4;">$1</span>"#))),
            ),
            (
                StandardTextModifier::Identifier.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::Identifier.modifier_pattern().clone(), String::from(r#"<span class="identifier" id="$2">$1</span>"#))),
            ),
            (
                StandardTextModifier::Highlight.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::Highlight.modifier_pattern().clone(), String::from(r#"<mark class="highlight">$1</mark>"#))),
            ),
            (
                StandardTextModifier::InlineMath.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::InlineMath.modifier_pattern().clone(), String::from(r#"<span class="inline-math">$$${1}$$</span>"#))),
            ),
            (
                StandardTextModifier::InlineCode.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::InlineCode.modifier_pattern().clone(), String::from(r#"<code class="language-markup inline-code">${1}</code>"#))),
            ),
            (
                StandardTextModifier::BoldStarVersion.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::BoldStarVersion.modifier_pattern().clone(), String::from(r#"<strong class="bold">${1}</strong>"#))),
            ),
            (
                StandardTextModifier::BoldUnderscoreVersion.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::BoldUnderscoreVersion.modifier_pattern().clone(), String::from(r#"<strong class="bold">${1}</strong>"#))),
            ),
            (
                StandardTextModifier::ItalicStarVersion.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::ItalicStarVersion.modifier_pattern().clone(), String::from(r#"<em class="italic">${1}</em>"#))),
            ),
            (
                StandardTextModifier::ItalicUnderscoreVersion.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::ItalicUnderscoreVersion.modifier_pattern().clone(), String::from(r#"<em class="italic">${1}</em>"#))),
            ),
            (
                StandardTextModifier::Strikethrough.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::Strikethrough.modifier_pattern().clone(), String::from(r#"<del class="strikethrough">${1}</del>"#))),
            ),
            (
                StandardTextModifier::Underlined.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::Underlined.modifier_pattern().clone(), String::from(r#"<u class="underlined">${1}</u>"#))),
            ),
            (
                StandardTextModifier::Superscript.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::Superscript.modifier_pattern().clone(), String::from(r#"<sup class="superscript">${1}</sup>"#))),
            ),
            (
                StandardTextModifier::Subscript.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::Subscript.modifier_pattern().clone(), String::from(r#"<sub class="subscript">${1}</sub>"#))),
            ),
            (
                StandardTextModifier::Link.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::Link.modifier_pattern().clone(), String::from(r#"<a href=\"$2\" class="link">${1}</a>"#))),
            ),
            (
                StandardTextModifier::Comment.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::Comment.modifier_pattern().clone(), String::from(r#"<div class="checkbox checkbox-unchecked"></div>"#))),
            ),
            (
                StandardTextModifier::Checkbox.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::Checkbox.modifier_pattern().clone(), String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))) as Box<dyn ParsingRule>,
            ),
            (
                StandardTextModifier::CheckboxChecked.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::CheckboxChecked.modifier_pattern().clone(), String::from(r#"<div class="checkbox checkbox-checked"></div>"#))),
            ),
            (
                StandardTextModifier::Emoji.identifier().clone(),
                Box::new(ReplacementRule::new(StandardTextModifier::Emoji.modifier_pattern().clone(), String::from(r#"<i class="em-svg em-${1}" aria-role="presentation"></i>"#))),
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
                StandardParagraphModifier::PageBreak.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::PageBreak.modifier_pattern().clone(), String::from(r#"<div class="page-break"></div>"#))) as Box<dyn ParsingRule>
            ),
            (
                StandardParagraphModifier::EmbeddedParagraphStyleWithId.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::EmbeddedParagraphStyleWithId.modifier_pattern().clone(), String::from(r#"<div class="identifier embedded-paragraph-style" id="$2" style="$3">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                StandardParagraphModifier::EmbeddedParagraphStyle.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::EmbeddedParagraphStyle.modifier_pattern().clone(), String::from(r#"<div class="embedded-paragraph-style" style="$2">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                StandardParagraphModifier::AbridgedEmbeddedParagraphStyleWithId.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::AbridgedEmbeddedParagraphStyleWithId.modifier_pattern().clone(),  String::from(r#"<div class="identifier abridged-embedded-paragraph-style" id="$2" style="color: $3; background-color: $4; font-family: $5;">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                StandardParagraphModifier::AbridgedEmbeddedParagraphStyle.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::AbridgedEmbeddedParagraphStyle.modifier_pattern().clone(), String::from(r#"<div class="abridged-embedded-paragraph-style" style="color: $2; background-color: $3; font-family: $4;">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                StandardParagraphModifier::ParagraphIdentifier.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::ParagraphIdentifier.modifier_pattern().clone(), String::from(r#"<span class="identifier" id="$2">$1</span>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                StandardParagraphModifier::ExtendedBlockQuote.identifier().clone(),
                Box::new(HtmlExtendedBlockQuoteRule::new()),
            ),
            (
                StandardParagraphModifier::MathBlock.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::MathBlock.modifier_pattern().clone(), String::from(r#"<p class="math-block">$$$$${1}$$$$</p>"#))),
            ),
            (
                StandardParagraphModifier::Image.identifier().clone(),
                Box::new(HtmlImageRule::new())
            ),
            (
                StandardParagraphModifier::CodeBlock.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::CodeBlock.modifier_pattern().clone(),  String::from(r#"<pre><code class="language-${1} code-block">$2</code></pre>"#))),
            ),
            (
                StandardParagraphModifier::List.identifier().clone(),
                Box::new(HtmlListRule::new()),
            ),
            (
                StandardParagraphModifier::FocusBlock.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::FocusBlock.modifier_pattern().clone(), String::from(r#"<div class="focus-block focus-block-$1"><div class="focus-block-title focus-block-$1-title"></div><div class="focus-block-description focus-block-$1-description"">$2</div></div>"#)).with_newline_fix(r"<br>".to_string()))
            ),
            (
                StandardParagraphModifier::LineBreakDash.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::LineBreakDash.modifier_pattern().clone(), String::from(r#"<hr class="line-break line-break-dash">"#)))
            ),
            (
                StandardParagraphModifier::LineBreakStar.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::LineBreakStar.modifier_pattern().clone(), String::from(r#"<hr class="line-break line-break-star">"#)))
            ),
            (
                StandardParagraphModifier::LineBreakPlus.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::LineBreakPlus.modifier_pattern().clone(), String::from(r#"<hr class="line-break line-break-plus">"#)))
            ),
            (
                StandardParagraphModifier::CommonParagraph.identifier().clone(),
                Box::new(ReplacementRule::new(StandardParagraphModifier::CommonParagraph.modifier_pattern_with_paragraph_separator().clone(), String::from(r#"<p class="paragraph">${1}</p>"#)))
            ),
        ]);

        let chapter_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>> = HashMap::new();

        // for i in (1..=MAX_HEADING_LEVEL).rev() {
        //     chapter_rules.insert(StandardChapterModifier::HeadingGeneralExtendedVersion(i).identifier().clone(), 
        //     Box::new(ReplacementRule::new(StandardChapterModifier::HeadingGeneralExtendedVersion(i).modifier_pattern().clone(), move |caps: &Captures| {
        //         let title = &caps[1];

        //         let id = Self::create_id(title);

        //         format!(r#"<h{} class="heading-{}" id="{}">{}</h{}>"#, i, i, id, title, i)
        //     })));

        //     chapter_rules.insert(StandardChapterModifier::HeadingGeneralCompactVersion(i).identifier().clone(), 
        //     Box::new(ReplacementRule::new(StandardChapterModifier::HeadingGeneralCompactVersion(i).modifier_pattern().clone(), |caps: &Captures| {
        //         let heading_lv = &caps[1];
        //         let title = &caps[2];

        //         let id = Self::create_id(title);

        //         format!(r#"<h{} class="heading-{}" id="{}">{}</h>"#, heading_lv, heading_lv, id, title)
        //     })));

        //     // content_rules.push(Box::new(ReplacementRule::new(Modifier::HeadingGeneralExtendedVersion(i), format!(r#"<h{} class="heading-{}">$1</h{}>"#, i, i, i))));
        //     // content_rules.push(Box::new(ReplacementRule::new(Modifier::HeadingGeneralCompactVersion(i), String::from(r#"<h${1} class="heading-${1}">$2</h$>"#))));
        // }

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

        Self::new(configuration, text_rules, paragraph_rules, chapter_rules, HashMap::new())
    }

    // pub fn heading_rules(&self) -> Vec<&Box<dyn ParsingRule>> {
        
    //     self.text_rules.iter().filter(|&rules| {
    //         Modifier::str_is_heading(content)
    //     }).collect()
    // }
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use crate::compiler::{loader::Loader, parser::parsing_rule::parsing_configuration::ParsingConfiguration};

    use super::*;

    #[test]
    fn html_multiple_uses() {
        let codex: &Codex = &Codex::of_html(CodexConfiguration::default());

        let nmd_text = "This is a simple **nmd** text for test";
        let expected_result = "This is a simple <strong>nmd</strong> text for test";
        let mut parsing_result = String::from(nmd_text);
        let parsing_configuration = Arc::new(ParsingConfiguration::default());

        for rule in codex.text_rules() {
            let result = rule.1.parse(parsing_result.as_str(), Arc::clone(&parsing_configuration)).unwrap();

            parsing_result = result.parsed_content().clone()
        }

        assert_eq!(parsing_result, expected_result);

        let nmd_text = "This is a simple *nmd* text for test";
        let expected_result = "This is a simple <em>nmd</em> text for test";
        let mut parsing_result = String::from(nmd_text);

        for rule in codex.text_rules() {
            let result = rule.1.parse(parsing_result.as_str(), Arc::clone(&parsing_configuration)).unwrap();

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
            let result = rule.1.parse(parsing_result.as_str(), Arc::clone(&parsing_configuration)).unwrap();

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

        let paragraphs = Loader::load_paragraphs_from_str(&codex, nmd_text).unwrap();

        assert_eq!(paragraphs.len(), 2)
    }

    #[test]
    fn id() {
        let s = "my $string<-_778ks";

        let id = Codex::create_id(s);

        assert_eq!(id, "my-string-_778ks");
    }
}