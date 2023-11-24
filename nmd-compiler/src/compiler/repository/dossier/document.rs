pub mod chapter;

pub use chapter::Chapter;

pub struct Document {
    name: String,
    chapters: Option<Vec<Chapter> >
}