pub mod list_bullet_configuration_record;
pub mod parsing_configuration_overlay;

use std::{collections::HashMap, ops::Add, path::PathBuf};

use getset::{CopyGetters, Getters, MutGetters, Setters};

use crate::{compiler::{bibliography::Bibliography, compilation_configuration::CompilationConfiguration}, resource::text_reference::TextReferenceMap};

use self::list_bullet_configuration_record::ListBulletConfigurationRecord;

use crate::compiler::codex::modifier::modifiers_bucket::ModifiersBucket;

use super::parsing_metadata::ParsingMetadata;


/// Struct which contains all information about possible parsing options 
#[derive(Debug, Getters, CopyGetters, MutGetters, Setters, Clone)]
pub struct ParsingConfiguration {

    #[getset(get = "pub", set = "pub")]
    input_location: PathBuf,

    #[getset(get = "pub", set = "pub")]
    output_location: PathBuf,

    #[getset(get_copy = "pub", set = "pub")]
    embed_local_image: bool,

    #[getset(get_copy = "pub", set = "pub")]
    embed_remote_image: bool,
    
    #[getset(get_copy = "pub", set = "pub")]
    compress_embed_image: bool,

    #[getset(get_copy = "pub", set = "pub")]
    strict_image_src_check: bool,

    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    metadata: ParsingMetadata,

    #[getset(get = "pub", set = "pub")]
    excluded_modifiers: ModifiersBucket,

    #[getset(get_copy = "pub", set = "pub")]
    parallelization: bool,

    #[getset(get = "pub", set = "pub")]
    list_bullets_configuration: Vec<ListBulletConfigurationRecord>,
    
    #[getset(get_copy = "pub", set = "pub")]
    strict_list_check: bool,

    #[getset(get_copy = "pub", set = "pub")]
    strict_focus_block_check: bool,

    #[getset(get = "pub", set = "pub")]
    references: TextReferenceMap,

    #[getset(get_copy = "pub", set = "pub")]
    fast_draft: bool,

    #[getset(get = "pub", set = "pub")]
    bibliography: Option<Bibliography>,
}

impl ParsingConfiguration {

    pub fn new(input_location: PathBuf, output_location: PathBuf, embed_local_image: bool, embed_remote_image: bool, 
                compress_embed_image: bool, strict_image_src_check: bool, metadata: ParsingMetadata, excluded_modifiers: ModifiersBucket, 
                parallelization: bool, list_bullets_configuration: Vec<ListBulletConfigurationRecord>, strict_list_check: bool, 
                strict_focus_block_check: bool, references: TextReferenceMap, fast_draft: bool, bibliography: Option<Bibliography>,) -> Self {

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
            list_bullets_configuration,
            strict_list_check,
            strict_focus_block_check,
            references,
            fast_draft,
            bibliography
        }
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
            metadata: ParsingMetadata::default(),
            excluded_modifiers: ModifiersBucket::None,
            parallelization: false,
            list_bullets_configuration: list_bullet_configuration_record::default_bullets_configuration(),
            strict_list_check: false,
            strict_focus_block_check: false,
            references: HashMap::new(),
            fast_draft: false,
            bibliography: None,
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
            references: compilation_configuration.references().clone().unwrap(),
            fast_draft: compilation_configuration.fast_draft(),
            bibliography: compilation_configuration.bibliography().clone(),
            
            ..Default::default()
        }
    }
}