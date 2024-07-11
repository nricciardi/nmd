use std::path::PathBuf;

use thiserror::Error;

use crate::resource::{Resource, ResourceError};

use self::{html_assembler::HtmlAssembler, assembler_configuration::AssemblerConfiguration};

use super::{artifact::{Artifact, ArtifactError}, bibliography::Bibliography, dossier::{Document, Dossier}, output_format::OutputFormat, parsing::parsing_error::ParsingError, table_of_contents::TableOfContents};

pub mod html_assembler;
pub mod assembler_configuration;


#[derive(Error, Debug)]
pub enum AssemblerError {
    #[error("too few elements to assemble: {0}")]
    TooFewElements(String),

    #[error(transparent)]
    ArtifactError(#[from] ArtifactError),

    #[error(transparent)]
    ParseError(#[from] ParsingError),

    #[error("expected parsed content not found")]
    ParsedContentNotFound,

    #[error(transparent)]
    ResourceError(#[from] ResourceError),
}

pub trait Assembler {

    fn configuration(&self) -> &AssemblerConfiguration;

    fn set_configuration(&mut self, configuration: AssemblerConfiguration);

    fn assemble_dossier(&self, dossier: &Dossier) -> Result<Artifact, AssemblerError>;

    fn assemble_document(&self, document: &Document) -> Result<Artifact, AssemblerError> {

        let mut result = String::new();

        for paragraph in document.preamble() {

            if let Some(parsed_content) = paragraph.parsed_content().as_ref() {

                result.push_str(&parsed_content.parsed_content());

            } else {

                return Err(AssemblerError::ParsedContentNotFound)
            }
        }

        for chapter in document.chapters() {

            if let Some(parsed_content) = chapter.heading().parsed_content().as_ref() {

                result.push_str(&parsed_content.parsed_content());

            } else {

                return Err(AssemblerError::ParsedContentNotFound)
            }

            for paragraph in chapter.paragraphs() {
                if let Some(parsed_content) = paragraph.parsed_content().as_ref() {

                    result.push_str(&parsed_content.parsed_content());
    
                } else {

                    return Err(AssemblerError::ParsedContentNotFound)
                }
            }
        }

        Ok(Artifact::new(result))

    }

    fn assemble_document_standalone(&self, page_title: &String, styles_references: Option<&Vec<String>>, toc: Option<&TableOfContents>, bibliography: Option<&Bibliography>, document: &Document) -> Result<Artifact, AssemblerError> {
        self.assemble_document(document)
    }
}

pub fn from(format: OutputFormat, configuration: AssemblerConfiguration) -> Box<dyn Assembler> {
    match format {
        OutputFormat::Html => Box::new(HtmlAssembler::new(configuration))  
    }
}