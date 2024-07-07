use getset::{Getters, Setters};



#[derive(Clone, Debug, Default, Getters, Setters)]
pub struct ParsingMetadata {

    #[getset(get = "pub", set = "pub")]
    dossier_name: Option<String>,

    #[getset(get = "pub", set = "pub")]
    document_name: Option<String>
}

impl ParsingMetadata {
    pub fn new() -> Self {
        Self {
            document_name: None,
            dossier_name: None
        }
    }
}