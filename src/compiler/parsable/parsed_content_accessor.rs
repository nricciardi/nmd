use crate::compiler::parsing::parsing_outcome::ParsingOutcome;



pub trait ParsedContentAccessor {
    fn parsed_content(&self) -> &Option<ParsingOutcome>;
}