use std::borrow::Borrow;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use regex::Regex;
use thiserror::Error;

use crate::resource::disk_resource::DiskResource;
use crate::resource::{Resource, ResourceError};

use super::codex::Codex;
use super::dossier::document::chapter::chapter_builder::{self, ChapterBuilder};
use super::dossier::dossier_configuration::DossierConfiguration;
use super::dossier::Dossier;
use super::{dossier::{document::{chapter::{heading::{Heading, HeadingLevel}, paragraph::ParagraphError}, Chapter, Paragraph}, Document, DocumentError}};


#[derive(Error, Debug)]
pub enum LoadError {
    #[error(transparent)]
    ResourceError(#[from] ResourceError),

    #[error("elaboration error: {0}")]
    ElaborationError(String),
    
    #[error(transparent)]
    IoError(#[from] io::Error)
}

impl Clone for LoadError {
    fn clone(&self) -> Self {
        match self {
            Self::IoError(e) => Self::ElaborationError(e.to_string()),
            other => other.clone()
        }
    }
}


pub struct Loader {
    
}

impl Loader {


    fn count_newlines_at_start(s: &str) -> usize {
        s.bytes().take_while(|&b| b == b'\n').count()
    }

    fn count_newlines_at_end(s: &str) -> usize {
        s.bytes().rev().take_while(|&b| b == b'\n').count()
    }

    pub fn load_document_from_str(codex: &Codex, document_name: &str, content: &str) -> Result<Document, DocumentError> {

        let content = String::from(content);

        let mut document_chapters: Vec<Chapter> = Vec::new();

        // usize: chapter start/end position
        // String: chapter heading + options found
        let mut chapter_borders: Vec<(usize, usize, String)> = Vec::new();

        for chapter_modifier in codex.configuration().ordered_chapter_modifier() {
            
            let modifier_pattern = chapter_modifier.modifier_pattern();

            log::debug!("test {}", modifier_pattern);

            let regex = Regex::new(&modifier_pattern).unwrap();

            regex.find_iter(content.as_str()).for_each(|m| {

                let matched_str = m.as_str().to_string();

                // TODO: remove count_newlines?
                let start = m.start() + Self::count_newlines_at_start(&matched_str);
                let end = m.end() - Self::count_newlines_at_end(&matched_str) - 1;

                let overlap_chapter = chapter_borders.par_iter().find_any(|c| {
                    (c.0 >= start && c.1 <= end) ||     // current paragraph contains p
                    (c.0 <= start && c.1 >= end) ||     // p contains current paragraph
                    (c.0 <= start && c.1 >= start && c.1 <= end) ||     // left overlap
                    (c.0 >= start && c.0 <= end && c.1 >= end)          // right overlap
                });

                if let Some(p) = overlap_chapter {     // => overlap
                    log::debug!("discarded chapter:\n{}\nbecause there is an overlap between {} and {} using pattern {:?}:\n{:#?}\n", m.as_str(), start, end, modifier_pattern, p);
                    return
                }

                log::debug!("found chapter between {} and {}:\n{}\nusing {:?}", start, end, matched_str, &chapter_modifier);

                let cb = (
                    start,
                    end,
                    matched_str
                );

                log::debug!("push in chapter_borders: {:?}", cb);

                chapter_borders.push(cb);

            });
        }

        chapter_borders.par_sort_by(|a, b| a.0.cmp(&b.0));

        log::debug!("start to load chapters...");

        for index in 0..chapter_borders.len() {

            log::debug!("load chapter {:?}", chapter_borders[index]);

            let start = chapter_borders[index].0;
            let end = chapter_borders[index].1;
            let raw_content = &chapter_borders[index].2;

            let heading = Self::load_heading_from_str(codex, raw_content);

            if heading.is_none() {
                return Err(DocumentError::Load(ResourceError::ResourceNotFound("heading".to_string())))
            }

            let mut next_start: usize = content.len();

            if index < chapter_borders.len() - 1 {
                next_start = chapter_borders[index + 1].0;
            }

            let content = content.get(end..next_start).unwrap();     // exclude heading

            let paragraphs = Self::load_paragraphs_from_str(codex, content)?;

            document_chapters.push(Chapter::new(heading.unwrap(), paragraphs));
        }


        let mut preamble = String::new();

        let first_start = chapter_borders[0].0;
        if first_start != 0 {      // => there is a preamble
            preamble = String::from(content.get(0..first_start).unwrap())
        }

        let preamble = Self::load_paragraphs_from_str(codex, &preamble)?;

        Ok(Document::new(document_name.to_string(), preamble, document_chapters))

    }

    pub fn load_document_from_path(codex: &Codex, path_buf: &PathBuf) -> Result<Document, LoadError> {

        let resource = DiskResource::try_from(path_buf.clone())?;

        let content = resource.content()?;

        let document_name = resource.name();

        match Self::load_document_from_str(codex, document_name, &content) {
            Ok(document) => {
                return Ok(document)
            },
            Err(err) => return Err(LoadError::ElaborationError(err.to_string()))
        }
    }


    /// Split a string in the corresponding vector of paragraphs
    pub fn load_paragraphs_from_str(codex: &Codex, content: &str) -> Result<Vec<Paragraph>, ParagraphError> {

        let mut paragraphs: Vec<(usize, usize, Paragraph)> = Vec::new();
        let mut content = String::from(content);

        content = content.replace("\n\n", "\n\n\n");

        // work-around to fix paragraph matching end line
        while !content.ends_with("\n\n") {
            content.push_str("\n");
        }

        for paragraph_modifier in codex.configuration().ordered_paragraph_modifiers() {

            let search_pattern = paragraph_modifier.modifier_pattern();

            log::debug!("test {}", search_pattern);

            let regex = Regex::new(&search_pattern).unwrap();

            regex.find_iter(content.clone().as_str()).for_each(|m| {

                let matched_str = m.as_str().to_string();

                // TODO: remove count_newlines?
                let start = m.start() + Self::count_newlines_at_start(&matched_str);
                let end = m.end() - Self::count_newlines_at_end(&matched_str) - 1;

                let overlap_paragraph = paragraphs.par_iter().find_any(|p| {
                    (p.0 >= start && p.1 <= end) ||     // current paragraph contains p
                    (p.0 <= start && p.1 >= end) ||     // p contains current paragraph
                    (p.0 <= start && p.1 >= start && p.1 <= end) ||     // left overlap
                    (p.0 >= start && p.0 <= end && p.1 >= end)          // right overlap
                });

                if let Some(p) = overlap_paragraph {     // => overlap
                    log::debug!("discarded paragraph:\n{}\nbecause there is an overlap between {} and {} using pattern {:?}:\n{:#?}\n", m.as_str(), start, end, search_pattern, p);
                    return
                }

                log::debug!("found paragraph between {} and {}:\n{}\nusing {:?}", start, end, matched_str, &paragraph_modifier);

                let paragraph = Paragraph::new(matched_str, paragraph_modifier.identifier().clone());

                if !paragraph.contains_only_newlines() {
                    paragraphs.push((start, end, paragraph));
                }

            });
        }

        paragraphs.par_sort_by(|a, b| a.0.cmp(&b.0));           // TODO: maybe b.1

        Ok(paragraphs.iter().map(|p| p.2.to_owned()).collect())
    }


    pub fn load_heading_from_str(codex: &Codex, content: &str) -> Option<Heading> {
        let chapter_modifiers = codex.configuration().ordered_chapter_modifier();

        for chapter_modifier in chapter_modifiers {
            let regex = Regex::new(&chapter_modifier.modifier_pattern()).unwrap();

            if regex.is_match(content) {
                let matched = regex.captures(content).unwrap();

                log::debug!("{:#?}", matched);
                
                // TODO: missed heading search (different way to parse heading... not always 1 or 2)
                let level = HeadingLevel::from_str(matched.get(1).unwrap().as_str()).unwrap();
                let title = matched.get(2).unwrap().as_str();

                return Some(Heading::new(title.to_string()))
            }
        }

        Option::None
    }

    pub fn load_dossier_from_path_buf(codex: &Codex, path_buf: &PathBuf) -> Result<Dossier, LoadError> {
        let dossier_configuration = DossierConfiguration::try_from(path_buf)?;

        Self::load_dossier_from_dossier_configuration(codex, &dossier_configuration)
    }


    pub fn load_dossier_from_dossier_configuration(codex: &Codex, dossier_configuration: &DossierConfiguration) -> Result<Dossier, LoadError> {

        // TODO: are really mandatory?
        if dossier_configuration.raw_documents_paths().is_empty() {
            return Err(LoadError::ResourceError(ResourceError::InvalidResourceVerbose("there are no documents".to_string())))
        }

        // TODO: is really mandatory?
        if dossier_configuration.name().is_empty() {
            return Err(LoadError::ResourceError(ResourceError::InvalidResourceVerbose("there is no name".to_string())))
        }

        if dossier_configuration.compilation().parallelization() {

            let mut documents_res: Vec<Result<Document, LoadError>> = Vec::new();

            dossier_configuration.raw_documents_paths().par_iter()
            .map(|document_path| {
                Loader::load_document_from_path(codex, &PathBuf::from(document_path))
            }).collect_into_vec(&mut documents_res);
            
            let error = documents_res.par_iter().find_any(|result| result.is_err());

            // handle errors
            if let Some(Err(err)) = error.as_ref() {
                return Err(err.clone())
            }

            let documents = documents_res.into_iter().map(|d| d.unwrap()).collect();

            return Ok(Dossier::new(dossier_configuration.clone(), documents))


        } else {

            let mut documents: Vec<Document> = Vec::new();

            for document_path in dossier_configuration.raw_documents_paths() {
    
                let document = Loader::load_document_from_path(codex, &PathBuf::from(document_path))?;
    
                documents.push(document)
            }

            return Ok(Dossier::new(dossier_configuration.clone(), documents))
        }
    }
}