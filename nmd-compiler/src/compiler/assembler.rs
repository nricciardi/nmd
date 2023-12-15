use thiserror::Error;

use self::html_assembler::HtmlAssembler;

use super::{supported_format::SupportedFormat, dossier::Dossier};

pub mod html_assembler;



#[derive(Error, Debug)]
pub enum AssemblerError {

}

pub trait Assembler {
    fn assemble(&self, dossier: Dossier) -> Result<(), AssemblerError>;
}

pub fn from(format: SupportedFormat) -> Box<dyn Assembler> {
    match format {
        SupportedFormat::Html => Box::new(HtmlAssembler::new())  
    }
}