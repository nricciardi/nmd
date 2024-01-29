use std::{fs::File, io::Read, path::PathBuf};

use base64::{engine::general_purpose::STANDARD, Engine};

use super::ResourceError;



pub struct Image {
    data: Vec<u8>
}

impl Image {
    pub fn to_base64(&self) -> String {
        STANDARD.encode(&self.data)
    }
}

impl TryFrom<PathBuf> for Image {
    type Error = ResourceError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Ok(Self { data: buffer })
    }
}

impl TryFrom<String> for Image {
    type Error = ResourceError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        let path = PathBuf::from(path);

        Self::try_from(path)
    }
}