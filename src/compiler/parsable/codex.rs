pub mod parsing_rule;
pub mod codex_configuration;
pub mod modifier;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

pub use parsing_rule::ParsingRule;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use regex::{Captures, Regex};
use self::modifier::chapter_modifier::ChapterModifier;
use self::modifier::modifiers_bucket::ModifiersBucket;
use self::modifier::paragraph_modifier::{self, ParagraphModifier};
use self::modifier::text_modifier::TextModifier;
use self::modifier::ModifierIdentifier;
pub use self::modifier::{MAX_HEADING_LEVEL, Modifier};
use self::parsing_rule::html_extended_block_quote_rule::HtmlExtendedBlockQuoteRule;
use self::parsing_rule::html_list_rule::HtmlListRule;
use crate::compiler::dossier::document::chapter::chapter_builder::ChapterBuilder;
use crate::compiler::dossier::document::chapter::heading::{Heading, HeadingLevel};
use crate::compiler::dossier::document::chapter::paragraph::ParagraphError;
use crate::compiler::dossier::document::{self, chapter, Chapter, Paragraph};
use crate::compiler::dossier::{Document, DocumentError};
use crate::compiler::output_format::OutputFormat;
use crate::compiler::parsable::codex::modifier::{text_modifier, Mod};
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

    pub fn create_id(s: &str) -> String {

        let allowed_chars = s.chars().filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == ' ').map(|c| {
            if c == ' ' {
                return '-';
            }

            c
        });

        allowed_chars.collect()
    }

    pub fn load_heading_from_str(&self, content: &str) -> Option<Heading> {
        let chapter_modifiers = self.configuration.ordered_chapter_modifier();

        for chapter_modifier in chapter_modifiers {
            let regex = Regex::new(&chapter_modifier.search_pattern()).unwrap();

            if regex.is_match(content) {
                let matched = regex.captures(content).unwrap();
                
                let level = HeadingLevel::from_str(matched.get(1).unwrap().as_str()).unwrap();
                let title = matched.get(2).unwrap().as_str();

                return Some(Heading::new(level, title.to_string()))
            }
        }

        Option::None
    }

    pub fn is_heading(&self, content: &str) -> bool {
        let chapter_modifiers = self.configuration.ordered_chapter_modifier();

        for chapter_modifier in chapter_modifiers {
            let regex = Regex::new(&chapter_modifier.search_pattern()).unwrap();

            if regex.is_match(content) {
                return true
            }
        }

        false
    }
}

impl Codex {

