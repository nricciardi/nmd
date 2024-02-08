use std::{fs::File, io::Read, path::PathBuf};

use base64::{engine::general_purpose::STANDARD, Engine};
use image::{codecs::jpeg, io::Reader, DynamicImage};

use super::ResourceError;
use jpeg::JpegEncoder;

const COMPRESSION_QUALITY: u8 = 75;


pub struct Image {
    image: DynamicImage
}

impl Image {
    pub fn to_base64(self) -> String {
        STANDARD.encode(self.image.into_bytes())
    }

    pub fn is_image(file_path: &PathBuf) -> bool {

        if let Ok(img) = Reader::open(file_path) {
            if let Ok(_) = img.with_guessed_format() {
                return true;
            }
        }

        false
    }

    pub fn compress(&mut self) -> Result<(), ResourceError> {
        
        let rgba_image = self.image.to_rgba8();
        let width = self.image.width();
        let height = self.image.height();

        let image_data = rgba_image.into_raw();
    
        let mut compressed_data: Vec<u8> = Vec::new();
        let mut jpeg_encoder = JpegEncoder::new_with_quality(&mut compressed_data, COMPRESSION_QUALITY);

        jpeg_encoder.encode(&image_data, width, height, image::ColorType::Rgba8)?;

        self.image = image::load_from_memory(&compressed_data)?;        

        Ok(())
    }
}

impl TryFrom<PathBuf> for Image {
    type Error = ResourceError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let image = image::open(path)?;

        Ok(Self { image })
    }
}

impl TryFrom<String> for Image {
    type Error = ResourceError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        let path = PathBuf::from(path);

        Self::try_from(path)
    }
}