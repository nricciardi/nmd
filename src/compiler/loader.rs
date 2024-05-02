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

    pub fn load_document_from_str(codex: &Codex, document_name: &str, content: &str) -> Result<Document, LoadError> {

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

                log::debug!("found chapter border between {} and {}:\n{}\nusing {:?}", start, end, matched_str, &chapter_modifier);

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

        log::debug!("start to load chapters of document: '{}'...", document_name);

        let mut last_heading_level: HeadingLevel = 0;

        // build chapters
        for index in 0..chapter_borders.len() {

            log::debug!("load chapter {:?}", chapter_borders[index]);

            let start = chapter_borders[index].0;
            let end = chapter_borders[index].1;
            let raw_content = &chapter_borders[index].2;

            let heading = Self::load_heading_from_raw_str(codex, raw_content, last_heading_level);

            if heading.is_none() {
                return Err(LoadError::ResourceError(ResourceError::ResourceNotFound("heading".to_string())))
            }

            let heading = heading.unwrap();

            last_heading_level = heading.level();

            let mut next_start: usize = content.len();

            if index < chapter_borders.len() - 1 {
                next_start = chapter_borders[index + 1].0;
            }

            let sub_content = content.get(end..next_start).unwrap();     // exclude heading

            let paragraphs = Self::load_paragraphs_from_str(codex, sub_content)?;

            document_chapters.push(Chapter::new(heading, paragraphs));
        }


        let mut preamble = String::new();

        let mut first_start = 0;

        if chapter_borders.len() > 0 {
            first_start = chapter_borders[0].0;
        }

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
    pub fn load_paragraphs_from_str(codex: &Codex, content: &str) -> Result<Vec<Paragraph>, LoadError> {

        let mut paragraphs: Vec<(usize, usize, Paragraph)> = Vec::new();
        let mut content = String::from(content);

        content = content.trim_end_matches('\n').to_string();
        content = content.replace("\n\n", "\n\n\n");

        // work-around to fix paragraph matching end line
        while !content.ends_with("\n\n") {
            content.push_str("\n");
        }

        for paragraph_modifier in codex.configuration().ordered_paragraph_modifiers() {

            let search_pattern = paragraph_modifier.modifier_pattern();

            log::debug!("test '{}': {}", paragraph_modifier.identifier(), search_pattern);

            let regex = Regex::new(&search_pattern).unwrap();

            regex.find_iter(content.clone().as_str()).for_each(|m| {

                let matched_str = String::from(&content[m.start()..m.end()]);

                let start = m.start() + Self::count_newlines_at_start(&matched_str);
                let mut end = m.end() - 1;

                let nl_at_end = Self::count_newlines_at_end(&matched_str);
                if end > nl_at_end {
                    end -= nl_at_end;
                }

                log::debug!("found paragraph using {:?} between {} and {}:\n{}", &paragraph_modifier, start, end, matched_str);

                let overlap_paragraph = paragraphs.par_iter().find_any(|p| {
                    (p.0 >= start && p.1 <= end) ||     // current paragraph contains p
                    (p.0 <= start && p.1 >= end) ||     // p contains current paragraph
                    (p.0 <= start && p.1 >= start && p.1 <= end) ||     // left overlap
                    (p.0 >= start && p.0 <= end && p.1 >= end)          // right overlap
                });

                if let Some(p) = overlap_paragraph {     // => overlap
                    log::debug!("paragraph discarded because there is an overlap between {} and {} using pattern {:?}:\n{:#?}\n", start, end, search_pattern, p);
                    return
                }

                let paragraph = Paragraph::new(matched_str, paragraph_modifier.identifier().clone());

                if !paragraph.contains_only_newlines() {
                    log::debug!("added paragraph to paragraphs list:\n{:#?}", paragraph);
                    
                    paragraphs.push((start, end, paragraph));
                }

            });
        }

        paragraphs.par_sort_by(|a, b| a.0.cmp(&b.0));           // TODO: maybe b.1

        Ok(paragraphs.iter().map(|p| p.2.to_owned()).collect())
    }


    fn load_heading_from_raw_str(codex: &Codex, content: &str, last_heading_level: HeadingLevel) -> Option<Heading> {

        log::debug!("load heading from (last heading level: {}):\n{}", last_heading_level, content);

        let chapter_modifiers = codex.configuration().ordered_chapter_modifier();

        for chapter_modifier in chapter_modifiers {

            let modifier_regex = Regex::new(chapter_modifier.modifier_pattern()).unwrap();

            if !modifier_regex.is_match(content) {
                continue
            }

            let regex_to_find_extended_version = Regex::new(r"heading-[[:digit:]]+-extended-version").unwrap();
            let regex_to_find_compact_version = Regex::new(r"heading-[[:digit:]]+-compact-version").unwrap();

            if regex_to_find_extended_version.is_match(chapter_modifier.identifier()) {

                let level: u32 = content.chars().take_while(|&c| c == '#').count() as u32;

                let matched = modifier_regex.captures(content).unwrap();

                let title = matched.get(1).unwrap().as_str();

                return Some(Heading::new(level, String::from(title)));
            }

            if regex_to_find_compact_version.is_match(chapter_modifier.identifier()) {
                let matched = modifier_regex.captures(content).unwrap();

                let level: HeadingLevel = matched.get(1).unwrap().as_str().parse().unwrap();
                let title = matched.get(2).unwrap().as_str();

                return Some(Heading::new(level, String::from(title)));
            }

            // TODO: others modifiers (e.g. #+, #=, #-)

        }

        Option::None
    }

    pub fn load_dossier_from_path_buf(codex: &Codex, path_buf: &PathBuf) -> Result<Dossier, LoadError> {
        let dossier_configuration = DossierConfiguration::try_from(path_buf)?;

        Self::load_dossier_from_dossier_configuration(codex, &dossier_configuration)
    }


    pub fn load_dossier_from_dossier_configuration(codex: &Codex, dossier_configuration: &DossierConfiguration) -> Result<Dossier, LoadError> {

        // TODO: are really mandatory?
        if dossier_configuration.documents_paths().is_empty() {
            return Err(LoadError::ResourceError(ResourceError::InvalidResourceVerbose("there are no documents".to_string())))
        }

        // TODO: is really mandatory?
        if dossier_configuration.name().is_empty() {
            return Err(LoadError::ResourceError(ResourceError::InvalidResourceVerbose("there is no name".to_string())))
        }

        if dossier_configuration.compilation().parallelization() {

            let mut documents_res: Vec<Result<Document, LoadError>> = Vec::new();

            dossier_configuration.documents_paths().par_iter()
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

            for document_path in dossier_configuration.documents_paths() {
    
                let document = Loader::load_document_from_path(codex, &PathBuf::from(document_path))?;
    
                documents.push(document)
            }

            return Ok(Dossier::new(dossier_configuration.clone(), documents))
        }
    }
}