use std::path::PathBuf;

pub struct GeneratorConfiguration {
    input_path: PathBuf,
    force_generation: bool,
    welcome: bool,
    gitkeep: bool
}


impl GeneratorConfiguration {
    pub fn new(input_path: PathBuf, force_generation: bool, welcome: bool, gitkeep: bool) -> Self {
        GeneratorConfiguration {
            input_path,
            force_generation,
            welcome,
            gitkeep
        }
    }

    pub fn input_path(&self) -> &PathBuf {
        &self.input_path
    }

    pub fn set_input_path(&mut self, new_input_path: PathBuf) {
        self.input_path = new_input_path;
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
}

impl Default for GeneratorConfiguration {
    fn default() -> Self {
        Self {
            input_path: Default::default(),
            force_generation: Default::default(),
            welcome: Default::default(),
            gitkeep: Default::default()
        }
    }
}