use std::{clone, path::PathBuf};

use super::{codex::Codex, dossier::dossier_configuration::DossierConfiguration, output_format::OutputFormat, parser::parsing_rule::parsing_configuration::ParsingConfiguration};



#[derive(Clone, Debug)]
pub struct CompilationConfiguration {
    format: OutputFormat,
    input_location: PathBuf,
    output_location: PathBuf,

    embed_local_image: Option<bool>,
    embed_remote_image: Option<bool>,
    compress_embed_image: Option<bool>,
    strict_image_src_check: Option<bool>,
    // excluded_modifiers: Modifiers,       // TODO
    parallelization: Option<bool>,
    use_remote_addons: Option<bool>
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

    pub fn format(&self) -> &OutputFormat {
        &self.format
    }

    pub fn codex(&self) -> Codex {
        Codex::from(&self.format, CodexConfiguration::default())
    }

    pub fn parsing_configuration(&self) -> ParsingConfiguration {
        ParsingConfiguration::from(self.clone())
    }

    pub fn input_location(&self) -> &PathBuf {
        &self.input_location
    }

    pub fn output_location(&self) -> &PathBuf {
        &self.output_location
    }

    pub fn set_format(&mut self, format: OutputFormat) {
        self.format = format
    }

    pub fn set_input_location(&mut self, path: PathBuf) {
        self.input_location = path
    }

    pub fn set_output_location(&mut self, path: PathBuf) {
        self.output_location = path
    }

    pub fn embed_local_image(&self) -> Option<bool> {
        self.embed_local_image
    }

    pub fn embed_remote_image(&self) -> Option<bool> {
        self.embed_remote_image
    }

    pub fn compress_embed_image(&self) -> Option<bool> {
        self.compress_embed_image
    }

    pub fn parallelization(&self) -> Option<bool> {
        self.parallelization
    }

    pub fn use_remote_addons(&self) -> Option<bool> {
        self.use_remote_addons
    }

    pub fn strict_image_src_check(&self) -> Option<bool> {
        self.strict_image_src_check
    }
    pub fn set_embed_local_image(&mut self, value: bool) {
        self.embed_local_image = Some(value);
    }

    pub fn set_embed_remote_image(&mut self, value: bool) {
        self.embed_remote_image = Some(value);
    }

    pub fn set_compress_embed_image(&mut self, value: bool) {
        self.compress_embed_image = Some(value);
    }

    pub fn set_strict_image_src_check(&mut self, value: bool) {
        self.strict_image_src_check = Some(value);
    }

    pub fn set_parallelization(&mut self, value: bool) {
        self.parallelization = Some(value);
    }

    pub fn set_use_remote_addons(&mut self, value: bool) {
        self.use_remote_addons = Some(value);
    }
}

impl CompilationConfiguration {
    pub fn merge_dossier_configuration(&mut self, dossier_configuration: &DossierConfiguration) {

        if self.embed_local_image().is_none() {
            self.set_embed_local_image(dossier_configuration.compilation().embed_local_image().clone());
        }

        if self.embed_remote_image().is_none() {
            self.set_embed_remote_image(dossier_configuration.compilation().embed_remote_image().clone());
        }

        if self.compress_embed_image().is_none() {
            self.set_compress_embed_image(dossier_configuration.compilation().compress_embed_image().clone());
        }

        if self.use_remote_addons().is_none() {
            self.set_use_remote_addons(dossier_configuration.compilation().use_remote_addons().clone());
        }

        if self.parallelization().is_none() {
            self.set_parallelization(dossier_configuration.compilation().parallelization().clone());
        }

        if self.strict_image_src_check().is_none() {
            self.set_strict_image_src_check(dossier_configuration.compilation().strict_image_src_check().clone());
        }
    }
}

impl Default for CompilationConfiguration {
    fn default() -> Self {
        Self {
            format: Default::default(),
            input_location: PathBuf::from("."),
            output_location: PathBuf::from("."),
            embed_local_image: None,
            embed_remote_image: None,
            compress_embed_image: None,
            strict_image_src_check: None,
            parallelization: None,
            use_remote_addons: None
        }
    }
}