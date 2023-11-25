use super::parsable::Parsable;
use super::location::Locatable;
use anyhow::Result;


pub trait Compilable: Locatable + Parsable {
    fn compile(&self) -> Result<()>;
}