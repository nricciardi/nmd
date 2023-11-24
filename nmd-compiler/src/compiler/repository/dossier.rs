mod document;

pub use document::Document;

pub struct Dossier {
    name: String,
    documents: Option<Vec<Document>>
}