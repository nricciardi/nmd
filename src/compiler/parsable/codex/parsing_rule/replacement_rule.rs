use std::fmt::{Display, Debug};
use std::sync::Arc;

use log;
use regex::{Captures, Regex, Replacer};

use crate::compiler::parsable::ParsingConfiguration;

use super::parsing_outcome::{ParsingError, ParsingOutcome};
use super::{Modifier, ParsingRule};


/// Rule to replace a NMD text based on a specific pattern matching rule
pub struct ReplacementRule<R: Replacer> {
    modifier: Modifier,
    replacer: R,
    newline_fix: bool,
    newline_fix_pattern: Option<String>
}

impl<R: Replacer> ReplacementRule<R> {
    /// Returns a new instance having a search pattern and a replication pattern
    ///
    /// # Arguments
    ///
    /// * `pattern_type` - PatternType which represent the pattern used to search in text 
    /// * `replacement_pattern` - A string slice which represent the pattern used to replace the text
    ///
    pub fn new(modifier: Modifier, replacer: R) -> Self {

        log::debug!("created new parsing rule with search_pattern: '{}'", modifier.search_pattern()); //  and replacer: '{:?}'

        Self {
            modifier,
            replacer,
            newline_fix: false,
            newline_fix_pattern: None
        }
    }

    pub fn with_newline_fix(mut self, pattern: String) -> Self {
        self.newline_fix = true;
        self.newline_fix_pattern = Some(pattern);

        self
    }
}

impl ParsingRule for ReplacementRule<String> {

    /// Parse the content using internal search and replacement pattern
    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {

        let regex = match Regex::new(&self.modifier().search_pattern()) {
          Ok(r) => r,
          Err(_) => return Err(ParsingError::InvalidPattern(self.modifier().search_pattern()))  
        };

        log::debug!("parsing:\n{}\nusing '{}'->'{}' (newline fix: {})", content, self.modifier().search_pattern(), self.replacer, self.newline_fix);

        let mut parsed_content = regex.replace_all(content, self.replacer.as_str()).to_string();

        if self.newline_fix {
            let regex = Regex::new("\n\n").unwrap();
            parsed_content = regex.replace_all(&parsed_content, self.newline_fix_pattern.clone().unwrap().as_str()).to_string();
        }

        log::debug!("result:\n{}", parsed_content);
        
        Ok(ParsingOutcome::new(parsed_content))
    }

    fn modifier(&self) -> &Modifier {
        &self.modifier
    }
}

impl<F> ParsingRule for ReplacementRule<F>
where F: 'static + Sync + Send + Fn(&Captures) -> String {

    /// Parse the content using internal search and replacement pattern
    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {

        let regex = match Regex::new(&self.modifier().search_pattern()) {
          Ok(r) => r,
          Err(_) => return Err(ParsingError::InvalidPattern(self.modifier().search_pattern()))  
        };

        // log::debug!("parsing:\n{}\nusing '{}'->'{}' (newline fix: {})", content, self.modifier().search_pattern(), self.replacer, self.newline_fix);

        let mut parsed_content = regex.replace_all(content, &self.replacer).to_string();

        if self.newline_fix {
            let regex = Regex::new("\n\n").unwrap();
            parsed_content = regex.replace_all(&parsed_content, self.newline_fix_pattern.clone().unwrap().as_str()).to_string();
        }

        log::debug!("result:\n{}", parsed_content);
        
        Ok(ParsingOutcome::new(parsed_content))
    }

    fn modifier(&self) -> &Modifier {
        &self.modifier
    }
}


#[cfg(test)]
mod test {

    use crate::compiler::parsable::ParsingConfiguration;

    use super::*;

    #[test]
    fn bold_parsing() {
        // valid pattern with a valid text modifier
        let parsing_rule = ReplacementRule::new(Modifier::BoldStarVersion, String::from("<strong>$1</strong>"));

        let text_to_parse = r"A piece of **bold text**";
        let parsing_configuration = Arc::new(ParsingConfiguration::default());

        let parsed_text = parsing_rule.parse(text_to_parse, Arc::clone(&parsing_configuration)).unwrap();

        assert_eq!(parsed_text.parsed_content(), r"A piece of <strong>bold text</strong>");

        // without text modifier
        let text_to_parse = r"A piece of text without bold text";

        let parsed_text = parsing_rule.parse(text_to_parse, Arc::clone(&parsing_configuration)).unwrap();

        assert_eq!(parsed_text.parsed_content(), r"A piece of text without bold text");


    }

    #[test]
    fn heading_parsing() {
        let parsing_configuration = Arc::new(ParsingConfiguration::default());

        let parsing_rule = ReplacementRule::new(Modifier::HeadingGeneralExtendedVersion(6), String::from("<h6>$1</h6>"));

        let text_to_parse = r"###### title 6";

        let parsed_text = parsing_rule.parse(text_to_parse, Arc::clone(&parsing_configuration)).unwrap();

        assert_eq!(parsed_text.parsed_content(), r"<h6>title 6</h6>");
    }

    #[test]
    fn paragraph_parsing() {
        let parsing_configuration = Arc::new(ParsingConfiguration::default());

        let parsing_rule = ReplacementRule::new(Modifier::CommonParagraph, String::from("<p>$1</p>"));

        let text_to_parse = r#"
paragraph 2a.

paragraph 2b.

paragraph
2c
.

"#.trim_start();

        let parsed_text = parsing_rule.parse(text_to_parse, Arc::clone(&parsing_configuration)).unwrap();

        assert_eq!(parsed_text.parsed_content(), r"<p>paragraph 2a.</p><p>paragraph 2b.</p><p>paragraph\n2c\n.</p>");
    }

    #[test]
    fn code_block() {
        let parsing_configuration = Arc::new(ParsingConfiguration::default());

        let parsing_rule = ReplacementRule::new(Modifier::CodeBlock, String::from(r#"<pre><code class="language-$1 codeblock">$2</code></pre>"#));

        let text_to_parse = r#"
```python

print("hello world")

```
"#;

        let parsed_text = parsing_rule.parse(text_to_parse, Arc::clone(&parsing_configuration)).unwrap();

        assert_eq!(parsed_text.parsed_content(), "\n<pre><code class=\"language-python codeblock\">print(\"hello world\")</code></pre>\n");
    }

    #[test]
    fn focus_block() {
        let parsing_configuration = Arc::new(ParsingConfiguration::default());

        let parsing_rule = ReplacementRule::new(Modifier::FocusBlock, String::from(r#"<div class="focus-block focus-block-$1">$2</div>"#)).with_newline_fix(r"<br>".to_string());

        let text_to_parse = r#"
# title 1

::: warning
new
warning

multiline
:::


"#;

        let parsed_text = parsing_rule.parse(text_to_parse, Arc::clone(&parsing_configuration)).unwrap();
        let parsed_text = parsed_text.parsed_content();

        assert_ne!(parsed_text, text_to_parse);
     
    }
}