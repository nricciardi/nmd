use std::collections::HashSet;
use std::io::{stdout, Write};
use std::num::ParseIntError;
use std::ops::Deref;
use std::sync::Arc;
use nmd_core::compilation::compilation_configuration::CompilableResourceType;
use nmd_core::output_format::{OutputFormat, OutputFormatError};
use nmd_core::resource::ResourceError;
use nmd_core::theme::{Theme, ThemeError};
use nmd_core::utility::file_utility;
use tokio::sync::RwLock as TokioRwLock;
use std::{path::PathBuf, str::FromStr};
use clap::{Arg, ArgAction, ArgMatches, Command};
use tokio::task::{JoinError, JoinHandle};
use crate::builder::builder_configuration::BuilderConfiguration;
use crate::builder::builder_error::BuilderError;
use crate::builder::Builder;
use crate::constants::{MINIMUM_WATCHER_TIME, VERSION};
use crate::dossier_manager::{dossier_manager_configuration::DossierManagerConfiguration, DossierManager, DossierManagerError};
use crate::generator::{generator_configuration::GeneratorConfiguration, Generator};
use crate::preview::html_preview::HtmlPreview;
use crate::preview::PreviewError;
use crate::preview::Preview;
use log::{LevelFilter, ParseLevelError};
use thiserror::Error;
use simple_logger::SimpleLogger;


#[derive(Error, Debug)]
pub enum NmdCliError {

    #[error("bad command")]
    BadCommand,

    #[error("unknown resource")]
    UnknownResource,

