use thiserror::Error;

use self::{html_assembler::HtmlAssembler, assembler_configuration::AssemblerConfiguration};

use super::{artifact::{Artifact, ArtifactError}, codex::Codex, dossier::{Document, Dossier}, output_format::OutputFormat, parser::{parsed_content_accessor::ParsedContentAccessor, parsing_rule::{parsing_error::ParsingError, parsing_outcome::ParsingOutcome}}};

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
}

pub trait Assembler {

    fn configuration(&self) -> &AssemblerConfiguration;

    fn set_configuration(&mut self, configuration: AssemblerConfiguration);

    fn assemble_dossier(&self, /*codex: &Codex,*/ dossier: &Dossier) -> Result<Artifact, AssemblerError>;

    // TODO: change return type
    fn assemble_document(&self, document: &Document) -> Result<String, AssemblerError> {
        let mut result = String::new();

        for paragraph in document.preamble() {

            if let Some(parsed_content) = paragraph.parsed_content().as_ref() {

                result.push_str(parsed_content.parsed_content());

            } else {
                return Err(AssemblerError::ParsedContentNotFound)
            }
        }

        for chapter in document.chapters() {

            if let Some(parsed_content) = chapter.heading().parsed_content().as_ref() {

                result.push_str(parsed_content.parsed_content());

            } else {
                return Err(AssemblerError::ParsedContentNotFound)
            }

            for paragraph in chapter.paragraphs() {
                if let Some(parsed_content) = paragraph.parsed_content().as_ref() {

                    result.push_str(parsed_content.parsed_content());
    
                } else {
                    return Err(AssemblerError::ParsedContentNotFound)
                }
            }
        }

        Ok(result)
    }
}

pub fn from(format: OutputFormat, configuration: AssemblerConfiguration) -> Box<dyn Assembler> {
    match format {
        OutputFormat::Html => Box::new(HtmlAssembler::new(configuration))  
    }
}