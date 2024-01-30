mod compilation_configuration;
mod parsable;
mod dossier;
pub mod output_format;
pub mod resource;
mod loadable;
mod assembler;
pub mod dumpable;
pub mod artifact;
mod utility;

use std::sync::Arc;

use serde_json::error;
use thiserror::Error;
use crate::compiler::{artifact::Artifact, dossier::Dossier, loadable::Loadable, dumpable::{Dumpable, DumpError}};

use self::{compilation_configuration::CompilationConfiguration, loadable::LoadError, parsable::{Parsable, ParsingError}, assembler::{assembler_configuration::AssemblerConfiguration, AssemblerError}};


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

        let mut dossier = Dossier::load(compilation_configuration.input_location())?;
        
        dossier.parse(Arc::new(compilation_configuration.codex()), Arc::new(compilation_configuration.parsing_configuration()))?;

        let assembler = assembler::from(compilation_configuration.format().clone(), AssemblerConfiguration::new(compilation_configuration.output_location().clone()));

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