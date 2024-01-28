use std::str::FromStr;


#[derive(Debug, Clone)]
pub enum DossierMetadata {

}

#[derive(Debug, Clone)]
pub struct DossierConfiguration {
    name: String,
    documents: Vec<String>,
    styles: Vec<String>,
    metadata: Vec<DossierMetadata>,
}

impl DossierConfiguration {

    pub fn new(name: String, documents: Vec<String>, styles: Vec<String>, metadata: Vec<DossierMetadata>) -> Self {
        Self {
            name,
            documents,
            styles,
            metadata
        }
    }

    pub fn documents(&self) -> &Vec<String> {
        &self.documents
    }

    pub fn set_documents(&mut self, documents: Vec<String>) -> () {
        self.documents = documents
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Default for DossierConfiguration {
    fn default() -> Self {
        Self {
            name: String::from("New Dossier"),
            documents: vec![],          // TODO: all .nmd file in running directory
            styles: vec![],              // TODO: default style
            metadata: vec![],
        }
    }
}

impl FromStr for DossierConfiguration {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}