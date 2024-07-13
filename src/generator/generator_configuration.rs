use std::path::PathBuf;

use getset::{CopyGetters, Getters, Setters};


#[derive(Debug, Clone, Getters, CopyGetters, Setters)]
pub struct GeneratorConfiguration {

    #[getset(get = "pub", set = "pub")]
    name: Option<String>,

    #[getset(get = "pub", set = "pub")]
    path: PathBuf,

    #[getset(get_copy = "pub", set = "pub")]
    force_generation: bool,

    #[getset(get_copy = "pub", set = "pub")]
    welcome: bool,

    #[getset(get_copy = "pub", set = "pub")]
    gitkeep: bool,

    #[getset(get_copy = "pub", set = "pub")]
    evaluate_existing_files: bool,
}


impl GeneratorConfiguration {
    pub fn new(name: Option<String>, path: PathBuf, force_generation: bool, welcome: bool, gitkeep: bool, evaluate_existing_files: bool) -> Self {
        Self {
            name,
            path,
            force_generation,
            welcome,
            gitkeep,
            evaluate_existing_files,
        }
    }

}

impl Default for GeneratorConfiguration {
    fn default() -> Self {
        Self {
            name: None,
            path: Default::default(),
            force_generation: false,
            welcome: false,
            gitkeep: false,
            evaluate_existing_files: true
        }
    }
}