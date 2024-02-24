pub mod compiler;
pub mod resource;
pub mod generator;
mod utility;

use std::{path::PathBuf, str::FromStr};

use clap::{Arg, ArgAction, Command};
use compiler::{output_format::OutputFormatError, CompilationError};
pub use compiler::Compiler;
use generator::{generator_configuration::GeneratorConfiguration, Generator};
use log::{LevelFilter, ParseLevelError};
use resource::ResourceError;
use thiserror::Error;
use simple_logger::SimpleLogger;

use crate::compiler::{compilation_configuration::CompilationConfiguration, output_format::OutputFormat};

pub const VERSION: &str = "0.9.3-alpha";

#[derive(Error, Debug)]
pub enum NmdCliError {
    #[error(transparent)]
    CompilationError(#[from] CompilationError),

    #[error("you must provide only one value of '{0}'")]
    MoreThanOneValue(String),

    #[error(transparent)]
    OutputFormatError(#[from] OutputFormatError),

    #[error(transparent)]
    VerboseLevelError(#[from] ParseLevelError),

    #[error(transparent)]
    ResourceError(#[from] ResourceError),
}


pub struct NmdCli {
    cli: Command
}

impl NmdCli {

    pub fn new() -> Self {

        let cli: Command = Command::new("nmd")
                .about("Official compiler to parse NMD")
                .version(VERSION)
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("compile")
                                .about("Compile an NMD resource")
                                .short_flag('c')
                                .subcommand_required(true)
                                .subcommand(
                                    Command::new("dossier")
                                        .about("Compile NMD dossier")
                                        .short_flag('d')
                                        .arg(
                                            Arg::new("format")
                                            .short('f')
                                            .long("format")
                                            .help("output format")
                                            .action(ArgAction::Set)
                                            .num_args(1)
                                            .default_value("html")
                                        )
                                        .arg(
                                            Arg::new("input-path")
                                            .short('i')
                                            .long("input-path")
                                            .help("input path")
                                            .action(ArgAction::Set)
                                            .num_args(1)
                                            .default_value(".")

                                        )
                                        .arg(
                                            Arg::new("output-path")
                                            .short('o')
                                            .long("output-path")
                                            .help("output path")
                                            .action(ArgAction::Set)
                                            .num_args(1)
                                            .default_value(".")
                                        )
                                        .arg(
                                            Arg::new("verbose")
                                                .short('v')
                                                .long("verbose")
                                                .action(ArgAction::Set)
                                                .default_value("info")
                                        )

                                )
                                
                )
                .subcommand(
                    Command::new("generate")
                        .about("Generate a new NMD resource")
                        .short_flag('g')
                        .subcommand_required(true)
                                .subcommand(
                                    Command::new("dossier")
                                        .about("Generate a new NMD dossier")
                                        .short_flag('d')
                                        .arg(
                                            Arg::new("path")
                                            .short('p')
                                            .long("path")
                                            .help("destination path")
                                            .action(ArgAction::Set)
                                            .num_args(1)
                                            .required(true)

                                        )
                                        .arg(
                                            Arg::new("force")
                                            .short('f')
                                            .long("force")
                                            .help("force generation")
                                            .action(ArgAction::SetTrue)

                                        )
                                        .arg(
                                            Arg::new("gitkeep")
                                            .short('k')
                                            .long("gitkeep")
                                            .help("add .gitkeep file")
                                            .action(ArgAction::SetTrue)

                                        )
                                        .arg(
                                            Arg::new("welcome")
                                            .short('w')
                                            .long("welcome")
                                            .help("add welcome page")
                                            .action(ArgAction::SetTrue)

                                        )
                                        .arg(
                                            Arg::new("verbose")
                                                .short('v')
                                                .long("verbose")
                                                .action(ArgAction::Set)
                                                .default_value("info")
                                        )

                                )
                );

        Self {
            cli
        }
    }

    pub fn parse(self) -> Result<(), NmdCliError> {

        let matches = self.cli.get_matches();

        match matches.subcommand() {
            Some(("compile", compile_matches)) => {
                match compile_matches.subcommand() {
                    Some(("dossier", compile_dossier_matches)) => {

                        if let Some(mut verbose) = compile_dossier_matches.get_many::<String>("verbose") {
                            
                            if verbose.len() != 1 {
                                return Err(NmdCliError::MoreThanOneValue("verbose".to_string()));
                            }
                            
                            
                            let log_level = LevelFilter::from_str(verbose.nth(0).unwrap())?;

                            Self::set_logger(log_level);
                        }

                        let mut compilation_configuration = CompilationConfiguration::default();
                        
                        if let Some(mut format) = compile_dossier_matches.get_many::<String>("format") {
                            
                            if format.len() != 1 {
                                return Err(NmdCliError::MoreThanOneValue("format".to_string()));
                            }
                            
                            
                            let format = OutputFormat::from_str(format.nth(0).unwrap())?;

                            compilation_configuration.set_format(format);
                        }

                        if let Some(mut input_path) = compile_dossier_matches.get_many::<String>("input-path") {
                            
                            if input_path.len() != 1 {
                                return Err(NmdCliError::MoreThanOneValue("input-path".to_string()));
                            }
                            
                            
                            let input_path = PathBuf::from(input_path.nth(0).unwrap());

                            compilation_configuration.set_input_location(input_path);
                        }

                        if let Some(mut output_path) = compile_dossier_matches.get_many::<String>("output-path") {
                            
                            if output_path.len() != 1 {
                                return Err(NmdCliError::MoreThanOneValue("output-path".to_string()));
                            }
                            
                            
                            let output_path = PathBuf::from(output_path.nth(0).unwrap());

                            compilation_configuration.set_output_location(output_path);
                        }


                        Ok(Compiler::compile_dossier(compilation_configuration)?)

                    },

                    _ => unreachable!()
                }
            },

            Some(("generate", generate_matches)) => {

                match generate_matches.subcommand() {
                    Some(("dossier", generate_dossier_matches)) => {

                        if let Some(mut format) = generate_dossier_matches.get_many::<String>("verbose") {
                            
                            if format.len() != 1 {
                                return Err(NmdCliError::MoreThanOneValue("verbose".to_string()));
                            }
                            
                            
                            let log_level = LevelFilter::from_str(format.nth(0).unwrap())?;
        
                            Self::set_logger(log_level);
                        }

                        let mut generator_configuration = GeneratorConfiguration::default();

                        if let Some(mut input_path) = generate_dossier_matches.get_many::<String>("path") {
                            
                            if input_path.len() != 1 {
                                return Err(NmdCliError::MoreThanOneValue("path".to_string()));
                            }
                            
                            
                            let input_path = PathBuf::from(input_path.nth(0).unwrap());

                            generator_configuration.set_path(input_path);
                        }

                        generator_configuration.set_force_generation(generate_dossier_matches.get_flag("force"));
                        generator_configuration.set_gitkeep(generate_dossier_matches.get_flag("gitkeep"));
                        generator_configuration.set_welcome(generate_dossier_matches.get_flag("welcome"));
                        
                        Ok(Generator::generate_dossier(generator_configuration)?)
                    },
                    _ => unreachable!()
                }
            },

            _ => unreachable!()
        }
    }

    fn set_logger(log_level: LevelFilter) {

        SimpleLogger::new()
            .with_level(log_level)
            .init()
            .unwrap();
    }

}