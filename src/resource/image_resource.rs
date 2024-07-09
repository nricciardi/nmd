use std::{fs::File, io::{Cursor, Read}, path::PathBuf, str::FromStr};

use base64::{engine::general_purpose::STANDARD, Engine};
use getset::{Getters, Setters};
use image::{error::UnsupportedError, DynamicImage, ImageOutputFormat};
use oxipng::Options;

use crate::compiler::dossier;

use super::ResourceError;
use image::io::Reader as ImageReader;


/// Image resource to manipulate images
#[derive(Debug, Getters, Setters)]
pub struct ImageResource {

    #[getset(get = "pub", set = "pub")]
    src: PathBuf,

    #[getset(get = "pub", set = "pub")]
    mime_type: Option<String>,

    #[getset(get = "pub", set = "pub")]
    caption: Option<String>,

    #[getset(get = "pub", set = "pub")]
    label: Option<String>,

    // #[getset(get = "pub", set = "pub")]
    // image: DynamicImage,
}

impl ImageResource {

    pub fn new(src: PathBuf, caption: Option<String>, label: Option<String>) -> Self {

        Self {
            src,
            mime_type: None,
            caption,
            label
        }
    }

    pub fn inferring_mime_type(mut self) -> Result<Self, ResourceError> {

        let mime_type = infer::get_from_path(&self.src)?;

        if let Some(t) = mime_type {

            let mut mime_type = t.mime_type().to_string();

            // work-around svg+xml
            if mime_type.contains("text/xml") {
                mime_type = String::from("image/svg+xml");
            }

            self.set_mime_type(Some(mime_type));

            return Ok(self);

        } else {
            return Err(ResourceError::InvalidResourceVerbose(format!("image {:?} mime type not found", self.src)));
        }
    }

    pub fn elaborating_relative_path(mut self, base_location: &PathBuf) -> Self {
        if self.src().is_relative() {

            self.set_src(base_location.join(self.src()));

            if !self.src().exists() {

                log::debug!("{:?} not found, try adding images directory path", self.src());

                let image_file_name = self.src().file_name().unwrap();

                self.set_src(base_location.join(dossier::ASSETS_DIR).join(dossier::IMAGES_DIR).join(image_file_name));
            }
        }

        self
    }

    // pub fn try_guess_format(&mut self) -> Result<(), ResourceError> {

    //     let image_res = ImageReader::open(self.src.clone());

    //     if let Ok(img) = image_res {
    //         if let Ok(img) = img.with_guessed_format() {

    //             self.mime_type = Some(img.format().unwrap().to_mime_type().to_string());

    //             return Ok(());
    //         }
    //     }

    //     Err(ResourceError::InvalidResourceVerbose(format!("unrecognized format image {:?}", self.src)))
    // }

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

        // let mut buffer: Vec<u8> = Vec::new();

        // self.image.write_to(&mut Cursor::new(&mut buffer), ImageOutputFormat::Png)?;

        // Ok(buffer)

        let mut image_file = File::open(self.src.clone())?;
        let mut raw_image: Vec<u8> = Vec::new();
        image_file.read_to_end(&mut raw_image)?;

        Ok(raw_image)
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

// impl TryFrom<PathBuf> for ImageResource {
//     type Error = ResourceError;

//     fn try_from(path: PathBuf) -> Result<Self, Self::Error> {

//         // TODO: open is a time consuming operation
//         // let image = ImageReader::open(path.clone())?.decode();

//         // if image.is_err() {

//         //     let e = image.err().unwrap();

//         //     log::error!("error occurs during image opening ({:?}): {}", path, e);
            
//         //     return Err(ResourceError::ImageError(e))
//         // }

//         // let image = image?;

//         Ok(Self {
//             format: path.extension().unwrap().to_str().unwrap().to_string(),
//             src: path,
//             built_src: None,
//             caption: None,
//             label: None
//             // image
//         })
//     }
// }

// impl TryFrom<String> for ImageResource {
//     type Error = ResourceError;

//     fn try_from(path: String) -> Result<Self, Self::Error> {
//         let path = PathBuf::from(path);

//         Self::try_from(path)
//     }
// }

impl FromStr for ImageResource {
    type Err = ResourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(PathBuf::from(s), None, None))
    }
}

#[cfg(test)]
mod test {
    // use std::{fs::File, io::{Cursor, Read}, path::PathBuf};

    // use image::ImageOutputFormat;

    // use crate::resource::image_resource::ImageResource;


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