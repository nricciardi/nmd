use std::{collections::HashSet, path::PathBuf};

use getset::{CopyGetters, Getters, MutGetters, Setters};

use crate::resource::text_reference::{TextReferenceMap};

use super::{bibliography::Bibliography, codex::{codex_configuration::CodexConfiguration, Codex}, dossier::dossier_configuration::DossierConfiguration, output_format::OutputFormat, parsing::parsing_configuration::ParsingConfiguration, theme::Theme};



/// Struct which contains all information about possible compilation options. It is used to wrap specific user requests for compilation 
#[derive(Debug, Getters, CopyGetters, MutGetters, Setters, Clone)]
pub struct CompilationConfiguration {

    #[getset(get = "pub", set = "pub")]
    format: OutputFormat,

    #[getset(get = "pub", set = "pub")]
    input_location: PathBuf,

    #[getset(get = "pub", set = "pub")]
    output_location: PathBuf,

    #[getset(get_copy = "pub", set = "pub")]
    fast_draft: bool,

    #[getset(get = "pub", set = "pub")]
    embed_local_image: Option<bool>,

    #[getset(get = "pub", set = "pub")]
    embed_remote_image: Option<bool>,

    #[getset(get = "pub", set = "pub")]
    compress_embed_image: Option<bool>,

    #[getset(get = "pub", set = "pub")]
    strict_image_src_check: Option<bool>,

    #[getset(get = "pub", set = "pub")]
    parallelization: Option<bool>,

    #[getset(get = "pub", set = "pub")]
    use_remote_addons: Option<bool>,

    #[getset(get = "pub", set = "pub")]
    references: Option<TextReferenceMap>,

    #[getset(get = "pub", set = "pub")]
    documents_subset_to_compile: Option<HashSet<String>>,
    
    #[getset(get = "pub", set = "pub")]
    bibliography: Option<Bibliography>,

    #[getset(get = "pub", set = "pub")]
    theme: Option<Theme>,
}

impl CompilationConfiguration {
    pub fn new(format: OutputFormat, input_location: PathBuf, output_location: PathBuf) -> Self {
        CompilationConfiguration {
            format,
            input_location,
            output_location,

            ..Default::default()
        }
    }

    pub fn codex(&self) -> Codex {
        Codex::from(&self.format, CodexConfiguration::default())
    }

    pub fn parsing_configuration(&self) -> ParsingConfiguration {
        ParsingConfiguration::from(self.clone())
    }
}

impl CompilationConfiguration {
    pub fn merge_dossier_configuration(&mut self, dossier_configuration: &DossierConfiguration) {

        if self.embed_local_image().is_none() {
            self.set_embed_local_image(Some(dossier_configuration.compilation().embed_local_image().clone()));
        }

        if self.embed_remote_image().is_none() {
            self.set_embed_remote_image(Some(dossier_configuration.compilation().embed_remote_image().clone()));
        }

        if self.compress_embed_image().is_none() {
            self.set_compress_embed_image(Some(dossier_configuration.compilation().compress_embed_image().clone()));
        }

        if self.use_remote_addons().is_none() {
            self.set_use_remote_addons(Some(dossier_configuration.compilation().use_remote_addons().clone()));
        }

        if self.parallelization().is_none() {
            self.set_parallelization(Some(dossier_configuration.compilation().parallelization().clone()));
        }

        if self.strict_image_src_check().is_none() {
            self.set_strict_image_src_check(Some(dossier_configuration.compilation().strict_image_src_check().clone()));
        }

        if self.references().is_none() {
            self.set_references(Some(dossier_configuration.references().clone()));
        }

        if self.bibliography().is_none() {
            self.set_bibliography(Some(Bibliography::from(dossier_configuration.bibliography())));
        }

        if self.theme().is_none() {
            self.set_theme(Some(dossier_configuration.style().theme().clone()));
        }
    }

    pub fn fill_with_default(&mut self) {
        if self.embed_local_image().is_none() {
            self.set_embed_local_image(Some(true));
        }

        if self.embed_remote_image().is_none() {
            self.set_embed_remote_image(Some(false));
        }

        if self.compress_embed_image().is_none() {
            self.set_compress_embed_image(Some(false));
        }

        if self.use_remote_addons().is_none() {
            self.set_use_remote_addons(Some(false));
        }

        if self.parallelization().is_none() {
            self.set_parallelization(Some(true));
        }

        if self.strict_image_src_check().is_none() {
            self.set_strict_image_src_check(Some(true));
        }

        if self.references().is_none() {
            self.set_references(Some(TextReferenceMap::default()));
        }

        if self.bibliography().is_none() {
            self.set_bibliography(None);
        }

        if self.theme().is_none() {
            self.set_theme(Some(Theme::default()));
        }
    }
}

impl Default for CompilationConfiguration {
    fn default() -> Self {
        Self {
            format: Default::default(),
            input_location: PathBuf::from("."),
            output_location: PathBuf::from("."),
            fast_draft: false,
            embed_local_image: None,
            embed_remote_image: None,
            compress_embed_image: None,
            strict_image_src_check: None,
            parallelization: None,
            use_remote_addons: None,
            references: None,
            documents_subset_to_compile: None,
            bibliography: None,
            theme: None,
        }
    }
}