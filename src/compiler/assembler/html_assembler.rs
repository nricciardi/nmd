use build_html::{HtmlPage, HtmlContainer, Html, Container};

use crate::compiler::{artifact::Artifact, dossier::Dossier, theme::Theme};

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

        if self.configuration.use_remote_addons() {

            // add code block js/css
            match self.configuration.theme() {
                Theme::Light => {
                    page = page
                        .with_script_link_attr("https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-core.min.js", [
                            ("crossorigin", "anonymous"),
                        ])
                        .with_script_link_attr("https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js", [
                            ("crossorigin", "anonymous"),
                        ])
                        .with_head_link("https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism.css", "stylesheet");
                    
                },
                Theme::Dark => {
                    page = page
                        .with_script_link_attr("https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-core.min.js", [
                            ("crossorigin", "anonymous"),
                        ])
                        .with_script_link_attr("https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js", [
                            ("crossorigin", "anonymous"),
                        ])
                        .with_head_link("https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-okaidia.css", "stylesheet");
                },
            }

            page = page
                    // add math block js/css
                    .with_head_link_attr("https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css", "stylesheet", [
                        ("integrity", "sha384-n8MVd4RsNIU0tAv4ct0nTaAbDJwPJzDEaqSD1odI+WdtXRGWt2kTvGFasHpSy3SV"),
                        ("crossorigin", "anonymous")
                    ])
                    .with_script_link_attr("https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js", [
                        ("integrity", "sha384-XjKyOOlGwcjNTAIQHIpgOno0Hl1YQqzUOEleOLALmuqehneUG+vnGctmUb0ZY0l8"),
                        ("crossorigin", "anonymous")
                    ])
                    .with_script_link_attr("https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js", [
                        ("integrity", "sha384-+VBxd3r6XgURycqtZ117nYw44OOcIax56Z4dCRWbxyPt0Koah1uHoK0o4+/RRE05"),
                        ("crossorigin", "anonymous")
                    ]);

            page.add_script_literal(r#"
                    document.addEventListener("DOMContentLoaded", function() {
                        renderMathInElement(document.body, {
                            
                            delimiters: [
                                {left: '$$', right: '$$', display: true},
                                {left: '$', right: '$', display: false},
                            ],
                            
                            throwOnError : false
                        });
                    });"#);
        } else {

            page.add_style(include_str!("html_assembler/math_block/katex.css"));
            page.add_style(include_str!("html_assembler/math_block/katex-fonts.css"));
            page.add_script_literal(include_str!("html_assembler/math_block/katex.min.js"));
            page.add_script_literal(include_str!("html_assembler/math_block/auto-render.min.js"));
            page.add_script_literal(r#"window.onload = function() {
                renderMathInElement(document.body, {
                    delimiters: [
                        {left: '$$', right: '$$', display: true},
                        {left: '$', right: '$', display: false},
                    ],
                    throwOnError : false
                  });
            }"#);

            // add code block js/css                        
            match self.configuration.theme() {
                Theme::Light => {
                    page.add_style(include_str!("html_assembler/code_block/light_theme/prismjs.min.css"));
                    page.add_script_literal(include_str!("html_assembler/code_block/light_theme/prismjs.min.js"));
                },
                Theme::Dark => {
                    page.add_style(include_str!("html_assembler/code_block/dark_theme/prismjs.min.css"));
                    page.add_script_literal(include_str!("html_assembler/code_block/dark_theme/prismjs.min.js"));
                },
            }
        }

        match self.configuration.theme() {
            Theme::Light => page.add_style(include_str!("html_assembler/default_style/light_theme.css")),
            Theme::Dark => page.add_style(include_str!("html_assembler/default_style/dark_theme.css")),
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

    use crate::compiler::{loadable::Loadable, dossier::dossier_configuration::DossierConfiguration, parsable::{Parsable, codex::{Codex, codex_configuration::CodexConfiguration}, ParsingConfiguration}};

    use super::*;

    #[test]
    fn assemble() {

        let codex = Arc::new(Codex::of_html(CodexConfiguration::default()));

        let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dossier_dir = "nmd-test-dossier-1";
        let nmd_file = project_directory.join("test-resources").join(dossier_dir).join("d1.nmd");

        assert!(nmd_file.is_file());

        let mut dossier_configuration = DossierConfiguration::default();
        dossier_configuration.set_documents(vec![nmd_file.to_string_lossy().to_string()]);

        let mut dossier = Dossier::load(Arc::clone(&codex), &dossier_configuration).unwrap();

        dossier.parse(Arc::clone(&codex), Arc::new(ParsingConfiguration::default())).unwrap();

        let assembler = HtmlAssembler::new(AssemblerConfiguration::default());

        let _ = assembler.assemble(*dossier).unwrap();
    }
}