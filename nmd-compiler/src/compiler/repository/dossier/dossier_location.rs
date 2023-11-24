use crate::compiler::location::{Locatable, Location};


pub struct DossierLocation {
    location: Location
}

impl Locatable for DossierLocation {
    fn location(self: &Self) -> &Location {
        &self.location
    }
}