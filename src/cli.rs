

use std::{path::PathBuf, str::FromStr};

use clap::{Arg, ArgAction, ArgMatches, Command};
use crate::compiler::{output_format::OutputFormatError, CompilationError};
use crate::dossier_manager::{dossier_manager_configuration::DossierManagerConfiguration, DossierManager, DossierManagerError};
use crate::generator::{generator_configuration::GeneratorConfiguration, Generator};
use log::{LevelFilter, ParseLevelError};
use crate::resource::ResourceError;
use thiserror::Error;
use simple_logger::SimpleLogger;

use crate::compiler::{compilation_configuration::CompilationConfiguration, output_format::OutputFormat};
use crate::Compiler;

use crate::VERSION;


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

    #[error("too few arguments: {0} needed")]
    TooFewArguments(String),

    #[error(transparent)]
    DossierManagerError(#[from] DossierManagerError),
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
                                            Arg::new("input-dossier-path")
                                            .short('i')
                                            .long("input-dossier-path")
                                            .help("input dossier path")
                                            .action(ArgAction::Set)
                                            .num_args(1)
                                            .default_value(".")

                                        )
                                        .arg(
                                            Arg::new("output-directory-path")
                                            .short('o')
                                            .long("output-directory-path")
                                            .help("output directory path")
                                            .action(ArgAction::Set)
                                            .num_args(1)
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
                )
                .subcommand(
                    Command::new("dossier")
                    .about("Manage NMD dossier")
                    .short_flag('d')
                    .subcommand_required(true)
                    .subcommand(
                        Command::new("add")
                        .about("Add resource to a dossier")
                        .short_flag('a')
                        .arg(
                            Arg::new("dossier-path")
                            .short('p')
                            .long("dossier-path")
                            .help("insert dossier path")
                            .action(ArgAction::Append)
                            .default_value(".")
                        )
                        .arg(
                            Arg::new("document-name")
                            .short('d')
                            .long("document-name")
                            .help("insert file name of the document")
                            .required(true)
                            .action(ArgAction::Append)
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

        let result = match matches.subcommand() {
            Some(("compile", compile_matches)) => {
                Self::handle_compile_command(&compile_matches)
            },

            Some(("generate", generate_matches)) => {
                Self::handle_generate_command(&generate_matches)                
            },

            Some(("dossier", dossier_matches)) => {
                Self::handle_dossier_command(&dossier_matches)
            },

            _ => unreachable!()
        };

        if let Err(error) = result {
            log::error!("{}", error);
            return Err(error)
        }

        Ok(())
    }

    fn set_logger(log_level: LevelFilter) {

        SimpleLogger::new()
            .with_level(log_level)
            .init()
            .unwrap();
    }

    fn handle_compile_command(matches: &ArgMatches) -> Result<(), NmdCliError> {
        match matches.subcommand() {
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

                if let Some(mut input_path) = compile_dossier_matches.get_many::<String>("input-dossier-path") {
                    
                    if input_path.len() != 1 {
                        return Err(NmdCliError::MoreThanOneValue("input-dossier-path".to_string()));
                    }
                    
                    
                    let input_path = PathBuf::from(input_path.nth(0).unwrap());

                    compilation_configuration.set_input_location(input_path);
                }

                if let Some(mut output_path) = compile_dossier_matches.get_many::<String>("output-directory-path") {
                    
                    if output_path.len() != 1 {
                        return Err(NmdCliError::MoreThanOneValue("output-directory-path".to_string()));
                    }
                    
                    
                    let output_path = PathBuf::from(output_path.nth(0).unwrap());

                    compilation_configuration.set_output_location(output_path);

                } else {

                    compilation_configuration.set_output_location(compilation_configuration.input_location().clone());
                }


                Ok(Compiler::compile_dossier(compilation_configuration)?)

            },

            _ => unreachable!()
        }
    }

    fn handle_generate_command(matches: &ArgMatches) -> Result<(), NmdCliError> {
        match matches.subcommand() {
            Some(("dossier", generate_dossier_matches)) => {

                if let Some(mut verbose) = generate_dossier_matches.get_many::<String>("verbose") {
                    
                    if verbose.len() != 1 {
                        return Err(NmdCliError::MoreThanOneValue("verbose".to_string()));
                    }
                    
                    
                    let log_level = LevelFilter::from_str(verbose.nth(0).unwrap())?;

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
    }

    fn handle_dossier_command(matches: &ArgMatches) -> Result<(), NmdCliError> {
        match matches.subcommand() {
            Some(("add", add_dossier_matches)) => {
                if let Some(mut verbose) = add_dossier_matches.get_many::<String>("verbose") {
                    
                    if verbose.len() != 1 {
                        return Err(NmdCliError::MoreThanOneValue("verbose".to_string()));
                    }
                    
                    
                    let log_level = LevelFilter::from_str(verbose.nth(0).unwrap())?;

                    Self::set_logger(log_level);
                }

                if let Some(mut dossier_path) = add_dossier_matches.get_many::<String>("dossier-path") {
                    
                    if dossier_path.len() != 1 {
                        return Err(NmdCliError::MoreThanOneValue("dossier-path".to_string()));
                    }

                    if let Some(document_names) = add_dossier_matches.get_many::<String>("document-name") {
                    
                        if document_names.len() < 1 {
                            return Err(NmdCliError::MoreThanOneValue("document-name".to_string()));
                        }

                        let dossier_path = PathBuf::from(dossier_path.nth(0).unwrap());
                
                        let dossier_manager_configuration = DossierManagerConfiguration::new(dossier_path);

                        let dossier_manager = DossierManager::new(dossier_manager_configuration);

                        for file_name in document_names {
                            dossier_manager.add_document(&file_name)?;
                        }

                        return Ok(())
                    }
                    
                }

                Err(NmdCliError::TooFewArguments("dossier path".to_string()))
            },

            _ => unreachable!()
        }
    }

}