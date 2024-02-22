use serde::{Deserialize, Serialize};


fn yes() -> bool {
    true
}

fn no() -> bool {
    false
}


#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DossierConfigurationCompilation {
    #[serde(default = "yes")]
    embed_local_image: bool,

    #[serde(default = "yes")]
    embed_remote_image: bool,
    
    #[serde(default = "yes")]
    compress_embed_image: bool,
    
    #[serde(default = "yes")]
    strict_image_src_check: bool,
    
    // excluded_modifiers: Modifiers,       // TODO
    #[serde(default = "yes")]
    parallelization: bool,
    
    #[serde(default = "no")]
    use_remote_addons: bool,

    #[serde(default = "no")]
    strict_list_check: bool,
}

impl DossierConfigurationCompilation {

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

    pub fn strict_list_check(&self) -> bool {
        self.strict_list_check
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

    pub fn set_strict_list_check(&mut self, strict_list_check: bool) {
        self.strict_list_check = strict_list_check;
    }
}

impl Default for DossierConfigurationCompilation {
    fn default() -> Self {
        Self {
            embed_local_image: true,
            embed_remote_image: true,
            compress_embed_image: true,
            strict_image_src_check: true,
            parallelization: true,
            use_remote_addons: false,
            strict_list_check: false
        }
    }
}