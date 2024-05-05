pub mod compilation_configuration;
pub mod dossier;
pub mod output_format;
mod assembler;
pub mod dumpable;
pub mod artifact;
pub mod theme;
pub mod parser;
pub mod loader;
pub mod codex;
pub mod parsable;
pub mod parsing;

use std::{sync::{Arc, RwLock}, time::Instant};

use thiserror::Error;
use crate::compiler::{dossier::Dossier, dumpable::{DumpError, Dumpable}, loader::Loader, parsable::Parsable, parsing::parsing_metadata::ParsingMetadata};
use self::{assembler::{assembler_configuration::AssemblerConfiguration, AssemblerError}, compilation_configuration::CompilationConfiguration, dossier::dossier_configuration, loader::LoadError, parsing::parsing_error::ParsingError};


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

    pub fn compile_dossier(mut compilation_configuration: CompilationConfiguration) -> Result<(), CompilationError> {

        log::info!("start to compile dossier");

        let compile_start = Instant::now();

        log::info!("compilation configuration (this will override dossier compilation configuration):\n\n{:#?}\n", compilation_configuration);

        let codex = Arc::new(compilation_configuration.codex());

        let mut dossier = Loader::load_dossier_from_path_buf(&codex, compilation_configuration.input_location())?;

        log::info!("dossier loaded");


        let dossier_configuration = dossier.configuration();

        compilation_configuration.merge_dossier_configuration(dossier_configuration);

        let mut parsing_configuration = compilation_configuration.parsing_configuration();
        parsing_configuration.set_list_bullets_configuration(dossier_configuration.style().list_bullets_configuration().clone());
        parsing_configuration.set_strict_list_check(dossier_configuration.compilation().strict_list_check());

        log::info!("will use dossier configuration: {:?}", compilation_configuration.input_location());
        log::debug!("will use dossier configuration:\n\n{:#?}\n", dossier_configuration);

        let mut assembler_configuration = AssemblerConfiguration::from(dossier_configuration.clone());

        let dossier_theme = dossier_configuration.style().theme().clone();
        
        log::info!("parsing...");
        log::debug!("parsing configuration:\n{:#?}\n", parsing_configuration);

        dossier.parse(Arc::clone(&codex), Arc::new(RwLock::new(parsing_configuration)))?;

        assembler_configuration.set_output_location(compilation_configuration.output_location().clone());
        assembler_configuration.set_theme(dossier_theme);

        log::info!("assembling...");

        let assembly_time = Instant::now();

        let assembler = assembler::from(compilation_configuration.format().clone(), assembler_configuration);

        let mut artifact = assembler.assemble_dossier(/*&*codex,*/ &dossier)?;

        log::info!("end to assembly (assembly time {} ms)", assembly_time.elapsed().as_millis());

        artifact.dump()?;

        log::info!("end to compile dossier (compile time: {} ms)", compile_start.elapsed().as_millis());

        Ok(())
    }

    // pub fn version() -> &'static str {
        
    // }
}

#[cfg(test)]
mod test {

    use std::path::PathBuf;

    use super::*;

    #[test]
    fn compile_dossier() {

        let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dossier_dir = "nmd-test-dossier-1";
        let nmd_dossier_path = project_directory.join("test-resources").join(dossier_dir);

        assert!(nmd_dossier_path.is_dir());

        let compilation_configuration = CompilationConfiguration::new(output_format::OutputFormat::Html, nmd_dossier_path.clone(), nmd_dossier_path.clone());

        Compiler::compile_dossier(compilation_configuration).unwrap()
    }

}