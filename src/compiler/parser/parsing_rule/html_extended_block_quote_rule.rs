use std::sync::Arc;

use regex::Regex;

use crate::compiler::parsable::{codex::{modifier::{modifiers_bucket::ModifiersBucket, paragraph_modifier::ParagraphModifier, Mod}, Modifier}, ParsingConfiguration};

use super::{parsing_outcome::{ParsingError, ParsingOutcome}, ParsingRule};

pub struct HtmlExtendedBlockQuoteRule {

}

impl HtmlExtendedBlockQuoteRule {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

impl ParsingRule for HtmlExtendedBlockQuoteRule {

    fn search_pattern(&self) -> &String {
        &ParagraphModifier::ExtendedBlockQuote.search_pattern()
    }
    
    fn incompatible_modifiers(&self) -> &ModifiersBucket {
        &ParagraphModifier::ExtendedBlockQuote.incompatible_modifiers()
    }

    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {

        let mut lines: Vec<&str> = content.lines().collect();
        let check_extended_block_quote_regex = Regex::new(r"(?:^(?m:^> \[!(.*)\]))").unwrap();
        let not_quote_type = check_extended_block_quote_regex.is_match(content);
        let mut quote_type: String = String::from("quote");

        if not_quote_type {

            quote_type = check_extended_block_quote_regex.captures(content).unwrap().get(1).unwrap().as_str().to_string().to_lowercase();

            lines.remove(0);
        }

        let mut tag_body = String::new();

        for line in lines {
            if !line.starts_with(">") {
                if parsing_configuration.strict_focus_block_check() {
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