

pub struct ParsingConfigurationOverLay {
    
    additional_style: Option<String>
}

impl ParsingConfigurationOverLay {
    
    pub fn additional_style(&self) -> &Option<String> {
        &self.additional_style
    }

    pub fn set_additional_style(&mut self, style: Option<String>) {
        self.additional_style = style
    }
}