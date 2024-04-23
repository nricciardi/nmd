use std::str::FromStr;

use regex::Regex;
use thiserror::Error;

const PREFIX_VALUE_SEPARATOR: &str = "-";
const SPACE_REPLACER: char = '-';


#[derive(Error, Debug)]
pub enum ReferenceError {

}


/// # Reference
/// 
/// A reference to a resource. A resource can be an internal or external resource.
/// An internal resource is an heading, image or text with an ID of a dossier document.
/// An external resource is a URL or dossier (asset) file.
/// 
/// An internal resource is composed by "document name" (where there is the resource) and the resource ID.
/// 
/// An external resource is interpreted "as it is"
pub struct Reference {
    value: String,
}

impl Reference {
    pub fn new(value: &str) -> Self {
        Self {
            value: String::from(value)
        }
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value
    }

    fn of_internal_resource(s: &str, document_name_if_missed: Option<&str>) -> Result<Self, ReferenceError> {

        // TODO: is mandatory # ??

        let regex = Regex::new(r"(.*)?#(.*)").unwrap();

        let caps = regex.captures(s);

        if caps.is_none() {
            return Err(ReferenceError)
        }

        let caps = caps.unwrap();

        let prefix = caps.get(1).unwrap().as_str();
        let value = caps.get(2).unwrap().as_str();

        Ok(Self::new_with_prefix(prefix, value))
    }

    pub fn build(&self) -> String {

        if let Some(prefix) = self.prefix.as_ref() {
            return format!("{}{}{}", Self::parse_str(prefix), PREFIX_VALUE_SEPARATOR, Self::parse_str(&self.value))
        }

        format!("{}", Self::parse_str(&self.value))
    }

    fn parse_str(s: &str) -> String {

        let allowed_chars = s.chars().filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == ' ').map(|c| {
            if c == ' ' {
                return SPACE_REPLACER;
            }

            c
        });

        allowed_chars.collect()
    }
}