    #[error(transparent)]
    BuilderError(#[from] BuilderError),

    #[error("you must provide only one value of '{0}'")]
    MoreThanOneValue(String),

    #[error(transparent)]
    OutputFormatError(#[from] OutputFormatError),

    #[error(transparent)]
    ThemeError(#[from] ThemeError),

    #[error(transparent)]
    VerboseLevelError(#[from] ParseLevelError),

    #[error(transparent)]
    ResourceError(#[from] ResourceError),

    #[error("too few arguments: {0} needed")]
    TooFewArguments(String),

    #[error(transparent)]
    DossierManagerError(#[from] DossierManagerError),

    #[error(transparent)]
    PreviewError(#[from] PreviewError),

    #[error(transparent)]
    JoinError(#[from] JoinError),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}


/// NMD CLI. It is used as interface with NDM compiler, NDM generator and others
pub struct NmdCli {
    cli: Command
}

impl NmdCli {

    pub fn new() -> Self {

        let cli: Command = Command::new("nmd")
                .about("Official NMD command line interface")
                .version(VERSION.unwrap_or("unknown"))
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
                    Command::new("build")
                                .about("Build a NMD file or dossier")
                                .short_flag('b')
                                .alias("compile")
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
                                    Arg::new("force-output")
                                    .long("force")
                                    .help("force output if destination not exists")
                                    .action(ArgAction::SetTrue)
                                )
                                .arg(
                                    Arg::new("theme")
                                        .short('t')
                                        .long("theme")
                                        .help("set theme")
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
                                        .long("watcher-time")
                                        .help("set minimum watcher time interval")
                                        .action(ArgAction::Set)
                                )
                                .arg(
                                    Arg::new("preview")
                                        .short('p')
                                        .long("preview")
                                        .help("show preview")
                                        .action(ArgAction::SetTrue)
                                )
                                .arg(
                                    Arg::new("preview-scraping-interval")
                                        .long("preview-scraping-interval")
                                        .help("set preview scraping interval")
                                        .required(false)
                                        .action(ArgAction::Set)
                                )
                                .arg(
                                    Arg::new("fast-draft")
                                    .long("fast-draft")
                                    .help("fast draft instead of complete compilation")
                                    .action(ArgAction::SetTrue)
                                )
                                .arg(
                                    Arg::new("style-file")
                                    .long("style-file")
                                    .help("add style file")
                                    .action(ArgAction::Append)
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
                                )
                                .arg(
                                    Arg::new("documents-subset")
                                    .short('s')
                                    .long("documents-subset")
                                    .help("compile only a documents subset")
                                    .action(ArgAction::Append)
                                )
                                .arg(
                                    Arg::new("parallelization")
                                    .long("parallelization")
                                    .help("set parallelization")
                                    .action(ArgAction::SetTrue)
                                )
                                .arg(
                                    Arg::new("embed-local-image")
                                    .long("embed-local-image")
                                    .help("set embed local image")
                                    .action(ArgAction::SetTrue)
                                )
                                .arg(
                                    Arg::new("embed-remote-image")
                                    .long("embed-remote-image")
                                    .help("set embed remote image")
                                    .action(ArgAction::SetTrue)
                                )
                                .arg(
                                    Arg::new("compress-embed-image")
                                    .long("compress-embed-image")
                                    .help("set compress embed image")
                                    .action(ArgAction::SetTrue)
                                )
                                .arg(
                                    Arg::new("strict-image-src-check")
                                    .long("strict-image-src-check")
                                    .help("set strict image source check")
                                    .action(ArgAction::SetTrue)
                                )
                                .arg(
                                    Arg::new("nuid")
                                    .long("nuid")
                                    .help("set nuid")
                                    .action(ArgAction::SetTrue)
                                )
                )
                .subcommand(
                    Command::new("generate")
                        .about("Generate a new NMD resource")
                        .short_flag('g')
                        .alias("new")
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
                                        .arg(
                                            Arg::new("name")
                                                .long("name")
                                                .short('n')
                                                .help("set dossier name")
                                                .action(ArgAction::Set)
                                        )

                                )
                )
                .subcommand(
                    Command::new("dossier")
                    .about("Manage NMD dossier")
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
                            Arg::new("no-preserve-documents")
                            .help("no preserve documents list")
                            .long("no-preserve-documents")
                            .required(false)
                            .action(ArgAction::SetTrue)
                        )
                    )
                )
                .subcommand(
                    Command::new("analyze")
                    .about("Analyze NMD dossier or document")
                    .subcommand_required(false)
                    .arg(
                        Arg::new("input-path")
                            .short('i')
                            .long("input")
                            .help("insert input path")
                            .action(ArgAction::Append)
                            .default_value(".")
                            .required(true)
                    )
                    .arg(
                        Arg::new("nuid")
                        .long("nuid")
                        .help("set nuid")
                        .action(ArgAction::SetTrue)
                    )
                    .arg(
                        Arg::new("pretty")
                        .long("pretty")
                        .help("pretty json")
                        .action(ArgAction::SetTrue)
                    )
                );
        Self {
            cli
        }
    }

    pub async fn serve(self) -> Result<(), NmdCliError> {

        let matches = self.cli.get_matches();

        if let Some(verbose) = matches.get_one::<String>("verbose") {            
            
            let log_level = LevelFilter::from_str(verbose)?;

            Self::set_logger(log_level);
        }

        let result: Result<(), NmdCliError> = match matches.subcommand() {

            Some(("build", compile_matches)) => Self::handle_build_command(&compile_matches).await,

            Some(("generate", generate_matches)) => Self::handle_generate_command(&generate_matches).await,

            Some(("dossier", dossier_matches)) => Self::handle_dossier_command(&dossier_matches).await,

            Some(("analyze", analyze_matches)) => Self::handle_analyze_command(&analyze_matches).await,

            _ => {
                log::error!("bad command");

                return Err(NmdCliError::BadCommand)
            }
        };

        if let Err(error) = result {
            log::error!("{}", error);
            return Err(error)
        }

        Ok(())
    }

    fn set_logger(log_level: LevelFilter) {

        SimpleLogger::new()
            .without_timestamps()
            .with_level(log_level)
            .init()
            .unwrap();
    }

    async fn handle_build_command(matches: &ArgMatches) -> Result<(), NmdCliError> {

        let mut builder_configuration = BuilderConfiguration::default();

        // FORMAT
        if let Some(format) = matches.get_one::<String>("format") {
                    
            let format = OutputFormat::from_str(format)?;

            builder_configuration.set_format(format);
        }

        let there_is_preview = matches.get_flag("preview");

        builder_configuration.set_preview(Some(there_is_preview));

        if there_is_preview {

            assert!(builder_configuration.format().eq(&OutputFormat::Html));        // there is only HtmlPreview
        }

        // INPUT & OUTPUT PATHs
        if let Some(input_path) = matches.get_one::<String>("input-path") {
                                        
            let input_path = PathBuf::from(input_path);

            builder_configuration.set_input_location(input_path);
        }

        if let Some(output_path) = matches.get_one::<String>("output-path") {
                    
            let output_path = PathBuf::from(output_path);

            builder_configuration.set_output_location(output_path);

        } else {
            
            match builder_configuration.resource_type() {
                CompilableResourceType::Dossier => {

                    builder_configuration.set_output_location(builder_configuration.input_location().clone());      // could be a dir or a file

                    if there_is_preview && builder_configuration.output_location().is_dir() {

                        builder_configuration.set_output_location(builder_configuration.output_location().join(file_utility::build_output_file_name(
                            "nmd-dossier-preview",
                            Some(&builder_configuration.format().get_extension())
                        )));
                    }
                },
                CompilableResourceType::File => {

                    let mut output_path = builder_configuration.input_location().clone();

                    if output_path.is_dir() {
                        
                        output_path = output_path.join(file_utility::build_output_file_name(
                            output_path.file_stem().unwrap().to_string_lossy().to_string().as_str(),
                        Some(&builder_configuration.format().get_extension())
                        ));

                    } else {

                        output_path = output_path.parent().unwrap().join(file_utility::build_output_file_name(
                            output_path.file_stem().unwrap().to_string_lossy().to_string().as_str(),
                        Some(&builder_configuration.format().get_extension())
                        ));
                    }

                    builder_configuration.set_output_location(output_path);
                },
                CompilableResourceType::Unknown => (),
            }
        }

        // PREVIEW
        let preview: Option<Arc<TokioRwLock<HtmlPreview>>>;
        let preview_start_handle: Option<JoinHandle<Result<(), PreviewError>>>;

        if there_is_preview {

            let scraping_interval: Option<u32>; 
            
            if let Some(interval) = matches.get_one::<String>("preview-scraping-interval") {

                scraping_interval = Some(interval.clone().parse::<u32>()?);

            } else {
                scraping_interval = None;
            }

            let p = HtmlPreview::new(builder_configuration.output_location().clone(), scraping_interval);

            let p = Arc::new(TokioRwLock::new(p));

            let handle = tokio::spawn({

                let p = Arc::clone(&p);

                async move {
                    p.write().await.start().await
                }
            });

            preview = Some(p);
            preview_start_handle = Some(handle);
        
        } else {

            preview = None;
            preview_start_handle = None;
        }
        
        if let Some(theme) = matches.get_one::<String>("theme") {
                    
            let theme = Theme::from_str(theme)?;

            builder_configuration.set_theme(Some(theme));
        }

        // WATCHER
        let watcher_time: u64;

        if let Some(wt) = matches.get_one::<String>("watcher-time") {
                                
            watcher_time = wt.parse::<u64>().unwrap();

        } else {
            watcher_time = MINIMUM_WATCHER_TIME;
        }

        let watch: bool = matches.get_flag("watch");

        builder_configuration.set_watching(Some(watch));

        
        // FAST DRAFT, FORCE, STYLEs
        let fast_draft: bool = matches.get_flag("fast-draft");

        builder_configuration.set_fast_draft(Some(fast_draft));

        builder_configuration.set_force_output(Some(matches.get_flag("force-output")));

        if let Some(styles) = matches.get_many::<String>("style-file") {
            builder_configuration.set_styles_raw_path(styles.map(|s| s.clone()).collect());
        }

        // PARALLELIZATION
        if matches.get_flag("parallelization") {
            builder_configuration.set_parallelization(Some(true));
        }

        // NUID
        if matches.get_flag("nuid") {
            builder_configuration.set_nuid(Some(true));
        }

        // IMAGEs
        if matches.get_flag("embed-local-image") {
            builder_configuration.set_embed_local_image(Some(true));
        }

        if matches.get_flag("embed-remote-image") {
            builder_configuration.set_embed_remote_image(Some(true));
        }

        if matches.get_flag("compress-embed-image") {
            builder_configuration.set_compress_embed_image(Some(true));
        }
        
        if matches.get_flag("strict-image-src-check") {
            builder_configuration.set_strict_image_src_check(Some(true));
        }

        // DOCUMENT SUBSET (only if dossier)
        if let Some(documents_subset) = matches.get_many::<String>("documents-subset") {
                    
            if documents_subset.len() < 1 {
                return Err(NmdCliError::MoreThanOneValue("documents-subset".to_string()));
            }

            let mut subset: HashSet<String> = HashSet::new();
            for file_name in documents_subset {
                subset.insert(file_name.clone());
            }

            builder_configuration.set_documents_subset_to_compile(Some(subset));
        }

        // wait preview startup
        if let Some(handle) = preview_start_handle {
            handle.await??;
        }

        let builder_configuration = Arc::new(TokioRwLock::new(builder_configuration));

        let build_handle: JoinHandle<Result<(), BuilderError>>;

        match builder_configuration.read().await.resource_type() {
            CompilableResourceType::Dossier => {

                if watch {
                    build_handle = tokio::spawn({

                        let preview = preview.clone();
                        let builder_configuration = builder_configuration.clone();

                        async move {
                            Builder::watch_compile_dossier(builder_configuration.read().await.deref().clone(), watcher_time, preview).await
                        }
                    });

                } else {
                    
                    build_handle = tokio::spawn({

                        let builder_configuration = builder_configuration.clone();

                        async move {

                            let mut dossier = Builder::load_dossier(builder_configuration.read().await.deref()).await?;

                            builder_configuration.write().await.merge_dossier_configuration(dossier.configuration());
    
                            Builder::build_dossier(&mut dossier, builder_configuration.read().await.deref()).await
                        }
                    });

                    if let Some(p) = preview.clone() {

                        tokio::spawn(async move {
                            p.write().await.render().await
                        }).await??;
                    }
                }

            },
            CompilableResourceType::File => {

                if watch {

                    build_handle = tokio::spawn({
                        async move {
                            unimplemented!("watch compile file will be added in a next version")
                        }
                    });

                } else {

                    build_handle = tokio::spawn({

                        let builder_configuration = builder_configuration.clone();

                        async move {

                            Builder::build_document(builder_configuration.read().await.deref()).await
                        }
                    });

                    if let Some(p) = preview.clone() {

                        tokio::spawn(async move {
                            p.write().await.render().await
                        }).await??;
                    }
                }
            },

            CompilableResourceType::Unknown => return Err(NmdCliError::ResourceError(ResourceError::InvalidResourceVerbose("resource is a dossier nor file".to_string()))),
        }

        build_handle.await??;

        if let Some(preview) = preview {
            preview.write().await.stop().await?;       // need Ctrl + C to terminate
        }

        Ok(())
    }

    async fn handle_generate_command(matches: &ArgMatches) -> Result<(), NmdCliError> {
        match matches.subcommand() {
            Some(("dossier", generate_dossier_matches)) => {
                
                let mut generator_configuration = GeneratorConfiguration::default();

                if let Some(name) = generate_dossier_matches.get_one::<String>("name") {
                     
                    generator_configuration.set_name(Some(name.clone()));
                }

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

    async fn handle_dossier_command(matches: &ArgMatches) -> Result<(), NmdCliError> {

        let dossier_path = PathBuf::from(matches.get_one::<String>("dossier-path").unwrap());
    
        match matches.subcommand() {
            Some(("add", add_dossier_matches)) => {

                if let Some(document_names) = add_dossier_matches.get_many::<String>("document-name") {
                    
                    let dossier_manager_configuration = DossierManagerConfiguration::new(dossier_path);

                    let dossier_manager = DossierManager::new(dossier_manager_configuration);

                    for file_name in document_names {
                        dossier_manager.add_empty_document(&file_name)?;
                    }

                    return Ok(())
                }

                Err(NmdCliError::TooFewArguments("dossier path".to_string()))
            },

            Some(("reset", reset_dossier_matches)) => {
                
                let dossier_manager_configuration = DossierManagerConfiguration::new(dossier_path.clone());

                let dossier_manager = DossierManager::new(dossier_manager_configuration);
                
                dossier_manager.reset_dossier_configuration(dossier_path, !reset_dossier_matches.get_flag("no-preserve-documents"))?;
                
                Ok(())
            },

            _ => unreachable!()
        }
    }

    async fn handle_analyze_command(matches: &ArgMatches) -> Result<(), NmdCliError> {
        let mut builder_configuration = BuilderConfiguration::default();

        builder_configuration.set_input_location(PathBuf::from(matches.get_one::<String>("input-path").unwrap()));

        if matches.get_flag("nuid") {
            builder_configuration.set_nuid(Some(true));
        }

        let json_output: String;
        
        if matches.get_flag("pretty") {

            match builder_configuration.resource_type() {
                CompilableResourceType::Dossier => {

                    let dossier = Builder::load_dossier(&builder_configuration).await?;

                    json_output = serde_json::to_string_pretty(&dossier)?;
                },

                CompilableResourceType::File => {
                    let document = Builder::load_document(&builder_configuration).await?;

                    json_output = serde_json::to_string_pretty(&document)?;
                },


                CompilableResourceType::Unknown => {
                    log::error!("unknown resource");

                    return Err(NmdCliError::UnknownResource)
                },
            }

        } else {
            
            match builder_configuration.resource_type() {
                CompilableResourceType::Dossier => {

                    let dossier = Builder::load_dossier(&builder_configuration).await?;

                    json_output = serde_json::to_string(&dossier)?;
                },

                CompilableResourceType::File => {
                    
                    let document = Builder::load_document(&builder_configuration).await?;

                    json_output = serde_json::to_string(&document)?;
                },


                CompilableResourceType::Unknown => {
                    log::error!("unknown resource");

                    return Err(NmdCliError::UnknownResource)
                },
            }
        }

        stdout().write_all(json_output.as_bytes())?;

        Ok(())
    }

}