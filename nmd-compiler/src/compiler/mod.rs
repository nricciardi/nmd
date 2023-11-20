mod compiler_configuration;
mod supported_format;

pub use self::compiler_configuration::CompilerConfiguration;


pub struct Compiler {
    configuration: CompilerConfiguration
}

impl Compiler {

    pub fn new(configuration: CompilerConfiguration) -> Self {
        Compiler {
            configuration
        }
    }

    pub fn compile(&self) {
        println!("compile...")
    }
}