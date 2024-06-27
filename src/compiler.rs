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

use std::{path::PathBuf, sync::{mpsc::{channel, RecvError}, Arc, RwLock}, thread, time::{Duration, Instant, SystemTime}};

use dossier::dossier_configuration::DossierConfiguration;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use thiserror::Error;
use crate::{compiler::{dossier::Dossier, dumpable::{DumpError, Dumpable}, loader::Loader, parsable::Parsable, parsing::parsing_metadata::ParsingMetadata}, constants::{DOSSIER_CONFIGURATION_JSON_FILE_NAME, DOSSIER_CONFIGURATION_YAML_FILE_NAME}};
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
    DumpError(#[from] DumpError),

    #[error(transparent)]
    WatcherError(#[from] notify::Error),

    #[error(transparent)]
    WatcherChannelError(#[from] RecvError),
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
        
        if parsing_configuration.fast_draft() {
            log::info!("fast draft!")
        }

        dossier.parse(Arc::clone(&codex), Arc::new(RwLock::new(parsing_configuration)), Arc::new(None))?;

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

    pub fn watch_compile(mut compilation_configuration: CompilationConfiguration, min_elapsed_time_between_events_in_secs: u64) -> Result<(), CompilationError> {

        let (tx, rx) = channel();

        let mut watcher = notify::recommended_watcher(move |res| {
            tx.send(res).unwrap();
        })?;

        watcher.watch(compilation_configuration.input_location(), RecursiveMode::Recursive)?;

        log::info!("watch mode ON: any modification to the dossier files will cause immediate recompilation");
        log::info!("press CTRL + C to terminate");

        let mut last_event_time = SystemTime::now();

        loop {
            match rx.recv() {
                Ok(res) => {


                    let original_log_max_level = log::max_level();

                    log::set_max_level(log::LevelFilter::Warn);

                    let dc = DossierConfiguration::try_from(compilation_configuration.input_location());

                    log::set_max_level(original_log_max_level);

                    if let Err(err) = dc {
                        log::error!("error during dossier configuration loading: {}", err);
                        continue;
                    }

                    let dc = dc.unwrap();

                    let mut relative_paths_to_monitoring = dc.raw_documents_paths().clone();
                    relative_paths_to_monitoring.push(String::from(DOSSIER_CONFIGURATION_YAML_FILE_NAME));
                    relative_paths_to_monitoring.push(String::from(DOSSIER_CONFIGURATION_JSON_FILE_NAME));
                    relative_paths_to_monitoring.push(String::from("assets/"));

                    let relative_paths_to_monitoring = Arc::new(relative_paths_to_monitoring);

                    let input_location_abs = compilation_configuration.input_location().canonicalize().unwrap(); 

                    match res {

                        Ok(event) => {
                            log::debug!("new event from watcher: {:?}", event);
                            log::debug!("change detected on file(s): {:?}", event.paths);

                            let event_time = SystemTime::now();

                            let elapsed_time = event_time.duration_since(last_event_time).unwrap();
                            
                            if elapsed_time.as_secs() < min_elapsed_time_between_events_in_secs {
                                log::info!("change detected, but minimum elapsed time not satisfied ({}/{} s)", elapsed_time.as_secs(), min_elapsed_time_between_events_in_secs);
                                continue;
                            }

                            last_event_time = event_time;
    
                            if let Some(_) = event.paths.par_iter().find_any(|path| {
    
                                let path = path.strip_prefix(input_location_abs.clone());

                                if let Ok(path) = path {
                                    let matched = relative_paths_to_monitoring.par_iter().find_any(|rptm| {
                                        log::debug!("{:?} contains {:?} -> {}", *rptm, path.to_string_lossy().to_string().as_str(), rptm.contains(path.to_string_lossy().to_string().as_str()));
                                        
                                        rptm.contains(path.to_string_lossy().to_string().as_str())
                                    });

                                    return matched.is_some()
                                }

                                false
                            }) {
                                log::info!("recompilation needed");

                                let compilation_configuration = compilation_configuration.clone();
    
                                let thread_res = thread::spawn(|| {
                                    let compilation_result = Self::compile_dossier(compilation_configuration);
    
                                    match compilation_result {
                                        Ok(_) => log::info!("compilation OK"),
                                        Err(err) => log::error!("error during compilation: {:?}", err)
                                    }
                                });
    
                            } else {
                                log::info!("recompilation not needed");
                            }
                            
                        },
                        Err(err) => {
                            log::error!("watch error: {:?}", err);
                            return Err(CompilationError::WatcherError(err))
                        }
                    }
                },
                Err(err) => {
                    log::error!("watch channel error: {:?}", err);
                    return Err(CompilationError::WatcherChannelError(err))
                },
            }
        }

        
        
    }
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