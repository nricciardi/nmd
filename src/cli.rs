

use std::collections::HashSet;
use std::{path::PathBuf, str::FromStr};

use clap::{Arg, ArgAction, ArgMatches, Command};
use crate::compiler::Compiler;
use crate::compiler::{output_format::OutputFormatError, CompilationError};
use crate::constants::{MINIMUM_WATCHER_TIME, VERSION};
use crate::dossier_manager::{dossier_manager_configuration::DossierManagerConfiguration, DossierManager, DossierManagerError};
use crate::generator::{generator_configuration::GeneratorConfiguration, Generator};
use log::{LevelFilter, ParseLevelError};
use crate::resource::ResourceError;
use thiserror::Error;
use simple_logger::SimpleLogger;

use crate::compiler::{compilation_configuration::CompilationConfiguration, output_format::OutputFormat};


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


/// NMD CLI. It is used as interface with NDM compiler, NDM generator and others
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
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("set verbose mode")
                        .action(ArgAction::Set)
                        .default_value("info")
                )
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
                                            Arg::new("watch")
                                                .short('w')
                                                .long("watch")
                                                .help("set to compile if files change")
                                                .action(ArgAction::SetTrue)
                                        )
                                        .arg(
                                            Arg::new("watcher-time")
                                                .short('t')
                                                .long("watcher-time")
                                                .help("set minimum watcher time interval")
                                                .action(ArgAction::Set)
                                        )
                                        .arg(
                                            Arg::new("fast-draft")
                                            .long("fast-draft")
                                            .help("fast draft instead of complete compilation")
                                            .action(ArgAction::SetTrue)
                                        )
                                        .arg(
                                            Arg::new("documents-subset")
                                            .short('s')
                                            .long("documents-subset")
                                            .help("compile only a documents subset")
                                            .action(ArgAction::Append)
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
                                            Arg::new("from-md")
                                                .long("from-md")
                                                .help("generate NMD dossier from Markdown file")
                                                .action(ArgAction::Set)
                                        )

                                )
                )
                .subcommand(
                    Command::new("dossier")
                    .about("Manage NMD dossier")
                    .short_flag('d')
                    .subcommand_required(true)
                    .arg(
                        Arg::new("dossier-path")
                            .short('p')
                            .long("dossier-path")
                            .help("insert dossier path")
                            .action(ArgAction::Append)
                            .default_value(".")
                            .required(true)
                    )
                    .subcommand(
                        Command::new("add")
                        .about("Add resource to a dossier")
                        .short_flag('a')
                        .arg(
                            Arg::new("document-name")
                            .short('d')
                            .long("document-name")
                            .help("insert file name of the document")
                            .required(true)
                            .action(ArgAction::Append)
                        )
                    )
                    .subcommand(
                        Command::new("reset")
                        .about("Reset dossier configuration")
                        .short_flag('r')
                        .arg(
                            Arg::new("preserve-documents")
                            .help("preserve documents list")
                            .short('p')
                            .long("preserve-documents")
                            .required(false)
                            .action(ArgAction::SetTrue)
                        )
                    )
                );

        Self {
            cli
        }
    }

    pub fn parse(self) -> Result<(), NmdCliError> {

        let matches = self.cli.get_matches();

        if let Some(verbose) = matches.get_one::<String>("verbose") {            
            
            let log_level = LevelFilter::from_str(verbose)?;

            Self::set_logger(log_level);
        }

        let result = match matches.subcommand() {

            Some(("compile", compile_matches)) => Self::handle_compile_command(&compile_matches),

            Some(("generate", generate_matches)) => Self::handle_generate_command(&generate_matches),

            Some(("dossier", dossier_matches)) => Self::handle_dossier_command(&dossier_matches),

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

                let mut compilation_configuration = CompilationConfiguration::default();
                
                if let Some(format) = compile_dossier_matches.get_one::<String>("format") {
                    
                    let format = OutputFormat::from_str(format)?;

                    compilation_configuration.set_format(format);
                }

                if let Some(input_path) = compile_dossier_matches.get_one::<String>("input-dossier-path") {
                                        
                    let input_path = PathBuf::from(input_path);

                    compilation_configuration.set_input_location(input_path);
                }

                if let Some(output_path) = compile_dossier_matches.get_one::<String>("output-directory-path") {
                    
                    let output_path = PathBuf::from(output_path);

                    compilation_configuration.set_output_location(output_path);

                } else {

                    compilation_configuration.set_output_location(compilation_configuration.input_location().clone());
                }

                if let Some(documents_subset) = compile_dossier_matches.get_many::<String>("documents-subset") {
                    
                    if documents_subset.len() < 1 {
                        return Err(NmdCliError::MoreThanOneValue("documents-subset".to_string()));
                    }

                    let mut subset: HashSet<String> = HashSet::new();
                    for file_name in documents_subset {
                        subset.insert(file_name.clone());
                    }

                    compilation_configuration.set_documents_subset_to_compile(Some(subset));
                }

                let watcher_time: u64;

                if let Some(wt) = compile_dossier_matches.get_one::<String>("watcher-time") {
                                        
                    watcher_time = wt.parse::<u64>().unwrap();

                } else {
                    watcher_time = MINIMUM_WATCHER_TIME;
                }

                let watch: bool = compile_dossier_matches.get_flag("watch");

                let fast_draft: bool = compile_dossier_matches.get_flag("fast-draft");

                compilation_configuration.set_fast_draft(fast_draft);

                if watch {
                    return Ok(Compiler::watch_compile(compilation_configuration, watcher_time)?)
                } else {
                    return Ok(Compiler::compile_dossier(compilation_configuration)?)
                }

            },

            _ => unreachable!()
        }
    }

    fn handle_generate_command(matches: &ArgMatches) -> Result<(), NmdCliError> {
        match matches.subcommand() {
            Some(("dossier", generate_dossier_matches)) => {
                
                let mut generator_configuration = GeneratorConfiguration::default();

                if let Some(input_path) = generate_dossier_matches.get_one::<String>("path") {
                     
                    let input_path = PathBuf::from(input_path);

                    generator_configuration.set_path(input_path);
                }

                let md_file_path: Option<PathBuf>;
                if let Some(md_fp) = generate_dossier_matches.get_one::<String>("from-md") {
                    
                    md_file_path = Some(PathBuf::from(md_fp));
                
                } else {
                    md_file_path = None;
                }

                generator_configuration.set_force_generation(generate_dossier_matches.get_flag("force"));
                generator_configuration.set_gitkeep(generate_dossier_matches.get_flag("gitkeep"));
                generator_configuration.set_welcome(generate_dossier_matches.get_flag("welcome"));
                
                if let Some(md_file_path) = md_file_path {

                    Generator::generate_dossier_from_markdown_file(&md_file_path, generator_configuration)?;
                    
                } else {

                    Generator::generate_dossier(generator_configuration)?;
                }

                Ok(())
            },
            _ => unreachable!()
        }
    }

    fn handle_dossier_command(matches: &ArgMatches) -> Result<(), NmdCliError> {

        let dossier_path: Option<PathBuf>;

        if let Some(dp) = matches.get_one::<String>("dossier-path") {
                    
            dossier_path = Some(PathBuf::from(dp));
        
        } else {
            
            dossier_path = None;
        }

        match matches.subcommand() {
            Some(("add", add_dossier_matches)) => {

                if let Some(dp) = dossier_path {
                    
                    if let Some(document_names) = add_dossier_matches.get_many::<String>("document-name") {
                    
                        let dossier_manager_configuration = DossierManagerConfiguration::new(dp);

                        let dossier_manager = DossierManager::new(dossier_manager_configuration);

                        for file_name in document_names {
                            dossier_manager.add_empty_document(&file_name)?;
                        }

                        return Ok(())
                    }
                    
                }

                Err(NmdCliError::TooFewArguments("dossier path".to_string()))
            },

            Some(("reset", reset_dossier_matches)) => {
                
                if let Some(dp) = dossier_path {

                    let dossier_manager_configuration = DossierManagerConfiguration::new(dp.clone());

                    let dossier_manager = DossierManager::new(dossier_manager_configuration);
                    
                    dossier_manager.reset_dossier_configuration(dp, reset_dossier_matches.get_flag("preserve-documents"))?;
                    
                    return Ok(())
                    
                }

                Err(NmdCliError::TooFewArguments("dossier path".to_string()))
            }

            _ => unreachable!()
        }
    }

}