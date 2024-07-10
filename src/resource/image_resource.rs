use std::{fs::{self, File}, io::Read, path::PathBuf, str::FromStr};

use base64::{engine::general_purpose::STANDARD, Engine};
use getset::{Getters, Setters};
use oxipng::Options;

use crate::compiler::dossier;

use super::ResourceError;


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

    /// Infer mime type from image path using `infer` lib.
    /// `text/xml` is replaced by `image/svg+xml`
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

    /// Elaborate `src` path if it is relative appending if it doesn't exist dossier `assets` directory
    pub fn elaborating_relative_path_as_dossier_assets(mut self, base_location: &PathBuf) -> Self {
        if self.src().is_relative() {

            self.set_src(base_location.join(self.src()));

            if !self.src().exists() {

                log::debug!("{:?} not found, try adding images directory path", self.src());

                let image_file_name = self.src().file_name().unwrap();

                self.set_src(base_location.join(dossier::ASSETS_DIR).join(dossier::IMAGES_DIR).join(image_file_name));

                if !self.src().exists() {
                    if let Ok(src) = fs::canonicalize(self.src().clone()) {
                        self.set_src(src);
                    }
                }
            }
        }

        self
    }

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

        let mut image_file = File::open(self.src.clone())?;
        let mut raw_image: Vec<u8> = Vec::new();
        image_file.read_to_end(&mut raw_image)?;

        Ok(raw_image)
    }
}

impl FromStr for ImageResource {
    type Err = ResourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(PathBuf::from(s), None, None).inferring_mime_type()?)
    }
}

#[cfg(test)]
mod test {

}