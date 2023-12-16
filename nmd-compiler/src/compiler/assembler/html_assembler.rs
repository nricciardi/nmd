use build_html::{HtmlPage, HtmlContainer, Html};

use crate::compiler::{dossier::Dossier, artifact::Artifact, portability_level::PortabilityLevel};

use super::{Assembler, AssemblerError, assembler_configuration::AssemblerConfiguration};

pub struct HtmlAssembler {
    configuration: AssemblerConfiguration
}

impl HtmlAssembler {
    pub fn new(configuration: AssemblerConfiguration) -> Self {
        Self {
            configuration    
        }
    }

    fn assemble_all_in_one(&self, dossier: Dossier) -> Result<Artifact, AssemblerError> {
        let page: String = HtmlPage::new()
                            .with_title(dossier.name())
                            .to_html_string();

        todo!()
                            
    }
}

impl Assembler for HtmlAssembler {

    fn set_configuration(&mut self, configuration: AssemblerConfiguration) {
        self.configuration = configuration
    }

    fn assemble(&self, dossier: Dossier) -> Result<Artifact, AssemblerError> {
        match self.configuration.portability_level() {
            PortabilityLevel::AllInOne => self.assemble_all_in_one(dossier)
        }
    }
}