pub mod paragraph;

pub use self::paragraph::Paragraph;

pub struct Chapter {
    title: String,
    paragraphs: Option<Vec<Paragraph>>,
    subchapters: Option<Vec<Box<Chapter>>>
}

