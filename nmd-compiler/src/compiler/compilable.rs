use super::parsable::Parsable;
use super::location::Locatable;


pub trait Compilable: Locatable + Parsable {}