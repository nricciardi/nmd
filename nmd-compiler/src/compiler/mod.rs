mod compiler_configuration;
mod supported_format;

pub use self::compiler_configuration::CompilerConfiguration;


pub struct Compiler {
    version: &'static str,
    configuration: CompilerConfiguration
}

impl Compiler {

    pub fn new(configuration: CompilerConfiguration) -> Self {
        Compiler {
            version: "0.0.1",
            configuration
        }
    }

    pub fn compile(&self) {
        todo!("compile...")
    }
}