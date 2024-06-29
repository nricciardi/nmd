
#[derive(Debug, Clone)]
pub enum ParsingOutcomePart {
    Fixed{ content: String },
    Mutable{ content: String },
}

impl ParsingOutcomePart {
    pub fn content(&self) -> &String {
        match self {
            ParsingOutcomePart::Fixed { content } => content,
            ParsingOutcomePart::Mutable { content } => content,
        }
    }
}



#[derive(Debug, Clone)]
pub struct ParsingOutcome {
    parts: Vec<ParsingOutcomePart>
}

impl ParsingOutcome {
    pub fn new(parts: Vec<ParsingOutcomePart>) -> Self {
        Self {
            parts
        }
    }

    pub fn new_empty() -> Self {
        Self {
            parts: Vec::new(),
        }
    }

    pub fn new_fixed(content: String) -> Self {
        Self::new(vec![ParsingOutcomePart::Fixed { content }])
    }

    pub fn parsed_content(&self) -> String {
        let mut parsed_content = String::new();

        for part in &self.parts {
            match part {
                ParsingOutcomePart::Fixed { content } => parsed_content.push_str(content),
                ParsingOutcomePart::Mutable { content } => parsed_content.push_str(content),
            }
        }

        parsed_content
    }

    pub fn add_fixed_part(&mut self, content: String) {
        self.parts.push(ParsingOutcomePart::Fixed{ content });
    }

    pub fn add_mutable_part(&mut self, content: String) {
        self.parts.push(ParsingOutcomePart::Mutable{ content });
    }

    pub fn parts(&self) -> &Vec<ParsingOutcomePart> {
        &self.parts
    }
}

impl Into<String> for ParsingOutcome {
    fn into(self) -> String {
        self.parsed_content()
    }
}

impl Into<Vec<ParsingOutcomePart>> for ParsingOutcome {
    fn into(self) -> Vec<ParsingOutcomePart> {
        self.parts
    }
}