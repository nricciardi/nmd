use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use log;
use regex::{Captures, Regex, Replacer};

use crate::compiler::codex::Codex;
use crate::compiler::parsing::parsing_configuration::ParsingConfiguration;
use crate::compiler::parsing::parsing_error::ParsingError;
use crate::compiler::parsing::parsing_outcome::{ParsingOutcome, ParsingOutcomePart};
use crate::compiler::parsing::parsing_rule::constants::DOUBLE_NEW_LINE_REGEX;
use crate::resource::resource_reference::ResourceReference;

use super::ParsingRule;


#[derive(Debug, Clone)]
pub struct ReplacementRuleReplacerPart<R: Replacer> {
    replacer: R,
    fixed: bool,
    references_at: Vec<usize>
}

impl<R: Replacer> ReplacementRuleReplacerPart<R> {

    pub fn new_fixed(replacer: R) -> Self {
        Self {
            replacer,
            fixed: true,
            references_at: Vec::new()
        }
    }

    pub fn new_mutable(replacer: R) -> Self {
        Self {
            replacer,
            fixed: false,
            references_at: Vec::new()
        }
    }

    pub fn with_references_at(mut self, references_at: Vec<usize>) -> Self {
        self.references_at = references_at;

        self
    }

    pub fn replacer(&self) -> &R {
        &self.replacer
    }

    pub fn replacer_mut(&mut self) -> &mut R {
        &mut self.replacer
    }

    pub fn set_replacer(&mut self, r: R) {
        self.replacer = r
    }

    pub fn references_at(&self) -> &Vec<usize> {
        &self.references_at
    }
}



/// Rule to replace a NMD text based on a specific pattern matching rule
pub struct ReplacementRule<R: Replacer> {
    search_pattern: String,
    search_pattern_regex: Regex,
    replacer_parts: Vec<ReplacementRuleReplacerPart<R>>,
    newline_fix_pattern: Option<String>,
    reference_at: Option<usize>,
}

impl<R: Replacer> ReplacementRule<R> {
    
    /// Returns a new instance having a search pattern and a replication pattern
    pub fn new(searching_pattern: String, replacers: Vec<ReplacementRuleReplacerPart<R>>) -> Self {

        log::debug!("created new parsing rule with search_pattern: '{}'", searching_pattern);

        Self {
            search_pattern_regex: Regex::new(&searching_pattern).unwrap(),
            search_pattern: searching_pattern,
            replacer_parts: replacers,
            newline_fix_pattern: None,
            reference_at: None
        }
    }

    pub fn with_newline_fix(mut self, pattern: String) -> Self {
        self.newline_fix_pattern = Some(pattern);

        self
    }
}

impl Debug for ReplacementRule<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReplacementRule").field("searching_pattern", &self.search_pattern).field("replacer", &self.replacer_parts).field("newline_fix_pattern", &self.newline_fix_pattern).finish()
    }
}

impl ParsingRule for ReplacementRule<String> {

    /// Parse the content using internal search and replacement pattern
    fn standard_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {

        log::debug!("parsing:\n{}\nusing '{}'->'{:?}' (newline fix: {}, id_at: {:?})", content, self.search_pattern(), self.replacer_parts, self.newline_fix_pattern.is_some(), self.reference_at);

        let mut outcome = ParsingOutcome::new_empty();
        let mut last_match = 0;

        for captures in self.search_pattern_regex.captures_iter(content) {

            let mut replacers = self.replacer_parts.clone(); 

            // replace references
            for index in 0..self.replacer_parts.len() {

                for reference_at in self.replacer_parts[index].references_at() {

                    let reference = captures.get(reference_at.clone()).unwrap().as_str();

                    let reference = ResourceReference::of(reference, Some(parsing_configuration.read().unwrap().metadata().document_name().as_ref().unwrap()))?;
    
                    let reference = reference.build();

                    let r = replacers[index].replacer().replace(&format!("${}", reference_at), &reference);
                    replacers[index].set_replacer(r);

                    let r = replacers[index].replacer_mut().replace(&format!("${{{}}}", reference_at), &reference);
                    replacers[index].set_replacer(r);

                    log::debug!("id: '{}', new replacer: {:?}", reference, replacers[index]);
                }
            }

            let matched_content = captures.get(0).unwrap();

            if last_match < matched_content.start() {
                outcome.add_mutable_part(content[last_match..matched_content.start()].to_string());
            }

            last_match = matched_content.end();

            for replacer in replacers {
                let parsed_content = self.search_pattern_regex.replace_all(matched_content.as_str(), replacer.replacer());

                if replacer.fixed {

                    outcome.add_fixed_part(parsed_content.to_string());
    
                } else {
    
                    outcome.add_mutable_part(parsed_content.to_string());
                }
            }
            
        }

        if last_match < content.len() {
            outcome.add_mutable_part(content[last_match..content.len()].to_string());
        }

        if let Some(newline_fix_pattern) = self.newline_fix_pattern.as_ref() {

            for part in outcome.parts_mut().iter_mut() {
                let new_parsed_content = DOUBLE_NEW_LINE_REGEX.replace_all(&part.content(), newline_fix_pattern).to_string();
        
                match part {
                    ParsingOutcomePart::Fixed { content: _ } => *part = ParsingOutcomePart::Fixed { content: new_parsed_content },
                    ParsingOutcomePart::Mutable { content: _ } => *part = ParsingOutcomePart::Mutable { content: new_parsed_content },
                };
            }
        }

        log::debug!("result:\n{:?}", outcome);
        
        Ok(outcome)
    }
    
