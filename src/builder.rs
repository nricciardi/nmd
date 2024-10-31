pub mod builder_error;
pub mod builder_configuration;
mod constants;


use std::{borrow::Borrow, collections::HashSet, path::PathBuf, sync::Arc, time::Instant};
use builder_configuration::BuilderConfiguration;
use builder_error::BuilderError;
use nmd_core::dossier::Dossier;
use nmd_core::load::{LoadConfiguration, LoadConfigurationOverLay};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tokio::{sync::RwLock as TokioRwLock, task::JoinSet};
use crate::preview::{html_preview::PREVIEW_URL, Preview};
use crate::{preview::html_preview::HtmlPreview, watcher::{NmdWatcher, WatcherError}};



pub struct Builder {
}

impl Builder {

    /// Load dossier from `BuilderConfiguration`
    pub async fn load_dossier(builder_configuration: &BuilderConfiguration) -> Result<Dossier, BuilderError> {
        
        log::info!("start to load dossier {:?}", builder_configuration.input_location());

        let loading_start = Instant::now();

        let mut loader_configuration = LoadConfiguration::default();
        loader_configuration.set_input_location(builder_configuration.input_location().clone());

        let loader_configuration_overlay = LoadConfigurationOverLay::default();

        let mut dossier: Dossier;

        if let Some(dstc) = builder_configuration.documents_subset_to_compile() {

            dossier = Loader::load_dossier_from_path_buf_only_documents(builder_configuration.input_location(), &dstc, &builder_configuration.codex(), &loader_configuration, loader_configuration_overlay)?;

        } else {

            dossier = Loader::load_dossier_from_path_buf(builder_configuration.input_location(), &builder_configuration.codex(), &loader_configuration, loader_configuration_overlay)?;
        }

        if let Some(with_nuid) = builder_configuration.nuid() {
            if with_nuid {
                log::info!("assign nuid...");
                dossier.documents_mut().iter_mut().for_each(|d| assign_nuid_to_document_paragraphs(d));
            }
        }

        log::info!("dossier loaded in {} ms", loading_start.elapsed().as_millis());

        Ok(dossier)
    }

    pub async fn build_dossier(dossier: &mut Dossier, builder_configuration: &BuilderConfiguration) -> Result<(), BuilderError> {
        Self::build_dossier_compiling_subset(dossier, builder_configuration, None).await
    }

    pub async fn build_dossier_compiling_subset(dossier: &mut Dossier, builder_configuration: &BuilderConfiguration, subset_documents_to_parse: Option<HashSet<String>>) -> Result<(), BuilderError> {
        
        log::info!("start to compile dossier");

        let compilation_start = Instant::now();

        let mut compilation_configuration = builder_configuration.generate_compilation_configuration();

        compilation_configuration.set_list_bullets_configuration(dossier.configuration().style().list_bullets_configuration().clone());
        compilation_configuration.set_strict_list_check(dossier.configuration().compilation().strict_list_check());

        if compilation_configuration.compress_embed_image() || compilation_configuration.embed_local_image() || compilation_configuration.embed_remote_image() {

            log::warn!("embedding or compressing images is a time consuming task! Consider not using this feature unless strictly necessary");
        }

        log::info!("will use dossier configuration: {:?}", compilation_configuration.input_location());
        log::debug!("will use dossier configuration:\n\n{:#?}\n", dossier.configuration());
        
        log::info!("parsing using theme: {}", compilation_configuration.theme());
        log::debug!("parsing configuration:\n{:#?}\n", compilation_configuration);
        
        if compilation_configuration.fast_draft() {
            log::info!("fast draft mode on!")
        }

        let mut compilation_configuration_overlay = CompilationConfigurationOverLay::default();

        if let Some(subset) = subset_documents_to_parse {

            compilation_configuration_overlay.set_compile_only_documents(Some(subset));
        }

        Compiler::compile_dossier(dossier, builder_configuration.format(), &builder_configuration.codex(), &compilation_configuration, compilation_configuration_overlay)?;

        log::info!("dossier compiled in {} ms", compilation_start.elapsed().as_millis());

        log::info!("assembling...");

        let assembly_time = Instant::now();

        let mut artifact = match builder_configuration.format() {
            OutputFormat::Html => {
                let mut assembler_configuration = HtmlAssemblerConfiguration::from(dossier.configuration().clone());

                if let Some(t) = builder_configuration.theme().as_ref() {
                
                    assembler_configuration.set_theme(t.clone());
                }
        
                if let Some(there_is_preview) = builder_configuration.preview() {
                    if there_is_preview {
                        if let Some(watch_mode) = builder_configuration.watching() {
                            if watch_mode {
        
                                assembler_configuration.external_scripts_mut()
                                                            .push(include_str!("preview/check_preview_updates.js").to_string())
        
                            }
                        }
                    }
                }

                HtmlAssembler::assemble_dossier(&dossier, &assembler_configuration)?
            },
        };

        log::info!("end to assembly (assembly time {} ms)", assembly_time.elapsed().as_millis());

        let mut output_location = compilation_configuration.output_location().clone();

        if output_location.is_dir() {
            output_location = output_location.join(file_utility::build_output_file_name(
               &dossier.name(),
                Some(&builder_configuration.format().get_extension())
            ));
        }

        let dump_configuration = DumpConfiguration::new(
                                                        output_location,
                                                        builder_configuration.force_output().unwrap_or(false)
                                                    );

        artifact.dump(&dump_configuration)?;

        Ok(())
    }

