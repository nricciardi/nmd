

#[derive(Clone, Debug, Default)]
pub struct ParsingMetadata {
    dossier_name: Option<String>,
    document_name: Option<String>
}

impl ParsingMetadata {
    pub fn new() -> Self {
        Self {
            document_name: None,
            dossier_name: None
        }
    }

    pub fn dossier_name(&self) -> &Option<String> {
        &self.dossier_name
    }

    pub fn document_name(&self) -> &Option<String> {
        &self.document_name
    }

    pub fn set_dossier_name(&mut self, dossier_name: Option<String>) {
        self.dossier_name = dossier_name
    }

    pub fn set_document_name(&mut self, document_name: Option<String>) {
        self.document_name = document_name
    }
}