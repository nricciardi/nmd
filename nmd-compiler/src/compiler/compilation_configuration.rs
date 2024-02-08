use std::{clone, path::PathBuf};

use crate::compiler::{parsable::{codex::{Codex, codex_configuration::CodexConfiguration}, ParsingConfiguration}, output_format::OutputFormat};

use super::dossier::dossier_configuration::{self, DossierConfiguration};


#[derive(Clone)]
pub struct CompilationConfiguration {
    format: OutputFormat,
    input_location: PathBuf,
    output_location: PathBuf,

    embed_local_image: bool,
    embed_remote_image: bool,
    compress_embed_image: bool,
    strict_image_src_check: bool,
    // excluded_modifiers: Modifiers,       // TODO
    parallelization: bool,
    use_remote_addons: bool
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

    pub fn embed_local_image(&self) -> bool {
        self.embed_local_image
    }

    pub fn embed_remote_image(&self) -> bool {
        self.embed_remote_image
    }

    pub fn compress_embed_image(&self) -> bool {
        self.compress_embed_image
    }

    pub fn parallelization(&self) -> bool {
        self.parallelization
    }

    pub fn use_remote_addons(&self) -> bool {
        self.use_remote_addons
    }

    pub fn strict_image_src_check(&self) -> bool {
        self.strict_image_src_check
    }

    pub fn set_embed_local_image(&mut self, value: bool) {
        self.embed_local_image = value;
    }

    pub fn set_embed_remote_image(&mut self, value: bool) {
        self.embed_remote_image = value;
    }

    pub fn set_compress_embed_image(&mut self, value: bool) {
        self.compress_embed_image = value;
    }

    pub fn set_strict_image_src_check(&mut self, value: bool) {
        self.strict_image_src_check = value;
    }

    pub fn set_parallelization(&mut self, value: bool) {
        self.parallelization = value;
    }

    pub fn set_use_remote_addons(&mut self, value: bool) {
        self.use_remote_addons = value;
    }
}

impl CompilationConfiguration {
    pub fn merge_dossier_configuration(&mut self, dossier_configuration: &DossierConfiguration) {
        self.set_embed_local_image(dossier_configuration.compilation().embed_local_image().clone());
        self.set_embed_remote_image(dossier_configuration.compilation().embed_remote_image().clone());
        self.set_compress_embed_image(dossier_configuration.compilation().compress_embed_image().clone());
        self.set_use_remote_addons(dossier_configuration.compilation().use_remote_addons().clone());
        self.set_parallelization(dossier_configuration.compilation().parallelization().clone());
        self.set_strict_image_src_check(dossier_configuration.compilation().strict_image_src_check().clone());
    }
}

impl Default for CompilationConfiguration {
    fn default() -> Self {
        Self {
            format: Default::default(),
            input_location: PathBuf::from("."),
            output_location: PathBuf::from("."),
            embed_local_image: true,
            embed_remote_image: true,
            compress_embed_image: true,
            strict_image_src_check: true,
            parallelization: true,
            use_remote_addons: false
        }
    }
}