    pub fn parse_text(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {

        let excluded_modifiers = parsing_configuration.modifiers_excluded().clone();

        self.parse_text_excluding_modifiers(content, Arc::clone(&parsing_configuration), excluded_modifiers)
    }

    pub fn parse_text_excluding_modifiers(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>, mut excluded_modifiers: ModifiersBucket) -> Result<ParsingOutcome, ParsingError> {

        log::debug!("start to parse content:\n{}\nexcluding: {:?}", content, excluded_modifiers);

        let mut outcome = ParsingOutcome::new(String::from(content));

        if excluded_modifiers == ModifiersBucket::All {
            log::debug!("parsing of content:\n{} is skipped are excluded all modifiers", content);
            
            return Ok(outcome)
        }

        for text_modifier in self.configuration.ordered_text_modifiers() {

            if excluded_modifiers.contains(text_modifier) {

                log::debug!("{:?} is skipped", text_modifier);
                continue;
            }

            let text_rule = self.text_rules().get(text_modifier.identifier());

            if text_rule.is_none() {
                log::warn!("text rule for {:#?} not found", text_modifier);
                continue;
            }

            let text_rule = text_rule.unwrap();

            if text_rule.is_match(content) {
                log::debug!("there is a match with {:#?}", text_rule);

                outcome = text_rule.parse(outcome.parsed_content(), Arc::clone(&parsing_configuration))?;
    
                excluded_modifiers = excluded_modifiers + text_rule.incompatible_modifiers().clone();

                if excluded_modifiers == ModifiersBucket::All {
                    log::debug!("all next modifiers will be skipped because {:?} is found", ModifiersBucket::All);
                    break;
                }

            } else {
                log::debug!("no matches with {:#?}", text_rule);
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

        outcome = self.parse_text_excluding_modifiers(outcome.parsed_content(), Arc::clone(&parsing_configuration), excluded_modifiers)?;

        Ok(outcome)
    }


    fn count_newlines_at_start(s: &str) -> usize {
        s.bytes().take_while(|&b| b == b'\n').count()
    }

    fn count_newlines_at_end(s: &str) -> usize {
        s.bytes().rev().take_while(|&b| b == b'\n').count()
    }

    pub fn load_document_from_str(&self, document_name: &str, content: &str) -> Result<Document, DocumentError> {
        

        let mut preamble: String = String::new();
        
        let mut end_preamble: Option<usize> = Option::None;

        for (index, line) in content.lines().enumerate() {

            if !self.is_heading(line) {
                preamble.push_str(line);
            } else {
                end_preamble = Some(index);
                break;
            }
        }

        if end_preamble.is_none() {     // => there is no chapters

            let preamble = self.str_to_paragraphs(&preamble)?;

            let document = Document::new(String::from(document_name), preamble, vec![]);

            return Ok(document)
        }

        let end_preamble = end_preamble.unwrap();

        let mut document_chapters: Vec<Chapter> = Vec::new();

        let mut chapter_builder: Option<ChapterBuilder> = Option::None;

        let mut chapter: Option<Chapter> = Option::None;
        let mut text_of_chapter = String::new();

        for (_, line) in content.lines().enumerate().filter(|(index, _)| *index >= end_preamble) {
            
            // TODO!: for paragraph style this must change
            if self.is_heading(line) {
                
                if let Some(mut chapter) = chapter {

                    chapter.set_paragraphs(self.str_to_paragraphs(&text_of_chapter)?);

                    document_chapters.push(chapter);
                }

                let heading = self.load_heading_from_str(line).unwrap();

                chapter = Option::Some(Chapter::new(heading, vec![]));

            } else {

                text_of_chapter.push_str(line);
                text_of_chapter.push_str("\n");        // because .lines() remove \n
            }
        }

        if let Some(mut chapter) = chapter {
            chapter.set_paragraphs(self.str_to_paragraphs(&text_of_chapter)?);

            document_chapters.push(chapter);
        }

        let preamble = self.str_to_paragraphs(&preamble)?;
        Ok(Document::new(document_name.to_string(), preamble, document_chapters))

    }

    /// Split a string in the corresponding vector of paragraphs
    pub fn str_to_paragraphs(&self, content: &str) -> Result<Vec<Paragraph>, ParagraphError> {

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

                // TODO: remove count_newlines?
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
                TextModifier::Todo.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::Todo.search_pattern().clone(), TextModifier::Todo.incompatible_modifiers().clone(), String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))) as Box<dyn ParsingRule>,
            ),
            (
                TextModifier::AbridgedTodo.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::AbridgedTodo.search_pattern().clone(), TextModifier::AbridgedTodo.incompatible_modifiers().clone(), String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))),
            ),
            (
                TextModifier::BookmarkWithId.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::BookmarkWithId.search_pattern().clone(), TextModifier::BookmarkWithId.incompatible_modifiers().clone(), String::from(r#"<div class="bookmark" id="$2"><div class="bookmark-title">$1</div><div class="bookmark-description">$3</div></div>"#))),
            ),
            (
                TextModifier::Bookmark.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::Bookmark.search_pattern().clone(), TextModifier::Bookmark.incompatible_modifiers().clone(), String::from(r#"<div class="bookmark"><div class="bookmark-title">$1</div><div class="bookmark-description">$2</div></div>"#))),
            ),
            (
                TextModifier::AbridgedBookmarkWithId.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::AbridgedBookmarkWithId.search_pattern().clone(), TextModifier::AbridgedBookmarkWithId.incompatible_modifiers().clone(), String::from(r#"<div class="abridged-bookmark" id="$2"><div class="abridged-bookmark-title">$1</div></div>"#))),
            ),
            (
                TextModifier::AbridgedBookmark.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::AbridgedBookmark.search_pattern().clone(), TextModifier::AbridgedBookmark.incompatible_modifiers().clone(), String::from(r#"<div class="abridged-bookmark"><div class="abridged-bookmark-title">$1</div></div>"#))),
            ),
            (
                TextModifier::EmbeddedStyleWithId.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::EmbeddedStyleWithId.search_pattern().clone(), TextModifier::EmbeddedStyleWithId.incompatible_modifiers().clone(), String::from(r#"<span class="identifier embedded-style" id="$2" style="$3">$1</span>"#))),
            ),
            (
                TextModifier::EmbeddedStyle.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::EmbeddedStyle.search_pattern().clone(), TextModifier::EmbeddedStyle.incompatible_modifiers().clone(), String::from(r#"<span class="embedded-style" style="$2">$1</span>"#))),
            ),
            (
                TextModifier::AbridgedEmbeddedStyleWithId.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::AbridgedEmbeddedStyleWithId.search_pattern().clone(), TextModifier::AbridgedEmbeddedStyleWithId.incompatible_modifiers().clone(), String::from(r#"<span class="identifier abridged-embedded-style" id="$2" style="color: $3; background-color: $4; font-family: $5;">$1</span>"#))),
            ),
            (
                TextModifier::AbridgedEmbeddedStyle.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::AbridgedEmbeddedStyle.search_pattern().clone(), TextModifier::AbridgedEmbeddedStyle.incompatible_modifiers().clone(), String::from(r#"<span class="abridged-embedded-style" style="color: $2; background-color: $3; font-family: $4;">$1</span>"#))),
            ),
            (
                TextModifier::Identifier.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::Identifier.search_pattern().clone(), TextModifier::Identifier.incompatible_modifiers().clone(), String::from(r#"<span class="identifier" id="$2">$1</span>"#))),
            ),
            (
                TextModifier::Highlight.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::Highlight.search_pattern().clone(), TextModifier::Highlight.incompatible_modifiers().clone(), String::from(r#"<mark class="highlight">$1</mark>"#))),
            ),
            (
                TextModifier::InlineMath.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::InlineMath.search_pattern().clone(), TextModifier::InlineMath.incompatible_modifiers().clone(), String::from(r#"<span class="inline-math">$$${1}$$</span>"#))),
            ),
            (
                TextModifier::InlineCode.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::InlineCode.search_pattern().clone(), TextModifier::InlineCode.incompatible_modifiers().clone(), String::from(r#"<code class="language-markup inline-code">${1}</code>"#))),
            ),
            (
                TextModifier::BoldStarVersion.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::BoldStarVersion.search_pattern().clone(), TextModifier::BoldStarVersion.incompatible_modifiers().clone(), String::from(r#"<strong class="bold">${1}</strong>"#))),
            ),
            (
                TextModifier::BoldUnderscoreVersion.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::BoldUnderscoreVersion.search_pattern().clone(), TextModifier::BoldUnderscoreVersion.incompatible_modifiers().clone(), String::from(r#"<strong class="bold">${1}</strong>"#))),
            ),
            (
                TextModifier::ItalicStarVersion.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::ItalicStarVersion.search_pattern().clone(), TextModifier::ItalicStarVersion.incompatible_modifiers().clone(), String::from(r#"<em class="italic">${1}</em>"#))),
            ),
            (
                TextModifier::ItalicUnderscoreVersion.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::ItalicUnderscoreVersion.search_pattern().clone(), TextModifier::ItalicUnderscoreVersion.incompatible_modifiers().clone(), String::from(r#"<em class="italic">${1}</em>"#))),
            ),
            (
                TextModifier::Strikethrough.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::Strikethrough.search_pattern().clone(), TextModifier::Strikethrough.incompatible_modifiers().clone(), String::from(r#"<del class="strikethrough">${1}</del>"#))),
            ),
            (
                TextModifier::Underlined.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::Underlined.search_pattern().clone(), TextModifier::Underlined.incompatible_modifiers().clone(), String::from(r#"<u class="underlined">${1}</u>"#))),
            ),
            (
                TextModifier::Superscript.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::Superscript.search_pattern().clone(), TextModifier::Superscript.incompatible_modifiers().clone(), String::from(r#"<sup class="superscript">${1}</sup>"#))),
            ),
            (
                TextModifier::Subscript.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::Subscript.search_pattern().clone(), TextModifier::Subscript.incompatible_modifiers().clone(), String::from(r#"<sub class="subscript">${1}</sub>"#))),
            ),
            (
                TextModifier::Link.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::Link.search_pattern().clone(), TextModifier::Link.incompatible_modifiers().clone(), String::from(r#"<a href=\"$2\" class="link">${1}</a>"#))),
            ),
            (
                TextModifier::Comment.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::Comment.search_pattern().clone(), TextModifier::Comment.incompatible_modifiers().clone(), String::from(r#"<div class="checkbox checkbox-unchecked"></div>"#))),
            ),
            (
                TextModifier::Checkbox.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::Checkbox.search_pattern().clone(), TextModifier::Checkbox.incompatible_modifiers().clone(), String::from(r#"<div class="todo"><div class="todo-title"></div><div class="todo-description">$1</div></div>"#))) as Box<dyn ParsingRule>,
            ),
            (
                TextModifier::CheckboxChecked.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::CheckboxChecked.search_pattern().clone(), TextModifier::CheckboxChecked.incompatible_modifiers().clone(), String::from(r#"<div class="checkbox checkbox-checked"></div>"#))),
            ),
            (
                TextModifier::Emoji.identifier().clone(),
                Box::new(ReplacementRule::new(TextModifier::Emoji.search_pattern().clone(), TextModifier::Emoji.incompatible_modifiers().clone(), String::from(r#"<i class="em-svg em-${1}" aria-role="presentation"></i>"#))),
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
                Box::new(ReplacementRule::new(ParagraphModifier::PageBreak.search_pattern().clone(), ParagraphModifier::PageBreak.incompatible_modifiers().clone(), String::from(r#"<div class="page-break"></div>"#))) as Box<dyn ParsingRule>
            ),
            (
                ParagraphModifier::EmbeddedParagraphStyleWithId.identifier().clone(),
                Box::new(ReplacementRule::new(ParagraphModifier::EmbeddedParagraphStyleWithId.search_pattern().clone(), ParagraphModifier::EmbeddedParagraphStyleWithId.incompatible_modifiers().clone(), String::from(r#"<div class="identifier embedded-paragraph-style" id="$2" style="$3">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                ParagraphModifier::EmbeddedParagraphStyle.identifier().clone(),
                Box::new(ReplacementRule::new(ParagraphModifier::EmbeddedParagraphStyle.search_pattern().clone(),  ParagraphModifier::EmbeddedParagraphStyle.incompatible_modifiers().clone(), String::from(r#"<div class="embedded-paragraph-style" style="$2">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                ParagraphModifier::AbridgedEmbeddedParagraphStyleWithId.identifier().clone(),
                Box::new(ReplacementRule::new(ParagraphModifier::AbridgedEmbeddedParagraphStyleWithId.search_pattern().clone(),  ParagraphModifier::AbridgedEmbeddedParagraphStyleWithId.incompatible_modifiers().clone(), String::from(r#"<div class="identifier abridged-embedded-paragraph-style" id="$2" style="color: $3; background-color: $4; font-family: $5;">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                ParagraphModifier::AbridgedEmbeddedParagraphStyle.identifier().clone(),
                Box::new(ReplacementRule::new(ParagraphModifier::AbridgedEmbeddedParagraphStyle.search_pattern().clone(),  ParagraphModifier::AbridgedEmbeddedParagraphStyle.incompatible_modifiers().clone(), String::from(r#"<div class="abridged-embedded-paragraph-style" style="color: $2; background-color: $3; font-family: $4;">$1</div>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                ParagraphModifier::ParagraphIdentifier.identifier().clone(),
                Box::new(ReplacementRule::new(ParagraphModifier::ParagraphIdentifier.search_pattern().clone(),  ParagraphModifier::ParagraphIdentifier.incompatible_modifiers().clone(), String::from(r#"<span class="identifier" id="$2">$1</span>"#)).with_newline_fix(r"<br>".to_string())),
            ),
            (
                ParagraphModifier::ExtendedBlockQuote.identifier().clone(),
                Box::new(HtmlExtendedBlockQuoteRule::new()),
            ),
            (
                ParagraphModifier::MathBlock.identifier().clone(),
                Box::new(ReplacementRule::new(ParagraphModifier::MathBlock.search_pattern().clone(),  ParagraphModifier::MathBlock.incompatible_modifiers().clone(), String::from(r#"<p class="math-block">$$$$${1}$$$$</p>"#))),
            ),
            (
                ParagraphModifier::Image.identifier().clone(),
                Box::new(HtmlImageRule::new())
            ),
            (
                ParagraphModifier::CodeBlock.identifier().clone(),
                Box::new(ReplacementRule::new(ParagraphModifier::CodeBlock.search_pattern().clone(),  ParagraphModifier::CodeBlock.incompatible_modifiers().clone(), String::from(r#"<pre><code class="language-${1} code-block">$2</code></pre>"#))),
            ),
            (
                ParagraphModifier::List.identifier().clone(),
                Box::new(HtmlListRule::new()),
            ),
            (
                ParagraphModifier::FocusBlock.identifier().clone(),
                Box::new(ReplacementRule::new(ParagraphModifier::FocusBlock.search_pattern().clone(),  ParagraphModifier::FocusBlock.incompatible_modifiers().clone(), String::from(r#"<div class="focus-block focus-block-$1"><div class="focus-block-title focus-block-$1-title"></div><div class="focus-block-description focus-block-$1-description"">$2</div></div>"#)).with_newline_fix(r"<br>".to_string()))
            ),
            (
                ParagraphModifier::LineBreakDash.identifier().clone(),
                Box::new(ReplacementRule::new(ParagraphModifier::LineBreakDash.search_pattern().clone(),  ParagraphModifier::LineBreakDash.incompatible_modifiers().clone(), String::from(r#"<hr class="line-break line-break-dash">"#)))
            ),
            (
                ParagraphModifier::LineBreakStar.identifier().clone(),
                Box::new(ReplacementRule::new(ParagraphModifier::LineBreakStar.search_pattern().clone(),  ParagraphModifier::LineBreakStar.incompatible_modifiers().clone(), String::from(r#"<hr class="line-break line-break-star">"#)))
            ),
            (
                ParagraphModifier::LineBreakPlus.identifier().clone(),
                Box::new(ReplacementRule::new(ParagraphModifier::LineBreakPlus.search_pattern().clone(),  ParagraphModifier::LineBreakPlus.incompatible_modifiers().clone(), String::from(r#"<hr class="line-break line-break-plus">"#)))
            ),
            (
                ParagraphModifier::CommonParagraph.identifier().clone(),
                Box::new(ReplacementRule::new(ParagraphModifier::CommonParagraph.search_pattern().clone(),  ParagraphModifier::CommonParagraph.incompatible_modifiers().clone(), String::from(r#"<p class="paragraph">${1}</p>"#)))
            ),
        ]);

        let chapter_rules: HashMap<ModifierIdentifier, Box<dyn ParsingRule>> = HashMap::new();

        for i in (1..=MAX_HEADING_LEVEL).rev() {
            chapter_rules.insert(ChapterModifier::HeadingGeneralExtendedVersion(i).identifier().clone(), 
            Box::new(ReplacementRule::new(ChapterModifier::HeadingGeneralExtendedVersion(i).search_pattern().clone(), ChapterModifier::HeadingGeneralExtendedVersion(i).incompatible_modifiers().clone(), move |caps: &Captures| {
                let title = &caps[1];

                let id = Self::create_id(title);

                format!(r#"<h{} class="heading-{}" id="{}">{}</h{}>"#, i, i, id, title, i)
            })));

            chapter_rules.insert(ChapterModifier::HeadingGeneralCompactVersion(i).identifier().clone(), 
            Box::new(ReplacementRule::new(ChapterModifier::HeadingGeneralCompactVersion(i).search_pattern().clone(), ChapterModifier::HeadingGeneralCompactVersion(i).incompatible_modifiers().clone(), |caps: &Captures| {
                let heading_lv = &caps[1];
                let title = &caps[2];

                let id = Self::create_id(title);

                format!(r#"<h{} class="heading-{}" id="{}">{}</h>"#, heading_lv, heading_lv, id, title)
            })));

            // content_rules.push(Box::new(ReplacementRule::new(Modifier::HeadingGeneralExtendedVersion(i), format!(r#"<h{} class="heading-{}">$1</h{}>"#, i, i, i))));
            // content_rules.push(Box::new(ReplacementRule::new(Modifier::HeadingGeneralCompactVersion(i), String::from(r#"<h${1} class="heading-${1}">$2</h$>"#))));
        }

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

    use crate::compiler::parsable::ParsingConfiguration;

    use super::*;

    use parsing_rule::ParsingRule;

    use crate::compiler::parsable::codex::modifier::{text_modifier, Mod};

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

        let paragraphs = codex.str_to_paragraphs(nmd_text).unwrap();

        assert_eq!(paragraphs.len(), 2)
    }

    #[test]
    fn id() {
        let s = "my $string<-_778ks";

        let id = Codex::create_id(s);

        assert_eq!(id, "my-string-_778ks");
    }
}