    /// Watch filesystem and compile dossier if any changes occur
    /// 
    /// - min_elapsed_time_between_events_in_secs is the minimum time interval between two compilation
    pub async fn watch_compile_dossier(mut builder_configuration: BuilderConfiguration, min_elapsed_time_between_events_in_secs: u64, preview: Option<Arc<TokioRwLock<HtmlPreview>>>) -> Result<(), BuilderError> {

        let input_location_abs = Arc::new(builder_configuration.input_location().canonicalize().unwrap()); 

        let dossier = Self::load_dossier(&builder_configuration).await?;

        builder_configuration.merge_dossier_configuration(dossier.configuration());

        let dossier = Arc::new(TokioRwLock::new(dossier));

        let builder_configuration = Arc::new(TokioRwLock::new(builder_configuration.clone()));

        let mut watcher = tokio::spawn(async move {

            NmdWatcher::new(
                min_elapsed_time_between_events_in_secs,
                &input_location_abs.clone(),
                Box::new({
    
                    let preview = preview.clone();
                    let builder_configuration: Arc<TokioRwLock<BuilderConfiguration>> = Arc::clone(&builder_configuration);
                    let dossier = dossier.clone();
                    let input_location_abs = input_location_abs.clone();
    
                    move || {
    
                        let builder_configuration = Arc::clone(&builder_configuration);
    
                        let preview = preview.clone();
    
                        let dossier = dossier.clone();
    
                        Box::pin({
    
                            let input_location_abs = input_location_abs.clone();
                            async move {
    
                                let compilation_result = tokio::spawn(async move {
                                    Self::build_dossier(&mut (*dossier.write().await), &builder_configuration.read().await.clone()).await
                                });
    
                                match compilation_result.await {
                                    Ok(_) => {
                
                                        log::info!("compilation OK");
    
                                        println!("\n\n");
                                        log::info!("watch mode ON: modification to the dossier files will cause recompilation");
                                        log::info!("start watching: {:?}", input_location_abs);
                                        log::info!("press CTRL + C to terminate");
                                        println!("\n\n");
                                        
                                        if let Some(preview) = preview {
    
                                            tokio::spawn(async move {
                                                preview.write().await.render().await
                                            }).await??;
                                        }
                
                                        return Ok(())
                                    },
                                    Err(err) => {
                                        log::error!("error during compilation: {:?}", err);
                
                                        return Err(WatcherError::ElaborationError(err.to_string()))
                                    }
                                }
                            }
                        })
                    }
                }),
                Box::new({
    
                    let input_location_abs = input_location_abs.clone();
    
                    move |event| {
    
                        let input_location_abs = input_location_abs.clone();
        
                        Box::pin(async move {
        
                            if event.paths.contains(&input_location_abs.join(DOSSIER_CONFIGURATION_YAML_FILE_NAME)) ||
                                event.paths.contains(&input_location_abs.join(DOSSIER_CONFIGURATION_JSON_FILE_NAME)) {
            
                                log::info!("recompilation needed");
                                return Ok(true)
                            }
            
                            Ok(false)
                        })
                    }
                }),
                Box::new({
    
                    let builder_configuration = Arc::clone(&builder_configuration);
        
                    let input_location_abs = input_location_abs.clone();
    
                    move |event| {
    
                        let builder_configuration = Arc::clone(&builder_configuration);
        
                        let input_location_abs = input_location_abs.clone();
        
                        Box::pin(async move {
        
                            let original_log_max_level = log::max_level();
        
                            log::set_max_level(log::LevelFilter::Warn);
        
                            let dc = DossierConfiguration::try_from(builder_configuration.read().await.input_location());
        
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
                
                                let path = path.strip_prefix(&*input_location_abs.clone());
        
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
                    }
                }),
                Box::new({
                    let builder_configuration = Arc::clone(&builder_configuration);
    
                    // let preview = Arc::clone(&preview);
                    let preview = preview.clone();
    
                    move |paths| {
                        Box::pin({
        
                            let builder_configuration = Arc::clone(&builder_configuration);
                            let preview = preview.clone();
                            let dossier = dossier.clone();
        
                            async move {
    
                                let documents_to_parse: Option<HashSet<String>>;        // None => all documents
    
                                // check if nmd.yml or nmd.json is changed => load whole dossier
                                if paths.iter()
                                        .map(|p| p.file_name())
                                        .filter(|f| f.is_some())
                                        .map(|f| f.unwrap().to_string_lossy().to_string())
                                        .find(|f| f.eq(DOSSIER_CONFIGURATION_YAML_FILE_NAME))
                                        .is_some() {
    
                                    documents_to_parse = None;
    
                                    match Self::load_dossier(&*builder_configuration.clone().read().await).await {
                                        Ok(d) => {

                                            builder_configuration.write().await.merge_dossier_configuration(d.configuration());

                                            *dossier.write().await = d;
                                        },
                                        Err(err) => return Err(WatcherError::ElaborationError(err.to_string())),
                                    }
    
                                } else {        // load dossier partially
                                    let codex = Arc::new(builder_configuration.read().await.codex());
    
                                    let mut dtp: HashSet<String> = HashSet::new();
    
                                    let mut document_read_handles = JoinSet::new();
                                    for path in &paths {
    
                                        if dossier.read().await.configuration().raw_documents_paths().par_iter().find_any(|raw_path| {
                                            let document_path = PathBuf::from(raw_path);
    
                                            if let Some(document_name) = document_path.file_name() {
    
                                                if let Some(file_name) = path.file_name() {
                                                    return document_name.eq(file_name);
                                                }
                                            }
    
                                            false
                                        }).is_some() {
    
                                            let path = path.clone();
                                            let codex = codex.clone();
        
                                            document_read_handles.spawn(async move {
        
                                                let document = Loader::load_document_from_path(&path, &codex, &LoadConfiguration::default(), LoadConfigurationOverLay::default());
    
                                                document
                                            });
                                        }
                                    }
    
                                    while let Some(document_read_res) = document_read_handles.join_next().await {
                                        if let Ok(document) = document_read_res? {
    
                                            let name = document.name().clone();
    
                                            let res = dossier.write().await.replace_document(&name, document);
    
                                            dtp.insert(name);
    
                                            res
                                        }
                                    }
    
                                    documents_to_parse = Some(dtp);
                                }
    
                                let build_result = tokio::spawn(async move {
    
                                    Self::build_dossier_compiling_subset(&mut *dossier.write().await, builder_configuration.read().await.borrow(), documents_to_parse).await
                                });
                
                                let preview = preview.clone();
    
                                match build_result.await {
                                    Ok(_) => {
                
                                        log::info!("compilation OK");
                                        
                                        if let Some(preview) = preview {
    
                                            tokio::spawn(async move {
                                                preview.write().await.update().await
                                            }).await??;
                                        }
                
                                        println!("\n\n");
                                        log::info!("preview is available on {}", PREVIEW_URL);
                                        println!("\n\n");
    
                                        return Ok(())
                                    },
                                    Err(err) => {
                                        log::error!("error during compilation: {:?}", err);
                
                                        return Err(WatcherError::ElaborationError(err.to_string()))
                                    }
                                }
                            }
                        })
                    }
                }),
            ).await
        }).await??;

        let watcher_join_handle = tokio::spawn(async move {
        
            watcher.start().await

        });

        watcher_join_handle.await??;

        log::info!("stop watching...");

        Ok(())
        
    }


    /// Load document
    pub async fn load_document(builder_configuration: &BuilderConfiguration) -> Result<Document, BuilderError> {

        log::info!("start to load dossier");

        let build_start = Instant::now();

        let codex = builder_configuration.codex();

        let mut document: Document = Loader::load_document_from_path(builder_configuration.input_location(), &codex, &LoadConfiguration::default(), LoadConfigurationOverLay::default())?;

        if let Some(with_nuid) = builder_configuration.nuid() {
            if with_nuid {
                log::info!("assign nuid...");
                assign_nuid_to_document_paragraphs(&mut document);
            }
        }

        log::info!("document loaded in {} ms", build_start.elapsed().as_millis());

        Ok(document)
    }

    /// Standard file compilation based on `BuilderConfiguration`
    /// It loads, compiles and dumps a document
    pub async fn build_document(builder_configuration: &BuilderConfiguration) -> Result<(), BuilderError> {

        log::info!("start to build document");

        let build_start = Instant::now();

        let mut document = Self::load_document(builder_configuration).await?;        

        let compilation_configuration = builder_configuration.generate_compilation_configuration();

        if compilation_configuration.compress_embed_image() || compilation_configuration.embed_local_image() || compilation_configuration.embed_remote_image() {

            log::warn!("embedding or compressing images is a time consuming task! Consider not using this feature unless strictly necessary");
        }

        log::info!("will use dossier configuration: {:?}", builder_configuration.input_location());
        
        log::info!("parsing using theme: {}", compilation_configuration.theme());
        log::debug!("parsing configuration:\n{:#?}\n", compilation_configuration);
        
        if compilation_configuration.fast_draft() {
            log::info!("fast draft mode on!")
        }

        Compiler::compile_document(&mut document, builder_configuration.format(), &builder_configuration.codex(), &compilation_configuration, CompilationConfigurationOverLay::default())?;

        log::info!("document compiled in {} ms", build_start.elapsed().as_millis());

        log::info!("assembling...");

        let output_location = builder_configuration.output_location().clone();

        let assembly_time = Instant::now();

        let mut artifact = match builder_configuration.format() {
            OutputFormat::Html => {

                let mut assembler_configuration = HtmlAssemblerConfiguration::default();

                assembler_configuration.set_theme(builder_configuration.theme().clone().unwrap_or(Theme::default()));

                if let Some(there_is_preview) = builder_configuration.preview() {
                    if there_is_preview {
                        assembler_configuration.external_styles_mut().push(include_str!("preview/check_preview_updates.js").to_string())
                    }
                }

                HtmlAssembler::assemble_document_standalone(&document, &output_location.file_stem().unwrap().to_string_lossy().to_string(), None, None, &assembler_configuration)?
            },
        };

        log::info!("end to assembly (assembly time {} ms)", assembly_time.elapsed().as_millis());

        let dump_configuration = DumpConfiguration::new(output_location, builder_configuration.force_output().unwrap_or(false));

        artifact.dump(&dump_configuration)?;

        log::info!("document build in {} ms", build_start.elapsed().as_millis());

        Ok(())
    }
}

#[cfg(test)]
mod test {
}