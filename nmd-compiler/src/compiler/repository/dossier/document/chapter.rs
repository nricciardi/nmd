pub mod paragraph;

pub use self::paragraph::Paragraph;

pub struct Chapter {
    title: String,
    paragraphs: Vec<Paragraph>,
    subchapters: Vec<Box<Chapter>>
}

