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
}

pub trait Assembler {

    fn configuration(&self) -> &AssemblerConfiguration;

    fn set_configuration(&mut self, configuration: AssemblerConfiguration);

    fn assemble_dossier(&self, /*codex: &Codex,*/ dossier: &Dossier) -> Result<Artifact, AssemblerError>;

    // TODO: change return type
    fn assemble_document(&self, document: &Document) -> Result<String, AssemblerError> {
        let mut result = String::new();

        for paragraph in document.preamble() {
            result.push_str(paragraph.parsed_content().unwrap().parsed_content());
        }

        for chapter in document.chapters() {

            result.push_str(chapter.heading().parsed_content().unwrap().parsed_content());

            for paragraph in chapter.paragraphs() {
                result.push_str(paragraph.parsed_content().unwrap().parsed_content());
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