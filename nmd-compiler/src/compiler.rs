pub mod compilation_configuration;
mod parsable;
mod dossier;
pub mod output_format;
pub mod resource;
mod loadable;
mod assembler;
pub mod dumpable;
pub mod artifact;
mod utility;
pub mod theme;

use std::sync::Arc;

use thiserror::Error;
use crate::compiler::{dossier::Dossier, loadable::Loadable, dumpable::{Dumpable, DumpError}};

use self::{assembler::{assembler_configuration::AssemblerConfiguration, AssemblerError}, compilation_configuration::CompilationConfiguration, dossier::dossier_configuration, loadable::LoadError, parsable::{Parsable, ParsingError}};


#[derive(Error, Debug)]
pub enum CompilationError {
    /* #[error(transparent)]
    InvalidTarget(#[from] LocationError), */

    #[error("unknown error")]
    Unknown(String),

    #[error(transparent)]
    LoadError(#[from] LoadError),

    #[error(transparent)]
    ParsingError(#[from] ParsingError),

    #[error(transparent)]
    AssemblerError(#[from] AssemblerError),

    #[error(transparent)]
    DumpError(#[from] DumpError)
}

pub struct Compiler {
}

impl Compiler {

    pub fn compile(compilation_configuration: CompilationConfiguration) -> Result<(), CompilationError> {

        let codex = Arc::new(compilation_configuration.codex());

        let mut dossier = Dossier::load(Arc::clone(&codex), compilation_configuration.input_location())?;

        let dossier_configuration = dossier.configuration();

        let dossier_theme = dossier_configuration.style().theme().clone();
        
        dossier.parse(Arc::clone(&codex), Arc::new(compilation_configuration.parsing_configuration()))?;

        let assembler_configuration = AssemblerConfiguration::new(compilation_configuration.output_location().clone(), dossier_theme);

        let assembler = assembler::from(compilation_configuration.format().clone(), assembler_configuration);

        let mut artifact = assembler.assemble(*dossier)?;

        Ok(artifact.dump()?)
    }

    pub fn version() -> &'static str {
        "0.0.1"
    }
}

#[cfg(test)]
mod test {

    use std::path::PathBuf;

    use super::*;

    #[test]
    fn compile() {

        let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dossier_dir = "nmd-test-dossier-1";
        let nmd_dossier_path = project_directory.join("test-resources").join(dossier_dir);

        assert!(nmd_dossier_path.is_dir());

        let compilation_configuration = CompilationConfiguration::new(output_format::OutputFormat::Html, nmd_dossier_path.clone(), nmd_dossier_path.clone());

        Compiler::compile(compilation_configuration).unwrap()
    }

}