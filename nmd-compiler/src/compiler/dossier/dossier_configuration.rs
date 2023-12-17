use crate::compiler::resource::Resource;


#[derive(Debug, Clone)]
pub enum Metadata {

}

#[derive(Debug, Clone)]
pub struct DossierConfiguration {
    name: String,
    documents: Vec<Resource>,
    styles: Vec<Resource>,
    metadata: Vec<Metadata>,
}

impl DossierConfiguration {
    pub fn documents(&self) -> &Vec<Resource> {
        &self.documents
    }

    pub fn set_documents(&mut self, documents: Vec<Resource>) -> () {
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