    fn search_pattern(&self) -> &String {
        &self.search_pattern
    }
    
    fn search_pattern_regex(&self) -> &Regex {
        &self.search_pattern_regex
    }
}


impl<F> Debug for ReplacementRule<F>
where F: 'static + Sync + Send + Fn(&Captures) -> String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReplacementRule").field("searching_pattern", &self.search_pattern).field("replacer", &"lambda function".to_string()).field("newline_fix_pattern", &self.newline_fix_pattern).finish()
    }
}

impl<F> ParsingRule for ReplacementRule<F>
where F: 'static + Sync + Send + Fn(&Captures) -> String {

    /// Parse the content using internal search and replacement pattern
    fn standard_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {

        log::debug!("parsing:\n{}\nusing '{}' (newline fix: {}, id_at: {:?})", content, self.search_pattern(), self.newline_fix_pattern.is_some(), self.reference_at);


        let mut outcome = ParsingOutcome::new_empty();

        for replacer in &self.replacer_parts {

            let parsed_content = self.search_pattern_regex.replace_all(content, replacer.replacer()).to_string();

            if replacer.fixed {

                outcome.add_fixed_part(parsed_content);

            } else {

                outcome.add_mutable_part(parsed_content);
            }
        }

        if let Some(newline_fix_pattern) = self.newline_fix_pattern.as_ref() {

            let last_index = outcome.parts().len() - 1;
            let last_element = outcome.parts().get(last_index).unwrap();

            let new_parsed_content = DOUBLE_NEW_LINE_REGEX.replace_all(&last_element.content(), newline_fix_pattern).to_string();
        
            match last_element {
                ParsingOutcomePart::Fixed { content: _ } => outcome.parts_mut().insert(last_index, ParsingOutcomePart::Fixed { content: new_parsed_content }),
                ParsingOutcomePart::Mutable { content: _ } => outcome.parts_mut().insert(last_index, ParsingOutcomePart::Mutable { content: new_parsed_content }),
            };
        }

        log::debug!("result:\n{:?}", outcome);
        
        Ok(outcome)
    }

    fn search_pattern(&self) -> &String {
        &self.search_pattern
    }

    fn search_pattern_regex(&self) -> &Regex {
        &self.search_pattern_regex
    }
}


#[cfg(test)]
mod test {

    use crate::compiler::codex::{codex_configuration::CodexConfiguration, modifier::{standard_chapter_modifier::StandardChapterModifier, standard_paragraph_modifier::StandardParagraphModifier, standard_text_modifier::StandardTextModifier, Modifier}};

    use super::*;

    #[test]
    fn bold_parsing() {

        let codex = Codex::of_html(CodexConfiguration::default());

        // valid pattern with a valid text modifier
        let parsing_rule = ReplacementRule::new(StandardTextModifier::BoldStarVersion.modifier_pattern().clone(), vec![
            ReplacementRuleReplacerPart::new_fixed(String::from("<strong>")),
            ReplacementRuleReplacerPart::new_mutable(String::from("$1")),
            ReplacementRuleReplacerPart::new_fixed(String::from("</strong>")),
        ]);

        let text_to_parse = r"A piece of **bold text** and **bold text2**";
        let parsing_configuration = Arc::new(RwLock::new(ParsingConfiguration::default()));

        let parsed_text = parsing_rule.parse(text_to_parse, &codex, Arc::clone(&parsing_configuration)).unwrap();

        assert_eq!(parsed_text.parsed_content(), r"A piece of <strong>bold text</strong> and <strong>bold text2</strong>");

        // without text modifier
        let text_to_parse = r"A piece of text without bold text";

        let parsed_text = parsing_rule.parse(text_to_parse, &codex, Arc::clone(&parsing_configuration)).unwrap();

        assert_eq!(parsed_text.parsed_content(), r"A piece of text without bold text");


    }

