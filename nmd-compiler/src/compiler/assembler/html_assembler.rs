use build_html::{HtmlPage, HtmlContainer, Html, Container};

use crate::compiler::{dossier::Dossier, artifact::{Artifact, self}, portability_level::PortabilityLevel};

use super::{Assembler, AssemblerError, assembler_configuration::AssemblerConfiguration};

pub struct HtmlAssembler {
    configuration: AssemblerConfiguration
}

impl HtmlAssembler {
    pub fn new(configuration: AssemblerConfiguration) -> Self {
        Self {
            configuration,
        }
    }

    fn assemble_all_in_one(&self, dossier: Dossier) -> Result<Artifact, AssemblerError> {

        let mut artifact = Artifact::new(self.configuration.output_location().clone());

        let mut page = HtmlPage::new()
                                .with_title(dossier.name())
                                .with_meta(vec![("charset", "utf-8")]);

        if let Some(documents) = dossier.documents() {

            for document in documents {
                let section = Container::new(build_html::ContainerType::Section)
                                                .with_raw(document);

                page.add_container(section);
            }

            // TODO:
            // - a file name parse utility
            // - prevent to save content in add_document 

            artifact.add_document(&format!("{}.html", dossier.name()).replace(" ", "-").to_ascii_lowercase(), &page.to_html_string())?;

        } else {
            return Err(AssemblerError::TooFewElements)
        }

        Ok(artifact)
                            
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

    use std::{path::PathBuf, sync::Arc};

    use crate::compiler::{loadable::Loadable, resource::Resource, dossier::dossier_configuration::{self, DossierConfiguration}, artifact, parsable::{Parsable, codex::Codex, ParsingConfiguration}};

    use super::*;

    #[test]
    fn assemble() {

        let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dossier_dir = "nmd-test-dossier-1";
        let nmd_file = project_directory.join("test-resources").join(dossier_dir).join("d1.nmd");

        assert!(nmd_file.is_file());

        let resource = Resource::try_from(nmd_file).unwrap();

        let mut dossier_configuration = DossierConfiguration::default();
        dossier_configuration.set_documents(vec![resource]);

        let mut dossier = Dossier::load(&dossier_configuration).unwrap();

        dossier.parse(Arc::new(Codex::of_html()), Arc::new(ParsingConfiguration::default())).unwrap();

        let assembler = HtmlAssembler::new(AssemblerConfiguration::default());

        let artifact = assembler.assemble(*dossier).unwrap();
    }
}