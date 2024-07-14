use std::{path::PathBuf, str::FromStr, fs::{self, OpenOptions}, io::Write};


use super::{ResourceError, Resource};



/// Resource which uses filesystem to store information 
#[derive(Debug, Clone)]
pub struct DiskResource {
    name: String, 
    location: PathBuf
}

impl DiskResource {
    pub fn new(location: PathBuf) -> Result<Self, ResourceError> {

        Self::try_from(location)
    }

    pub fn create_parents_dir(&self) -> Result<(), ResourceError> {

        let prefix = self.location.parent().unwrap();

        if !prefix.exists() {

            std::fs::create_dir_all(prefix)?;
        }

        Ok(())
    }
}

impl FromStr for DiskResource {
    type Err = ResourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.is_empty() {
            return Err(ResourceError::Creation("resource cannot be an empty string".to_string()));
        }

        Self::try_from(PathBuf::from_str(s).unwrap())
    }
}


impl ToString for DiskResource {
    fn to_string(&self) -> String {
        self.location().to_string_lossy().to_string()
    }
}

impl TryFrom<PathBuf> for DiskResource {
    type Error = ResourceError;

    fn try_from(location: PathBuf) -> Result<Self, Self::Error> {
        if location.is_dir() {
            return Err(ResourceError::InvalidResourceVerbose(format!("{} is a directory", location.to_string_lossy())))
        }

        if let Some(name) = location.file_name() {
            Ok(Self {
                name: name.to_string_lossy().to_string(),
                location
            })
        } else {
            Err(ResourceError::InvalidResource)
        }
    }
}

impl Resource for DiskResource {

    type LocationType = PathBuf;

    fn location(&self) -> &PathBuf {
        &self.location
    }

    fn name(&self) -> &String {
        &self.name        
    }

    fn write(&mut self, content: &str) -> Result<(), ResourceError> {

        let file_path = &self.location;

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_path)?;

        file.write_all(content.as_bytes())?;

        file.flush()?;
        file.sync_all()?;

        Ok(())
    }

    fn append(&mut self, content: &str) -> Result<(), ResourceError> {
        let file_path = &self.location;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        file.write_all(content.as_bytes())?;

        file.flush()?;
        file.sync_all()?;

        Ok(())
    }

    fn read(&self) -> Result<String, ResourceError> {
        match fs::read_to_string(self.location.clone()) {           // TODO: remove clone
            Ok(content) => Ok(content),
            Err(err) => Err(ResourceError::ReadError(format!("error during read content of {}: {}", self.to_string(), err.to_string())))
        }
    }

    fn erase(&mut self) -> Result<(), ResourceError> {
        if self.location.exists() {
            return Ok(fs::remove_file(self.location.clone())?)
        }

        Ok(())
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[should_panic]
    fn dir() {
        let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dossier_dir = "nmd-test-dossier-1";
        let nmd_file = project_directory.join("test-resources").join(dossier_dir);

        let _ = DiskResource::new(nmd_file).unwrap();
    }

    #[test]
    fn write() {
        let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dossier_dir = "nmd-test-dossier-1";
        let nmd_file = project_directory.join("test-resources").join(dossier_dir).join("document-to-write.nmd");

        let nmd_text = 
r#"
#1 title 1
## title 2
###### title 6
"#.trim();

        let mut resource = DiskResource::try_from(nmd_file).unwrap();

        resource.write(nmd_text).unwrap();

        assert_eq!(nmd_text, resource.content().unwrap());

        resource.erase().unwrap();
    }
}
