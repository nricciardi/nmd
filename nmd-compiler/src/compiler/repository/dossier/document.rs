pub mod chapter;

pub use chapter::Chapter;

pub struct Document {
    name: String,
    chapters: Vec<Chapter> 
}