use std::{path::{PathBuf, MAIN_SEPARATOR_STR}, sync::Mutex};

use crate::resource::remote_resource::RemoteResource;

use super::dossier_configuration_path_reference::{DossierConfigurationPathReference, DossierConfigurationRawPathReference};


pub static DOSSIER_CONFIGURATION_RAW_REFERENCE_MANAGER: Mutex<DossierConfigurationRawReferenceManager> = Mutex::new(DossierConfigurationRawReferenceManager::new());



pub const PATH_START_FOR_RELATIVE: &str = r"./";

#[derive(Debug, Clone)]
pub struct DossierConfigurationRawReferenceManager {
    root_path: Option<PathBuf>,
    not_unix_like_os: bool
}

impl DossierConfigurationRawReferenceManager {

    pub const fn new() -> Self {
        Self {
            root_path: None,
            not_unix_like_os: false
        }
    }

    pub fn using_root_path(root_path: PathBuf) -> Self {
        Self {
            root_path: Some(root_path),
            ..Self::new()
        }
    }

    pub fn set_root_path(&mut self, root_path: PathBuf) {

        self.not_unix_like_os = false;

        if MAIN_SEPARATOR_STR.eq(r"/") {     // => unix-like
            log::info!("UNIX-like OS inferred: no logical separator replacement");

        } else {     // => windows
            log::info!("Windows OS inferred: it will be applied a logical separator replacement (from {} to /)", MAIN_SEPARATOR_STR);
            self.not_unix_like_os = true;
        }

        self.root_path = Some(root_path)
    }

    pub fn parse_raw_reference(&self, raw_reference: &DossierConfigurationRawPathReference, plain_root_path: Option<PathBuf>) -> DossierConfigurationPathReference {
        
        let mut raw_reference = raw_reference.clone();

        // fix not unix relative start (see PATH_START_FOR_RELATIVE)
        if self.not_unix_like_os {
            raw_reference = raw_reference.replace(format!(".{}", MAIN_SEPARATOR_STR).as_str(), r"./");
        }

        if raw_reference.starts_with(PATH_START_FOR_RELATIVE) {

            log::debug!("parsing raw path reference... apply root path to '{}'", raw_reference);

            // remove relative start
            for _ in 0..PATH_START_FOR_RELATIVE.len() {
                raw_reference.remove(0);
            }

            if let Some(ref root_path) = self.root_path {
                return DossierConfigurationPathReference::from(root_path.join(&raw_reference).to_string_lossy());
            }
            
            return DossierConfigurationPathReference::from(&raw_reference);

        } else if !raw_reference.is_empty() && raw_reference.chars().nth(0).unwrap().is_alphanumeric() && !RemoteResource::is_valid_remote_resource(&raw_reference) {

            // it's plain
            log::debug!("parsing raw path reference... it is plain reference, plain root path: {:#?}", plain_root_path);

            let plain_root_path = plain_root_path.unwrap();

            return self.parse_raw_reference(&plain_root_path.join(raw_reference).to_string_lossy().to_string(), None);
        }


        log::debug!("parsing raw path reference... '{}' skipped from apply any logic", raw_reference);
        
        return DossierConfigurationPathReference::from(&raw_reference);
        

    }
}