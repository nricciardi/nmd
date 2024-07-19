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
pub mod table_of_contents;
pub mod bibliography;
pub mod preview;
pub mod watcher;

use std::sync::RwLock;
use std::{sync::Arc, time::Instant};

use dossier::{dossier_configuration::DossierConfiguration, Document, Dossier};
use dumpable::DumpConfiguration;
use preview::{html_preview::HtmlPreview, Preview, PreviewError};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use theme::Theme;
use thiserror::Error;
use tokio::task::{spawn_blocking, JoinHandle};
use tokio::sync::RwLock as TokioRwLock;
use watcher::{NmdWatcher, WatcherError};
use crate::{compiler::{dumpable::{DumpError, Dumpable}, loader::Loader, parsable::Parsable}, constants::{DOSSIER_CONFIGURATION_JSON_FILE_NAME, DOSSIER_CONFIGURATION_YAML_FILE_NAME}, utility::file_utility};
use self::{assembler::{assembler_configuration::AssemblerConfiguration, AssemblerError}, compilation_configuration::CompilationConfiguration, loader::LoadError, parsing::parsing_error::ParsingError};


#[derive(Error, Debug)]
pub enum CompilationError {

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
    PreviewError(#[from] PreviewError),

    #[error(transparent)]
    WatcherError(#[from] WatcherError),
}

pub struct Compiler {
}

impl Compiler {

    /// Standard dossier compilation based on CompilationConfiguration.
    /// It loads, parses and dumps dossier
    pub fn compile_dossier(mut compilation_configuration: CompilationConfiguration) -> Result<(), CompilationError> {

        log::info!("start to compile dossier");

        let compile_start = Instant::now();

        log::info!("compilation configuration (this will override dossier compilation configuration):\n\n{:#?}\n", compilation_configuration);

        let codex = Arc::new(compilation_configuration.codex());

        let mut dossier: Dossier;

        let loader = Loader::new();

        if let Some(dstc) = compilation_configuration.documents_subset_to_compile() {

            dossier = loader.load_dossier_from_path_buf_only_documents(&codex, compilation_configuration.input_location(), dstc)?;

        } else {

            dossier = loader.load_dossier_from_path_buf(&codex, compilation_configuration.input_location())?;
        }

        log::info!("dossier loaded in {} ms", compile_start.elapsed().as_millis());

        let dossier_configuration = dossier.configuration();

        compilation_configuration.merge_dossier_configuration(dossier_configuration);

        let mut parsing_configuration = compilation_configuration.parsing_configuration();
        parsing_configuration.set_list_bullets_configuration(dossier_configuration.style().list_bullets_configuration().clone());
        parsing_configuration.set_strict_list_check(dossier_configuration.compilation().strict_list_check());

        if parsing_configuration.compress_embed_image() || parsing_configuration.embed_local_image() || parsing_configuration.embed_remote_image() {

            log::warn!("embedding or compressing images is a time consuming task! Consider not using this feature unless strictly necessary");
        }

        log::info!("will use dossier configuration: {:?}", compilation_configuration.input_location());
        log::debug!("will use dossier configuration:\n\n{:#?}\n", dossier_configuration);

        let mut assembler_configuration = AssemblerConfiguration::from(dossier_configuration.clone());

        let dossier_theme = dossier_configuration.style().theme().clone();
        
        log::info!("parsing using theme: {}", parsing_configuration.theme());
        log::debug!("parsing configuration:\n{:#?}\n", parsing_configuration);
        
        if parsing_configuration.fast_draft() {
            log::info!("fast draft mode on!")
        }

        dossier.parse(compilation_configuration.format(), Arc::clone(&codex), Arc::new(RwLock::new(parsing_configuration)), Arc::new(None))?;

        assembler_configuration.set_theme(compilation_configuration.theme().as_ref().unwrap_or(&dossier_theme).clone());

        log::info!("assembling...");

        let assembly_time = Instant::now();

        let assembler = assembler::from(compilation_configuration.format().clone(), assembler_configuration);

        let mut artifact = assembler.assemble_dossier(&dossier)?;

        log::info!("end to assembly (assembly time {} ms)", assembly_time.elapsed().as_millis());

        let mut output_location = compilation_configuration.output_location().clone();

        if output_location.is_dir() {
            output_location = output_location.join(file_utility::build_output_file_name(
                &dossier.name(),
            Some(&compilation_configuration.format().get_extension())
            ));
        }

        let dump_configuration = DumpConfiguration::new(output_location, compilation_configuration.force_output());

        artifact.dump(&dump_configuration)?;

        log::info!("end to compile dossier (compile time: {} ms)", compile_start.elapsed().as_millis());

        Ok(())
    }

    /// Watch filesystem and compile dossier if any changes occur
    /// 
    /// - min_elapsed_time_between_events_in_secs is the minimum time interval between two compilation
    pub async fn watch_compile_dossier(compilation_configuration: CompilationConfiguration, min_elapsed_time_between_events_in_secs: u64) -> Result<(), CompilationError> {

        let preview: HtmlPreview = HtmlPreview::new(compilation_configuration.output_location().clone());

        let preview = Arc::new(TokioRwLock::new(preview));

        let input_location_abs = compilation_configuration.input_location().canonicalize().unwrap(); 

        let compilation_configuration = Arc::new(TokioRwLock::new(compilation_configuration));

        let mut watcher = NmdWatcher::new(
            min_elapsed_time_between_events_in_secs,
            &input_location_abs,
            Box::new(|| {

                let preview = Arc::clone(&preview);

                Box::pin(async move {
                    let res = preview.write().await.start().await;
    
                    if let Err(e) = res {
                        log::error!("error occurs during preview start: {}", e);
    
                        return Err(WatcherError::PreviewError(e));
                    }
    
                    Ok(())
                })
            }),
            Box::new(|event| {

                let input_location_abs = input_location_abs.clone();

                Box::pin(async move {

                    if event.paths.contains(&input_location_abs.join(DOSSIER_CONFIGURATION_YAML_FILE_NAME)) ||
                        event.paths.contains(&input_location_abs.join(DOSSIER_CONFIGURATION_JSON_FILE_NAME)) {
    
                        log::info!("recompilation needed");
                        return Ok(true)
                    }
    
                    Ok(false)
                })
            }),
            Box::new(|event| {

                let compilation_configuration = Arc::clone(&compilation_configuration);

                let input_location_abs = input_location_abs.clone();

                Box::pin(async move {

                    let original_log_max_level = log::max_level();

                    log::set_max_level(log::LevelFilter::Warn);

                    let dc = DossierConfiguration::try_from(compilation_configuration.read().await.input_location());

                    log::set_max_level(original_log_max_level);

                    if let Err(err) = dc {
                        log::error!("error during dossier configuration loading: {}", err);

                        return Ok(false)
                    }

                    let dc = dc.unwrap();

                    let mut relative_paths_to_monitoring = dc.raw_documents_paths().clone();
                    relative_paths_to_monitoring.push(String::from("assets/"));

                    let relative_paths_to_monitoring = Arc::new(relative_paths_to_monitoring);

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

                        return Ok(true)


                    } else {
                        log::info!("recompilation not needed");

                        return Ok(false)
                    }
                })
            }),
            Box::new(|| {
                Box::pin({

                    let compilation_configuration = Arc::clone(&compilation_configuration);
                    let preview = Arc::clone(&preview);

                    async move {
                        let compilation_result = Self::compile_dossier(compilation_configuration.read().await.clone());
        
                        match compilation_result {
                            Ok(_) => {
        
                                log::info!("compilation OK");
        
                                // TODO
                                preview.write().await.update().await?;
        
                                return Ok(())
                            },
                            Err(err) => {
                                log::error!("error during compilation: {:?}", err);
        
                                return Err(WatcherError::ElaborationError(err.to_string()))
                            }
                        }
                    }
                })
            }),
        )?;

        log::info!("watch mode ON: any modification to the dossier files will cause immediate recompilation");
        log::info!("press CTRL + C to terminate");
        
        watcher.start().await?;

        preview.write().await.render().await?;

        log::info!("stop watching...");

        preview.write().await.stop().await?;

        Ok(())
        
    }

    /// Standard file compilation based on CompilationConfiguration.
    /// It loads, parses and dumps dossier
    pub fn compile_file(mut compilation_configuration: CompilationConfiguration) -> Result<(), CompilationError> {

        log::info!("start to compile dossier");

        let compile_start = Instant::now();

        log::info!("compilation configuration (this will override dossier compilation configuration):\n\n{:#?}\n", compilation_configuration);

        let codex = compilation_configuration.codex();

        let loader = Loader::new();

        let mut document: Document = loader.load_document_from_path(&codex, compilation_configuration.input_location())?;

        log::info!("document loaded in {} ms", compile_start.elapsed().as_millis());

        compilation_configuration.fill_with_default();

        let parsing_configuration = compilation_configuration.parsing_configuration();

        if parsing_configuration.compress_embed_image() || parsing_configuration.embed_local_image() || parsing_configuration.embed_remote_image() {

            log::warn!("embedding or compressing images is a time consuming task! Consider not using this feature unless strictly necessary");
        }

        log::info!("will use dossier configuration: {:?}", compilation_configuration.input_location());

        let mut assembler_configuration = AssemblerConfiguration::default();
        
        log::info!("parsing using theme: {}", parsing_configuration.theme());
        log::debug!("parsing configuration:\n{:#?}\n", parsing_configuration);
        
        if parsing_configuration.fast_draft() {
            log::info!("fast draft mode on!")
        }

        let codex = Arc::new(codex);

        document.parse(compilation_configuration.format(), Arc::clone(&codex), Arc::new(RwLock::new(parsing_configuration)), Arc::new(None))?;

        assembler_configuration.set_theme(compilation_configuration.theme().clone().unwrap_or(Theme::default()));

        log::info!("assembling...");

        let mut output_location = compilation_configuration.output_location().clone();

        if output_location.is_dir() {
            output_location = output_location.join(file_utility::build_output_file_name(
                compilation_configuration.input_location().file_stem().unwrap().to_string_lossy().to_string().as_str(),
            Some(&compilation_configuration.format().get_extension())
            ));
        }

        let assembly_time = Instant::now();

        let assembler = assembler::from(compilation_configuration.format().clone(), assembler_configuration);

        let mut artifact = assembler.assemble_document_standalone(&output_location.file_name().unwrap().to_string_lossy().to_string(), Some(compilation_configuration.styles_raw_path()), None, None, &document)?;

        log::info!("end to assembly (assembly time {} ms)", assembly_time.elapsed().as_millis());

        let dump_configuration = DumpConfiguration::new(output_location, compilation_configuration.force_output());

        artifact.dump(&dump_configuration)?;

        log::info!("end to compile dossier (compile time: {} ms)", compile_start.elapsed().as_millis());

        Ok(())
    }

    pub fn watch_compile_file(compilation_configuration: CompilationConfiguration, min_elapsed_time_between_events_in_secs: u64) -> Result<(), CompilationError> {
        unimplemented!("watch compile file will be added in a next version")
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