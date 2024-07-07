use getset::{Getters, Setters};



#[derive(Debug, Getters, Setters)]
pub struct ParsingConfigurationOverLay {
    
    #[getset(get = "pub", set = "pub")]
    additional_style: Option<String>
}