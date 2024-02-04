pub mod compiler;

use std::{path::PathBuf, str::FromStr};

use clap::{error, Arg, ArgAction, Command};
use compiler::{output_format::OutputFormatError, CompilationError};
pub use compiler::Compiler;
use log::LevelFilter;
use thiserror::Error;
use simple_logger::SimpleLogger;

use crate::compiler::{compilation_configuration::CompilationConfiguration, output_format::OutputFormat};


#[derive(Error, Debug)]
pub enum CompilerCliError {
    #[error(transparent)]
    CompilationError(#[from] CompilationError),

    #[error("you must provide only one value of '{0}'")]
    MoreThanOneValue(String),

    #[error(transparent)]
    OutputFormatError(#[from] OutputFormatError),
}


pub struct CompilerCli {
    cli: Command
}

impl CompilerCli {

    pub fn new() -> Self {

        SimpleLogger::new()
            .with_level(LevelFilter::Debug)
            .init()
            .unwrap();


        let cli: Command = Command::new("nmd-compiler")
                .about("Official compiler to parse NMD")
                .version(Compiler::version())
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

                                )
                                
                );

        Self {
            cli
        }
    }

    pub fn parse(self) -> Result<(), CompilerCliError> {

        let matches = self.cli.get_matches();

        match matches.subcommand() {
            Some(("compile", compile_matches)) => {
                match compile_matches.subcommand() {
                    Some(("dossier", compile_dossier_matches)) => {

                        let mut compilation_configuration = CompilationConfiguration::default();
                        
                        if let Some(mut format) = compile_dossier_matches.get_many::<String>("format") {
                            
                            if format.len() != 1 {
                                return Err(CompilerCliError::MoreThanOneValue("format".to_string()));
                            }
                            
                            
                            let format = OutputFormat::from_str(format.nth(0).unwrap())?;

                            compilation_configuration.set_format(format);
                        }

                        if let Some(mut input_path) = compile_dossier_matches.get_many::<String>("input-path") {
                            
                            if input_path.len() != 1 {
                                return Err(CompilerCliError::MoreThanOneValue("input-path".to_string()));
                            }
                            
                            
                            let input_path = PathBuf::from(input_path.nth(0).unwrap());

                            compilation_configuration.set_input_location(input_path);
                        }

                        if let Some(mut output_path) = compile_dossier_matches.get_many::<String>("output-path") {
                            
                            if output_path.len() != 1 {
                                return Err(CompilerCliError::MoreThanOneValue("output-path".to_string()));
                            }
                            
                            
                            let output_path = PathBuf::from(output_path.nth(0).unwrap());

                            compilation_configuration.set_output_location(output_path);
                        }


                        Ok(Compiler::compile(compilation_configuration)?)

                    },

                    _ => unreachable!()
                }
            },

            _ => unreachable!()
        }
    }

}