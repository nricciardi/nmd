use build_html::{HtmlPage, HtmlContainer, Html, Container};

use crate::compiler::{dossier::Dossier, artifact::Artifact};

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
}

impl Assembler for HtmlAssembler {

    fn set_configuration(&mut self, configuration: AssemblerConfiguration) {
        self.configuration = configuration
    }

    fn assemble(&self, dossier: Dossier) -> Result<Artifact, AssemblerError> {
        let mut artifact = Artifact::new(self.configuration.output_location().clone());

        let mut page = HtmlPage::new()
                                .with_title(dossier.name())
                                .with_meta(vec![("charset", "utf-8")]);

        match self.configuration.theme() {
            crate::compiler::theme::Theme::Light => {
                page.add_style(include_str!("html_assembler/code_block/light_theme/code_block.css"));
                page.add_script_literal(include_str!("html_assembler/code_block/light_theme/code_block.js"));
            },
            crate::compiler::theme::Theme::Dark => {
                page.add_style(include_str!("html_assembler/code_block/dark_theme/code_block.css"));
                page.add_script_literal(include_str!("html_assembler/code_block/dark_theme/code_block.js"));
            },
        }

        if dossier.documents().is_empty() {
            return Err(AssemblerError::TooFewElements("there are no documents".to_string()))
        }

        for document in dossier.documents() {
            let section = Container::new(build_html::ContainerType::Section)
                                            .with_raw(document);

            page.add_container(section);
        }

        // TODO:
        // - a file name parse utility

        let document_name = &format!("{}.html", dossier.name()).replace(" ", "-").to_ascii_lowercase();

        artifact.add_document(document_name, &page.to_html_string())?;


        Ok(artifact)
    }
}

#[cfg(test)]
mod test {

    use std::{path::PathBuf, sync::Arc};

    use crate::compiler::{loadable::Loadable, dossier::dossier_configuration::DossierConfiguration, artifact, parsable::{Parsable, codex::{Codex, codex_configuration::CodexConfiguration}, ParsingConfiguration}};

    use super::*;

    #[test]
    fn assemble() {

        let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dossier_dir = "nmd-test-dossier-1";
        let nmd_file = project_directory.join("test-resources").join(dossier_dir).join("d1.nmd");

        assert!(nmd_file.is_file());

        let mut dossier_configuration = DossierConfiguration::default();
        dossier_configuration.set_documents(vec![nmd_file.to_string_lossy().to_string()]);

        let mut dossier = Dossier::load(&dossier_configuration).unwrap();

        dossier.parse(Arc::new(Codex::of_html(CodexConfiguration::default())), Arc::new(ParsingConfiguration::default())).unwrap();

        let assembler = HtmlAssembler::new(AssemblerConfiguration::default());

        let artifact = assembler.assemble(*dossier).unwrap();
    }
}