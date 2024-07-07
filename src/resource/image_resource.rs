use std::{fs::File, io::{Cursor, Read}, path::PathBuf, str::FromStr};

use base64::{engine::general_purpose::STANDARD, Engine};
use image::{codecs::jpeg, DynamicImage, ImageOutputFormat};
use oxipng::Options;

use super::ResourceError;
use image::io::Reader as ImageReader;


/// Image resource to manipulate images
pub struct ImageResource {
    src: PathBuf,
    image: DynamicImage,
}

impl ImageResource {

    /// Encode image in base64
    pub fn to_base64(&self, compression: bool) -> Result<String, ResourceError> {

        let buffer = self.to_vec_u8()?;

        if compression {

            let original_log_level = log::max_level();
            log::set_max_level(log::LevelFilter::Warn);

            let options = Options::max_compression();

            let optimized_png = oxipng::optimize_from_memory(&buffer, &options);

            log::set_max_level(original_log_level);
    
            match optimized_png {
                Ok(image) => return Ok(STANDARD.encode(image)),
                Err(err) => return Err(ResourceError::ElaborationError(format!("image compression error: {}", err)))
            }

        } else {

            Ok(STANDARD.encode(buffer))
        }
    }

    pub fn to_vec_u8(&self) -> Result<Vec<u8>, ResourceError> {

        let mut buffer: Vec<u8> = Vec::new();

        self.image.write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Png)?;

        Ok(buffer)
    }

    /// Check if a PathBuf is an image
    pub fn pathbuf_is_image(file_path: &PathBuf) -> bool {

        if let Ok(img) = ImageReader::open(file_path) {
            if let Ok(_) = img.with_guessed_format() {
                return true;
            }
        }

        false
    }
}

impl TryFrom<PathBuf> for ImageResource {
    type Error = ResourceError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {

        // TODO: open is a time consuming operation
        let image = ImageReader::open(path.clone())?.decode();

        if image.is_err() {

            let e = image.err().unwrap();

            log::error!("error occurs during image opening ({:?}): {}", path, e);
            
            return Err(ResourceError::ImageError(e))
        }

        let image = image?;

        Ok(Self {
            src: path,
            image
        })
    }
}

impl TryFrom<String> for ImageResource {
    type Error = ResourceError;

    fn try_from(path: String) -> Result<Self, Self::Error> {
        let path = PathBuf::from(path);

        Self::try_from(path)
    }
}

impl FromStr for ImageResource {
    type Err = ResourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(String::from(s))
    }
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::{Cursor, Read}, path::PathBuf};

    use image::ImageOutputFormat;

    use crate::resource::image_resource::ImageResource;


    #[test]
    fn vec_u8() {
        // let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // let image_path = project_directory.join("test-resources").join("wikipedia-logo.png");

        // let mut image_file = File::open(image_path.clone()).unwrap();
        // let mut raw_image: Vec<u8> = Vec::new();
        // image_file.read_to_end(&mut raw_image).unwrap();

        // let image = ImageResource::try_from(image_path).unwrap();

        // // let dynamic_image = image::load_from_memory(&buffer).unwrap();
        // // let mut buffer_from_dynamic_image: Vec<u8> = Vec::new();

        // // dynamic_image.write_to(&mut Cursor::new(&mut buffer_from_dynamic_image), ImageOutputFormat::Png).unwrap();

        // assert_eq!(raw_image.len(), image.to_vec_u8().unwrap().len());
    }

}