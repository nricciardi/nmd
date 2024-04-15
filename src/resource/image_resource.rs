use std::{fs::File, io::{Cursor, Read}, path::PathBuf};

use base64::{engine::general_purpose::STANDARD, Engine};
use image::{codecs::jpeg, DynamicImage, ImageOutputFormat};

use super::ResourceError;
use jpeg::JpegEncoder;



use image::io::Reader as ImageReader;

const COMPRESSION_QUALITY: u8 = 75;


pub struct ImageResource {
    image: DynamicImage
}

impl ImageResource {
    pub fn to_base64(self) -> Result<String, ResourceError> {

        let mut buffer: Vec<u8> = Vec::new();

        self.image.write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Png)?;

        Ok(STANDARD.encode(buffer))
    }

    pub fn is_image(file_path: &PathBuf) -> bool {

        if let Ok(img) = ImageReader::open(file_path) {
            if let Ok(_) = img.with_guessed_format() {
                return true;
            }
        }

        false
    }

    pub fn compress(&mut self) -> Result<(), ResourceError> {
        
        todo!();
        
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

impl TryFrom<PathBuf> for ImageResource {
    type Error = ResourceError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {

        let image = ImageReader::open(path).unwrap().decode();

        if image.is_err() {
            panic!("{:#?}", image)
        }

        let image = image?;

        Ok(Self { image })
    }
}

impl TryFrom<String> for ImageResource {
    type Error = ResourceError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        let path = PathBuf::from(path);

        Self::try_from(path)
    }
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::{Cursor, Read}, path::PathBuf};

    use image::ImageOutputFormat;


    #[test]
    fn vec_u8() {
        let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let image_path = project_directory.join("test-resources").join("wikipedia-logo.png");

        let mut image_vec_u8 = File::open(image_path.clone()).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        image_vec_u8.read_to_end(&mut buffer).unwrap();

        let dynamic_image = image::load_from_memory(&buffer).unwrap();
        let mut buffer_from_dynamic_image: Vec<u8> = Vec::new();

        dynamic_image.write_to(&mut Cursor::new(&mut buffer_from_dynamic_image), ImageOutputFormat::Png).unwrap();

        assert_eq!(buffer.len(), buffer_from_dynamic_image.len());
        assert_eq!(buffer, buffer_from_dynamic_image)
    }

}