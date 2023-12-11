use crate::compiler::resource::Resource;



pub enum Metadata {

}

pub struct DossierConfiguration {
    documents: Vec<Resource>,
    styles: Vec<Resource>,
    metadata: Vec<Metadata>,
}


impl Default for DossierConfiguration {
    fn default() -> Self {
        Self {
            documents: vec![],          // TODO: all .nmd file in running directory
            styles: vec![],              // TODO: default style
            metadata: vec![],
        }
    }
}