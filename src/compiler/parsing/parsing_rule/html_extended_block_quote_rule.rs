use std::sync::{Arc, RwLock};

use regex::Regex;

use crate::compiler::{codex::{modifier::{modifiers_bucket::ModifiersBucket, standard_paragraph_modifier::StandardParagraphModifier, Modifier}, Codex}, parsing::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome}};

use super::ParsingRule;


#[derive(Debug)]
pub struct HtmlExtendedBlockQuoteRule {
    searching_pattern: String
}

impl HtmlExtendedBlockQuoteRule {
    pub fn new() -> Self {
        Self {
            searching_pattern: StandardParagraphModifier::ExtendedBlockQuote.modifier_pattern_with_paragraph_separator()
        }
    }
}

impl ParsingRule for HtmlExtendedBlockQuoteRule {

    fn searching_pattern(&self) -> &String {
        &self.searching_pattern
    }
    fn standard_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {

        let content = content.trim();
        let mut lines: Vec<&str> = content.lines().collect();

        let check_extended_block_quote_regex = Regex::new(r"(?:^(?m:^> \[!(.*)\]))").unwrap();
        let there_is_quote_type = check_extended_block_quote_regex.is_match(content);
        let mut quote_type: String = String::from("quote");

        if there_is_quote_type {

            quote_type = check_extended_block_quote_regex.captures(content).unwrap().get(1).unwrap().as_str().to_string().to_lowercase();

            lines.remove(0);
        }

        let mut tag_body = String::new();

        for line in lines {
            if !line.starts_with(">") {
                if parsing_configuration.read().unwrap().strict_focus_block_check() {
                    log::warn!("invalid line in focus (quote) block: {}", line);
                    continue;
                } else {
                    log::error!("invalid line in focus (quote) block: {}", line);
                    panic!("invalid line in focus (quote) block");
                }
            }

            let mut c = line[1..].trim_start();

            if c.is_empty() {
                c = "\n\n";
            }

            tag_body.push_str(c);
        }

        let regex = Regex::new("\n\n").unwrap();
        tag_body = regex.replace_all(&tag_body, "<br>").to_string();

        let outcome = ParsingOutcome::new(format!(r#"
        <div class="focus-quote-block focus-quote-block-{}">
        <div class="focus-quote-block-title focus-quote-block-{}-title"></div>
        <div class="focus-quote-block-description focus-quote-block-{}-description">
            {}
        </div>
        </div>"#, quote_type, quote_type, quote_type, tag_body));

        Ok(outcome)
    }

}