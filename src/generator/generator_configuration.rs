use std::path::PathBuf;

pub struct GeneratorConfiguration {
    path: PathBuf,
    force_generation: bool,
    welcome: bool,
    gitkeep: bool,
    evaluate_existing_files: bool,
}


impl GeneratorConfiguration {
    pub fn new(path: PathBuf, force_generation: bool, welcome: bool, gitkeep: bool, evaluate_existing_files: bool) -> Self {
        Self {
            path,
            force_generation,
            welcome,
            gitkeep,
            evaluate_existing_files,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn set_path(&mut self, new_input_path: PathBuf) {
        self.path = new_input_path;
    }

    pub fn force_generation(&self) -> bool {
        self.force_generation
    }

    pub fn set_force_generation(&mut self, new_force: bool) {
        self.force_generation = new_force;
    }

    pub fn welcome(&self) -> bool {
        self.welcome
    }

    pub fn set_welcome(&mut self, new_welcome: bool) {
        self.welcome = new_welcome;
    }

    pub fn gitkeep(&self) -> bool {
        self.gitkeep
    }

    pub fn set_gitkeep(&mut self, gitkeep: bool) {
        self.gitkeep = gitkeep;
    }

    pub fn evaluate_existing_files(&self) -> bool {
        self.evaluate_existing_files
    }

    pub fn set_evaluate_existing_files(&mut self, evaluate_existing_files: bool) {
        self.evaluate_existing_files = evaluate_existing_files;
    }
}

impl Default for GeneratorConfiguration {
    fn default() -> Self {
        Self {
            path: Default::default(),
            force_generation: false,
            welcome: false,
            gitkeep: false,
            evaluate_existing_files: true
        }
    }
}