use crate::compiler::dossier::Dossier;

use super::Assembler;

pub struct HtmlAssembler {
}

impl HtmlAssembler {
    pub fn new() -> Self {
        Self {     
        }
    }
}

impl Assembler for HtmlAssembler {
    fn assemble(&self, dossier: Dossier) -> Result<(), super::AssemblerError> {
        todo!()
    }
}