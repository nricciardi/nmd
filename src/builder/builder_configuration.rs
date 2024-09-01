use std::{collections::HashSet, path::PathBuf};
use getset::{CopyGetters, Getters, MutGetters, Setters};
use nmd_core::{bibliography::Bibliography, codex::Codex, compiler::compilation_configuration::{CompilableResourceType, CompilationConfiguration}, dossier::dossier_configuration::DossierConfiguration, output_format::OutputFormat, resource::text_reference::TextReferenceMap, theme::Theme};


/// Struct which contains all information about possible compilation options. It is used to wrap specific user requests for compilation 
#[derive(Debug, Getters, CopyGetters, MutGetters, Setters, Clone)]
pub struct BuilderConfiguration {

    #[getset(get = "pub", set = "pub")]
    format: OutputFormat,

    #[getset(get = "pub")]
    input_location: PathBuf,

    #[getset(get = "pub", set = "pub")]
    output_location: PathBuf,

    #[getset(get_copy = "pub", set = "pub")]
    force_output: Option<bool>,

    #[getset(get = "pub", set = "pub")]
    fast_draft: Option<bool>,

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

    #[getset(get = "pub", set = "pub")]
    styles_raw_path: Vec<String>,

    #[getset(get = "pub", set = "pub")]
    resource_type: CompilableResourceType,

    #[getset(get_copy = "pub", set = "pub")]
    preview: Option<bool>,

    #[getset(get_copy = "pub", set = "pub")]
    watching: Option<bool>,

    #[getset(get_copy = "pub", set = "pub")]
    nuid: Option<bool>,
}

impl BuilderConfiguration {
    pub fn new(input_location: PathBuf, output_location: PathBuf) -> Self {

        let mut builder_configuration = Self {
            output_location,

            ..Default::default()
        };

        builder_configuration.set_input_location(input_location);
        
        builder_configuration
    }

    /// Set input location and auto-set resource type
    pub fn set_input_location(&mut self, input_location: PathBuf) {
        if input_location.is_dir() {

            self.resource_type = CompilableResourceType::Dossier;
        
        } else {

            self.resource_type = CompilableResourceType::File;
        }

        self.input_location = input_location;
    }

    pub fn codex(&self) -> Codex {

        Codex::from(&self.format)
    }

    pub fn generate_compilation_configuration(&self) -> CompilationConfiguration {
        let mut compilation_configuration = CompilationConfiguration::default();
        
        compilation_configuration.set_input_location(self.input_location().clone());

        compilation_configuration.set_output_location(self.output_location().clone());

        if let Some(val) = self.embed_local_image() {

            compilation_configuration.set_embed_local_image(*val);
        }

        if let Some(val) = self.compress_embed_image() {

            compilation_configuration.set_compress_embed_image(*val);
        }

        if let Some(val) = self.embed_remote_image() {

            compilation_configuration.set_embed_remote_image(*val);
        }

        if let Some(val) = self.strict_image_src_check() {

            compilation_configuration.set_strict_image_src_check(*val);
        }

        if let Some(val) = self.embed_remote_image() {

            compilation_configuration.set_embed_remote_image(*val);
        }

        if let Some(val) = self.embed_remote_image() {

            compilation_configuration.set_embed_remote_image(*val);
        }

        if let Some(val) = self.parallelization() {

            compilation_configuration.set_parallelization(*val);
        }

        if let Some(val) = self.fast_draft() {

            compilation_configuration.set_fast_draft(*val);
        }

        if let Some(val) = self.references() {

            compilation_configuration.set_references(val.clone());
        }

        compilation_configuration.set_bibliography(self.bibliography().clone());

        if let Some(val) = self.theme() {

            compilation_configuration.set_theme(val.clone());
        }

        compilation_configuration.set_resource_type(self.resource_type().clone());

        compilation_configuration
    }
}

impl BuilderConfiguration {
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
            self.set_embed_local_image(Some(false));
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
            self.set_parallelization(Some(false));
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

impl Default for BuilderConfiguration {
    fn default() -> Self {
        Self {
            format: Default::default(),
            input_location: PathBuf::from("."),
            output_location: PathBuf::from("."),
            fast_draft: Some(false),
            force_output: Some(false),
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
            styles_raw_path: Vec::new(),
            resource_type: CompilableResourceType::default(),
            preview: Some(false),
            watching: Some(false),
            nuid: Some(false),
        }
    }
}