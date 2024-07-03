
#[derive(Debug, Clone)]
pub struct ContentTree {
    title: String,
    sub_contents: Vec<ContentTree>,
}

impl ContentTree {
    pub fn new(title: String, sub_contents: Vec<ContentTree>) -> Self {
        Self {
            title,
            sub_contents
        }
    }
}