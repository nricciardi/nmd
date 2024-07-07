use std::str::FromStr;

use url::Url;

use super::{Resource, ResourceError};



/// Remote resource based on URL
pub struct RemoteResource {
    url: Url
}

impl RemoteResource {

    pub fn is_valid_remote_resource(s: &str) -> bool {
        Self::is_valid_url(s)
    }

    fn is_valid_url(s: &str) -> bool {
        match reqwest::Url::parse(s) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl FromStr for RemoteResource {
    type Err = ResourceError;

    fn from_str(url: &str) -> Result<Self, Self::Err> {
        if !Self::is_valid_url(url) {
            return Err(ResourceError::InvalidResourceVerbose(format!("{} is an invalid url", url)))
        }

        match Url::parse(url) {
            Ok(url) => Ok(Self {
                url
            }),
            Err(_) => Err(ResourceError::InvalidResourceVerbose(format!("{} is an invalid url", url)))
        }
        
    }
}

impl Resource for RemoteResource {
    type LocationType = Url;

    fn write(&mut self, _content: &str) -> Result<(), super::ResourceError> {
        todo!()
    }

    fn append(&mut self, _content: &str) -> Result<(), super::ResourceError> {
        todo!()
    }

    fn read(&self) -> Result<String, super::ResourceError> {
        todo!()
    }

    fn name(&self) -> &String {
        todo!()
    }

    fn location(&self) -> &Self::LocationType {
        &self.url
    }

    fn erase(&mut self) -> Result<(), ResourceError> {
        todo!()
    }
}