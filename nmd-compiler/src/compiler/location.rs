use std::str::FromStr;

#[derive(Debug)]
pub enum Location {
    Url(String),
    Path(String)
}

impl Location {
    pub fn to_string(&self) -> &String {

        match self {
            Self::Url(url) => url,
            Self::Path(path) => path
        }
    }
}

pub trait Locatable{

    fn location(self: &Self) -> &Location;
}

// pub fn wrap_locatable(locatable: dyn Locatable) -> Box<>