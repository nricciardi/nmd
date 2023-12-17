use build_html::{HtmlPage, HtmlContainer, Html, Container};

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
        let mut page = HtmlPage::new()
                                .with_title(dossier.name())
                                .with_meta(vec![("charset", "utf-8")]);

        if let Some(documents) = dossier.documents() {

            for document in documents {
                let section = Container::new(build_html::ContainerType::Section)
                                                .with_raw(document);

                page.add_container(section);
            }

        } else {
            return Err(AssemblerError::TooFewElements)
        }

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

#[cfg(test)]
mod test {

    use std::{error, path::PathBuf};

    use crate::compiler::{loadable::Loadable, resource::Resource, dossier::dossier_configuration::{self, DossierConfiguration}, artifact};

    use super::*;

    #[test]
    fn assemble() {

        let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dossier_dir = "nmd-test-dossier-1";
        let project_directory = project_directory.join("test-resources").join(dossier_dir).join("d1.nmd");

        assert!(project_directory.is_file());

        let resource = Resource::new(project_directory).unwrap();

        let mut dossier_configuration = DossierConfiguration::default();
        dossier_configuration.set_documents(vec![resource]);

        let dossier = Dossier::load(&dossier_configuration).unwrap();

        let assembler = HtmlAssembler::new(AssemblerConfiguration::default());

        let artifact = assembler.assemble(*dossier).unwrap();

        todo!()
    }
}