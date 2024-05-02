use std::{str::FromStr, sync::{Arc, Mutex}};

use build_html::{HtmlPage, HtmlContainer, Html, Container};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{compiler::{artifact::Artifact, dossier::Dossier, theme::Theme}, resource::{disk_resource, dynamic_resource::DynamicResource, remote_resource, Resource, ResourceError}, utility::file_utility};

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


    fn apply_remote_addons(&self, mut page: HtmlPage) -> HtmlPage {
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
                    .with_head_link("https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism.css", "stylesheet")
                    .with_head_link("https://emoji-css.afeld.me/emoji.css", "stylesheet");
                
            },
            Theme::Dark => {
                page = page

                    .with_script_link_attr("https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-core.min.js", [
                        ("crossorigin", "anonymous"),
                    ])
                    .with_script_link_attr("https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/plugins/autoloader/prism-autoloader.min.js", [
                        ("crossorigin", "anonymous"),
                    ])
                    .with_head_link("https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-okaidia.css", "stylesheet")
                    .with_head_link("https://emoji-css.afeld.me/emoji.css", "stylesheet");
            },
            Theme::Scientific => todo!(),
            Theme::Vintage => todo!(),
            Theme::None => todo!(),
        };

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

        page
    }

    fn apply_local_addons(&self, mut page: HtmlPage) -> HtmlPage {

        page.add_style(include_str!("html_assembler/emoji/emoji.min.css"));
        
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
            Theme::Scientific => todo!(),
            Theme::Vintage => todo!(),
            Theme::None => todo!(),
        };

        page
    }
}

impl Assembler for HtmlAssembler {

    fn set_configuration(&mut self, configuration: AssemblerConfiguration) {
        self.configuration = configuration
    }

    fn assemble_dossier(&self, dossier: &Dossier) -> Result<Artifact, AssemblerError> {
                        
        if dossier.documents().is_empty() {
            return Err(AssemblerError::TooFewElements("there are no documents".to_string()))
        }

        let mut artifact = Artifact::new(self.configuration.output_location().clone())?;

        let mut page = HtmlPage::new()
                                .with_title(dossier.name())
                                .with_meta(vec![("charset", "utf-8")]);

        if self.configuration.use_remote_addons() {
            page = self.apply_remote_addons(page);
            
        } else {
            page = self.apply_local_addons(page);
        }

        // apply embedded styles
        match self.configuration.theme() {
            Theme::Light => page.add_style(include_str!("html_assembler/default_style/light_theme.css")),
            Theme::Dark => page.add_style(include_str!("html_assembler/default_style/dark_theme.css")),
            Theme::Scientific => todo!(),
            Theme::Vintage => todo!(),
            Theme::None => todo!(),
        }

        let styles_references = dossier.configuration().style().styles_references();
        log::info!("appending {} custom styles", styles_references.len());

        for style_ref in styles_references {

            log::debug!("appending style (reference): {}", style_ref);

            let resource = DynamicResource::from_str(style_ref)?;

            match resource {
                DynamicResource::DiskResource(disk_resource) => page.add_style(disk_resource.read()?),
                DynamicResource::ImageResource(_) => return Err(AssemblerError::ResourceError(ResourceError::InvalidResourceVerbose("image cannot be an addons".to_string()))),
                DynamicResource::RemoteResource(remote_resource) => {
                    page = page.with_script_link_attr(remote_resource.location().to_string(), [
                        ("crossorigin", "anonymous"),
                    ])
                },
            }
        }

        if self.configuration.parallelization() {

            let mut assembled_documents: Vec<Result<String, AssemblerError>> = Vec::new();

            dossier.documents().par_iter().map(|document| {
                self.assemble_document(document)
            }).collect_into_vec(&mut assembled_documents);

            for assembled_document in assembled_documents {
                let section = Container::new(build_html::ContainerType::Section)
                                                .with_raw(assembled_document?);
    
                page.add_container(section);
            }

        } else {
            for document in dossier.documents() {
                let section = Container::new(build_html::ContainerType::Section)
                                                .with_raw(self.assemble_document(document)?);
    
                page.add_container(section);
            }
        }

        let document_name = file_utility::build_output_file_name(dossier.name(), "html");

        artifact.add_document(&document_name, &page.to_html_string())?;


        Ok(artifact)
    }
    
    fn configuration(&self) -> &AssemblerConfiguration {
        &self.configuration
    }
}

// #[cfg(test)]
// mod test {

//     use std::{path::PathBuf, sync::Arc};

//     use crate::compiler::{codex::Codex, dossier::dossier_configuration::DossierConfiguration, parser::parsing_rule::parsing_configuration::ParsingConfiguration};

//     use super::*;

//     #[test]
//     fn assemble() {

//         let codex = Arc::new(Codex::of_html(CodexConfiguration::default()));

//         let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//         let dossier_dir = "nmd-test-dossier-1";
//         let nmd_file = project_directory.join("test-resources").join(dossier_dir).join("d1.nmd");

//         assert!(nmd_file.is_file());

//         let mut dossier_configuration = DossierConfiguration::default();
//         dossier_configuration.set_raw_documents_paths(vec![nmd_file.to_string_lossy().to_string()]);

//         let mut dossier = Dossier::load(Arc::clone(&codex), &dossier_configuration).unwrap();

//         dossier.parse(Arc::clone(&codex), Arc::new(ParsingConfiguration::default())).unwrap();

//         let assembler = HtmlAssembler::new(AssemblerConfiguration::default());

//         let _ = assembler.assemble(codex.into(), *dossier).unwrap();
//     }
// }