use std::str::FromStr;

use regex::Regex;
use serde::de::value;
use thiserror::Error;

use crate::{compiler::dossier::document, resource::remote_resource::RemoteResource, utility::file_utility};

const VALUE_SEPARATOR: &str = "-";
const SPACE_REPLACER: char = '-';


#[derive(Error, Debug)]
pub enum ReferenceError {
    #[error("invalid URL reference")]
    InvalidUrlReference,

    #[error("invalid URL reference")]
    InvalidAssetReference,

    #[error("invalid URL reference")]
    InvalidInternalReference,
}

#[derive(Debug, Clone)]
enum ReferenceType {
    Url,
    Asset,
    Internal
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
#[derive(Debug, Clone)]
pub struct Reference {
    value: String,
    ref_type: ReferenceType
}

impl Reference {
    pub fn new(value: &str, ref_type: ReferenceType) -> Self {
        Self {
            value: String::from(value),
            ref_type
        }
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value
    }

    pub fn of_url(raw: &str) -> Result<Self, ReferenceError> {

        if RemoteResource::is_valid_remote_resource(raw) {
            return Err(ReferenceError::Invalid)
        }

        Ok(Self::new(raw, ReferenceType::Url))
    }

    pub fn of_asset(raw: &str) -> Result<Self, ReferenceError> {
        if file_utility::is_file_path(raw) {
            return Err(ReferenceError::Invalid)
        }

        Ok(Self::new(raw, ReferenceType::Asset))
    }

    pub fn of_internal(raw: &str, document_name_if_missed: Option<&str>) -> Result<Self, ReferenceError> {
        let regex = Regex::new(r"(.*)?#(.*)").unwrap();

        let caps = regex.captures(raw);

        if caps.is_none() {
            return Err(ReferenceError::Invalid)
        }

        let caps = caps.unwrap();

        let document_name = caps.get(1);
        let value = caps.get(2);

        if value.is_none() {
            return Err(ReferenceError::Invalid)
        }
        let value = value.unwrap().as_str();

        if let Some(document_name) = document_name {
            return Ok(Self::new(&format!("{}{}{}", document_name.as_str(), VALUE_SEPARATOR, value), ReferenceType::Asset))
        } else {
            return Ok(Self::new(&format!("{}{}{}", document_name_if_missed.unwrap(), VALUE_SEPARATOR, value), ReferenceType::Asset))
        }
    }

    /// Create new based on string. Argument can be in the following forms:
    /// 
    /// - document_name#id
    /// - #id
    /// - url
    /// - url#id
    /// - asset 
    pub fn of(raw: &str, document_name_if_missed: Option<&str>) -> Result<Self, ReferenceError> {

        if RemoteResource::is_valid_remote_resource(raw) {
            return Self::of_url(raw)
        }

        if raw.contains("#") {
            return Self::of_internal(raw, document_name_if_missed)

        } else {        // asset
            return Self::of_asset(raw)
        }
    }

    pub fn build(&self) -> String {

        match self.ref_type {
            ReferenceType::Url | ReferenceType::Asset => String::from(&self.value),
            ReferenceType::Internal => format!("{}", Self::parse_str(&self.value))
        }
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