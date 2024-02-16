pub mod list_bullet_configuration_record;

use std::{ops::Add, path::PathBuf};

use crate::compiler::compilation_configuration::CompilationConfiguration;

use self::list_bullet_configuration_record::ListBulletConfigurationRecord;

use super::codex::modifier::Modifiers;


#[derive(Clone, Default, Debug)]
pub struct ParsingConfigurationMetadata {}

#[derive(Clone, Debug)]
pub struct ParsingConfiguration {

    input_location: PathBuf,
    output_location: PathBuf,

    embed_local_image: bool,
    embed_remote_image: bool,
    compress_embed_image: bool,
    strict_image_src_check: bool,

    metadata: ParsingConfigurationMetadata,

    excluded_modifiers: Modifiers,

    parallelization: bool,

    list_bullets_configuration: Vec<ListBulletConfigurationRecord>
}

impl ParsingConfiguration {

    pub fn new(input_location: PathBuf, output_location: PathBuf, embed_local_image: bool, embed_remote_image: bool, compress_embed_image: bool, strict_image_src_check: bool, metadata: ParsingConfigurationMetadata, excluded_modifiers: Modifiers, parallelization: bool, list_bullets_configuration: Vec<ListBulletConfigurationRecord>) -> Self {
        Self {
            input_location,
            output_location,
            embed_local_image,
            embed_remote_image,
            compress_embed_image,
            strict_image_src_check,
            metadata,
            excluded_modifiers,
            parallelization,
            list_bullets_configuration
        }
    }

    pub fn input_location(&self) -> &PathBuf {
        &self.input_location
    }

    pub fn output_location(&self) -> &PathBuf {
        &self.output_location
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

    pub fn strict_image_src_check(&self) -> bool {
        self.strict_image_src_check
    }

    pub fn metadata(&self) -> &ParsingConfigurationMetadata {
        &self.metadata
    }

    pub fn modifiers_excluded(&self) -> &Modifiers {
        &self.excluded_modifiers
    }

    pub fn parallelization(&self) -> bool {
        self.parallelization
    }

    pub fn list_bullets_configuration(&self) -> &Vec<ListBulletConfigurationRecord> {
        &self.list_bullets_configuration
    }

    pub fn set_input_location(&mut self, new_input_location: PathBuf) {
        self.input_location = new_input_location;
    }

    pub fn set_output_location(&mut self, new_output_location: PathBuf) {
        self.output_location = new_output_location;
    }

    pub fn set_embed_local_image(&mut self, new_embed_local_image: bool) {
        self.embed_local_image = new_embed_local_image;
    }

    pub fn set_embed_remote_image(&mut self, new_embed_remote_image: bool) {
        self.embed_remote_image = new_embed_remote_image;
    }

    pub fn set_compress_embed_image(&mut self, compress_embed_image: bool) {
        self.compress_embed_image = compress_embed_image;
    }

    pub fn set_strict_image_src_check(&mut self, new_strict_image_src_check: bool) {
        self.strict_image_src_check = new_strict_image_src_check;
    }

    pub fn set_metadata(&mut self, new_metadata: ParsingConfigurationMetadata) {
        self.metadata = new_metadata;
    }

    pub fn set_excluded_modifiers(&mut self, modifiers_excluded: Modifiers) {
        self.excluded_modifiers = modifiers_excluded
    }

    pub fn add_excluded_modifiers(&mut self, modifiers_excluded: Modifiers) {
        self.excluded_modifiers = self.excluded_modifiers.clone().add(modifiers_excluded)
    }

    pub fn set_parallelization(&mut self, value: bool) {
        self.parallelization = value
    }

    pub fn set_list_bullets_configuration(&mut self, value: Vec<ListBulletConfigurationRecord>) {
        self.list_bullets_configuration = value
    }
}

impl Default for ParsingConfiguration {
    fn default() -> Self {
        Self {
            input_location: PathBuf::from("."),
            output_location: PathBuf::from("."),
            embed_local_image: true,
            embed_remote_image: false,
            compress_embed_image: false,
            strict_image_src_check: false,
            metadata: ParsingConfigurationMetadata::default(),
            excluded_modifiers: Modifiers::None,
            parallelization: false,
            list_bullets_configuration: list_bullet_configuration_record::default_bullets_configuration()
        }
    }
}


impl From<CompilationConfiguration> for ParsingConfiguration {
    fn from(compilation_configuration: CompilationConfiguration) -> Self {
        Self {

            input_location: compilation_configuration.input_location().clone(),
            output_location: compilation_configuration.output_location().clone(),
            embed_local_image: compilation_configuration.embed_local_image().clone().unwrap(),
            embed_remote_image: compilation_configuration.embed_remote_image().clone().unwrap(),
            compress_embed_image: compilation_configuration.compress_embed_image().clone().unwrap(),
            strict_image_src_check: compilation_configuration.strict_image_src_check().clone().unwrap(),
            parallelization: compilation_configuration.parallelization().clone().unwrap(),
            
            ..Default::default()
        }
    }
}