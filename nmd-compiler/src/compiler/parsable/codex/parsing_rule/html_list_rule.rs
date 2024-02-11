mod list;

use std::sync::Arc;

use crate::compiler::parsable::{codex::Modifier, ParsingConfiguration};

use super::{parsing_outcome::{ParsingError, ParsingOutcome}, ParsingRule};


pub struct HtmlListRule {

}

impl HtmlListRule {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

impl ParsingRule for HtmlListRule {
    fn modifier(&self) -> &Modifier {
        todo!()
    }

    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::list::List;

    #[test]
    fn parsing() {
        let nmd_text = r#"

- element 1
- element 2
    - element 2.1
        - element 2.1.1a
        element 2.1.1b
        - element 2.1.2
        - element 2.1.3
    - element 2.2
"#.trim();

        let root_list = List::default();

        


    }
}