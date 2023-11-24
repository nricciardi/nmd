mod document;
mod dossier_location;

pub use document::Document;
pub use dossier_location::DossierLocation;

pub struct Dossier {
    name: String,
    documents: Vec<Document>
}