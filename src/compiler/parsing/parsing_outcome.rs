
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

    pub fn new_mutable(content: String) -> Self {
        Self::new(vec![ParsingOutcomePart::Mutable { content }])
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

    pub fn parts_mut(&mut self) -> &mut Vec<ParsingOutcomePart> {
        &mut self.parts
    }

    pub fn apply_parsing_function_to_mutable_parts<F, E>(&mut self, f: F) -> Result<(), E>
        where F: Fn(&ParsingOutcomePart) -> Result<ParsingOutcome, E> {

            let mut new_parts: Vec<ParsingOutcomePart> = Vec::new();
            for part in &self.parts {
                match part {
                    ParsingOutcomePart::Fixed { content: _ } => new_parts.push(part.clone()),
                    ParsingOutcomePart::Mutable { content: _ } => {
                        let outcome = f(part)?;

                        Into::<Vec<ParsingOutcomePart>>::into(outcome).into_iter().for_each(|p| new_parts.push(p))
                    },
                }
            }

            Ok(())
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