    #[test]
    fn heading_parsing() {

        let codex = Codex::of_html(CodexConfiguration::default());

        let parsing_configuration = Arc::new(RwLock::new(ParsingConfiguration::default()));

        let parsing_rule = ReplacementRule::new(StandardChapterModifier::HeadingGeneralExtendedVersion(6).modifier_pattern().clone(), vec![
            ReplacementRuleReplacerPart::new_fixed(String::from("<h6>")),
            ReplacementRuleReplacerPart::new_mutable(String::from("$1")),
            ReplacementRuleReplacerPart::new_fixed(String::from("</h6>")),
        ]);

        let text_to_parse = r"###### title 6";

        let parsed_text = parsing_rule.parse(text_to_parse, &codex, Arc::clone(&parsing_configuration)).unwrap();

        assert_eq!(parsed_text.parsed_content(), r"<h6>title 6</h6>");
    }

    #[test]
    fn paragraph_parsing() {

        let codex = Codex::of_html(CodexConfiguration::default());

        let parsing_configuration = Arc::new(RwLock::new(ParsingConfiguration::default()));

        let parsing_rule = ReplacementRule::new(StandardParagraphModifier::CommonParagraph.modifier_pattern_with_paragraph_separator().clone(), vec![
            ReplacementRuleReplacerPart::new_fixed(String::from("<p>")),
            ReplacementRuleReplacerPart::new_mutable(String::from("$1")),
            ReplacementRuleReplacerPart::new_fixed(String::from("</p>")),
        ]);

        let text_to_parse = concat!(  "\n\n",
                                            "p1\n\n\n",
                                            "p2\n\n\n",
                                            "p3a\np3b\np3c\n\n"
                                        );

        let parsed_text = parsing_rule.parse(text_to_parse, &codex, Arc::clone(&parsing_configuration)).unwrap();

        assert_eq!(parsed_text.parsed_content(), "<p>p1</p><p>p2</p><p>p3a\np3b\np3c</p>");
    }

    #[test]
    fn code_block() {

        let codex = Codex::of_html(CodexConfiguration::default());

        let parsing_configuration = Arc::new(RwLock::new(ParsingConfiguration::default()));

        let parsing_rule = ReplacementRule::new(StandardParagraphModifier::CodeBlock.modifier_pattern_with_paragraph_separator().clone(), vec![
            ReplacementRuleReplacerPart::new_fixed(String::from(r#"<pre><code class="language-$1 codeblock">"#)),
            ReplacementRuleReplacerPart::new_mutable(String::from("$2")),
            ReplacementRuleReplacerPart::new_fixed(String::from("</code></pre>")),
        ]);

        let text_to_parse = concat!(
            "\n\n",
            "```python\n\n",
            r#"print("hello world")"#,
            "\n\n```\n\n"
        );

        let parsed_text = parsing_rule.parse(text_to_parse, &codex, Arc::clone(&parsing_configuration)).unwrap();

        assert_eq!(parsed_text.parsed_content(), "<pre><code class=\"language-python codeblock\">print(\"hello world\")</code></pre>");
    }

    #[test]
    fn focus_block() {

        let codex = Codex::of_html(CodexConfiguration::default());
        
        let parsing_configuration = Arc::new(RwLock::new(ParsingConfiguration::default()));

        let parsing_rule = ReplacementRule::new(StandardParagraphModifier::FocusBlock.modifier_pattern_with_paragraph_separator().clone(), vec![
            ReplacementRuleReplacerPart::new_fixed(String::from(r#"<div class="focus-block focus-block-$1">$2</div>"#)),
            ReplacementRuleReplacerPart::new_mutable(String::from(r#"$2"#)),
            ReplacementRuleReplacerPart::new_fixed(String::from(r#"</div>"#)),
        ]).with_newline_fix(r"<br>".to_string());

        let text_to_parse = concat!(
            "# title 1",
            "::: warning\nnew warning\n\nmultiline\n:::",
            "\n",
        );

        let parsed_text = parsing_rule.parse(text_to_parse, &codex, Arc::clone(&parsing_configuration)).unwrap();
        let parsed_text = parsed_text.parsed_content();

        assert_ne!(parsed_text, text_to_parse);
     
    }
}