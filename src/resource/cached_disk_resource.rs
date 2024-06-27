use std::{path::PathBuf, str::FromStr};

use super::{disk_resource::DiskResource, ResourceError, Resource};



/// Resource which uses filesystem to store information. It use simple cache logics avoid duplicating read and write operations
#[derive(Debug, Clone)]
pub struct CachedDiskResource {
    name: String, 
    location: PathBuf,
    shadow_resource: DiskResource,
    cached_content: Option<String>
}

impl FromStr for CachedDiskResource {
    type Err = ResourceError;

    fn from_str(path: &str) -> Result<Self, Self::Err> {

        if path.is_empty() {
            return Err(ResourceError::Creation("resource cannot be an empty string".to_string()));
        }

        Self::try_from(PathBuf::from_str(path).unwrap())
    }
}


impl ToString for CachedDiskResource {
    fn to_string(&self) -> String {
        self.location().to_string_lossy().to_string()
    }
}

impl TryFrom<PathBuf> for CachedDiskResource {
    type Error = ResourceError;

    fn try_from(location: PathBuf) -> Result<Self, Self::Error> {
        if location.is_dir() {
            return Err(ResourceError::InvalidResourceVerbose(format!("{} is a directory", location.to_string_lossy())))
        }

        if let Some(name) = location.file_name() {

            let l = location.clone();

            Ok(Self {
                name: name.to_string_lossy().to_string(),
                location: l.clone(),
                shadow_resource: DiskResource::try_from(l)?,
                cached_content: Option::None
            })
        } else {
            Err(ResourceError::InvalidResource)
        }
    }
}


#[allow(dead_code)]
impl CachedDiskResource {
    fn new(location: PathBuf) -> Result<Self, ResourceError> {

        Self::try_from(location)
    }

    pub fn cached_content(&self) -> &Option<String> {
        &self.cached_content
    }

    pub fn set_cached_content(&mut self, content: &str) -> () {
        self.cached_content = Some(content.to_string())
    }

    pub fn append_to_cached_content(&mut self, content: &str) -> () {
        if self.cached_content.is_some() {
            self.cached_content = Option::Some(self.cached_content.clone().unwrap() + content);
            
        } else {
            self.cached_content = Option::Some(content.to_string())
        }
    }

    pub fn dump_cached_content(&mut self) -> Result<(), ResourceError> {
        self.shadow_resource.write(self.cached_content.as_ref().unwrap().as_str())
    }

    pub fn clear_cached_content(&mut self) -> () {
        self.cached_content = Option::None
    }

    pub fn refresh_cached_content(&mut self) -> Result<(), ResourceError> {
        self.cached_content = Option::Some(self.shadow_resource.read()?);

        Ok(())   
    }
}

impl Resource for CachedDiskResource {
    type LocationType = PathBuf;

    fn write(&mut self, content: &str) -> Result<(), ResourceError> {

        self.set_cached_content(content);

        self.shadow_resource.write(self.cached_content.as_ref().unwrap().as_str())
    }

    fn append(&mut self, content: &str) -> Result<(), ResourceError> {

        self.append_to_cached_content(content);

        self.shadow_resource.append(self.cached_content.as_ref().unwrap().as_str())
    }

    fn read(&self) -> Result<String, ResourceError> {

        match &self.cached_content {
            Some(content) => Ok(content.clone()),
            None => self.shadow_resource.read()
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn location(&self) -> &Self::LocationType {
        &self.location
    }

    fn erase(&mut self) -> Result<(), ResourceError> {
        self.shadow_resource.erase()
    }
}