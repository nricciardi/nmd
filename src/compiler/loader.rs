use std::collections::HashSet;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use once_cell::sync::Lazy;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use regex::Regex;
use thiserror::Error;

use crate::compiler::codex::modifier::constants::{INCOMPATIBLE_CHAPTER_HEADING_REGEX, NEW_LINE};
use crate::compiler::codex::modifier::standard_chapter_modifier::StandardChapterModifier;
use crate::resource::disk_resource::DiskResource;
use crate::resource::{Resource, ResourceError};

use super::codex::modifier::constants::CHAPTER_STYLE_PATTERN;
use super::codex::Codex;
use super::dossier::document::chapter::chapter_tag::ChapterTag;
use super::dossier::dossier_configuration::DossierConfiguration;
use super::dossier::Dossier;
use super::dossier::{document::{chapter::heading::{Heading, HeadingLevel}, Chapter, Paragraph}, Document};



static CHAPTER_STYLE_PATTERN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(CHAPTER_STYLE_PATTERN).unwrap());
static FIND_EXTENDED_VERSION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"heading-[[:digit:]]+-extended-version").unwrap());
static FIND_COMPACT_VERSION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"heading-[[:digit:]]+-compact-version").unwrap());
static DOUBLE_NEW_LINES: Lazy<String> = Lazy::new(|| format!("{}{}", NEW_LINE, NEW_LINE));
static TRIPLE_NEW_LINES: Lazy<String> = Lazy::new(|| format!("{}{}{}", NEW_LINE, NEW_LINE, NEW_LINE));



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

    pub fn new() -> Self {
        Self {
        }
    }

    fn count_newlines_at_start(s: &str) -> usize {
        s.bytes().take_while(|&b| b == b'\n').count()
    }

    fn count_newlines_at_end(s: &str) -> usize {
        s.bytes().rev().take_while(|&b| b == b'\n').count()
    }

    pub fn load_document_from_str(&self, codex: &Codex, document_name: &str, content: &str) -> Result<Document, LoadError> {

        log::info!("loading document '{}' from its content...", document_name);

        let content = String::from(content);

        let mut document_chapters: Vec<Chapter> = Vec::new();

        log::debug!("start to find chapter borders in document '{}'", document_name);

        // usize: chapter start/end position
        let mut incompatible_chapter_heading_borders: Vec<(usize, usize)> = Vec::new();

        INCOMPATIBLE_CHAPTER_HEADING_REGEX.iter().for_each(|regex| {            // TODO: par_iter
            regex.find_iter(&content).for_each(|m| {
                incompatible_chapter_heading_borders.push((m.start(), m.end()));
            });
        });

        // usize: chapter start/end position
        // String: chapter heading + options found
        let mut chapter_borders: Vec<(usize, usize, String)> = Vec::new();

        for chapter_modifier in codex.configuration().ordered_chapter_modifier() {
            
            let modifier_pattern = chapter_modifier.modifier_pattern();

            log::debug!("find chapter borders using chapter modifier: {:#?}", chapter_modifier);

            chapter_modifier.modifier_pattern_regex().find_iter(content.as_str()).for_each(|m| {

                let matched_str = m.as_str().to_string();

                let start = m.start();
                let end = m.end();

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

                let not_heading = incompatible_chapter_heading_borders.par_iter().find_any(|border| {
                    (border.0 <= start && border.1 >= start) ||
                    (border.0 <= end && border.1 >= end)
                });

                if let Some(p) = not_heading {     // => overlap
                    log::debug!("discarded chapter:\n{}\nbecause there is in an incompatible slice between {} and {} ({:#?})", m.as_str(), start, end, p);
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

        log::debug!("loading {} chapters of document '{}'...", chapter_borders.len(), document_name);

        let mut last_heading_level: HeadingLevel = 0;

        // build chapters
        for index in 0..chapter_borders.len() {

            log::debug!("load chapter {:?}", chapter_borders[index]);

            let _start = chapter_borders[index].0;
            let end = chapter_borders[index].1;
            let raw_content = &chapter_borders[index].2;

            let (heading, tags) = self.load_chapter_metadata_from_raw_str(codex, raw_content, last_heading_level);

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

            let paragraphs = self.load_paragraphs_from_str(codex, sub_content)?;

            document_chapters.push(Chapter::new(heading, tags, paragraphs));
        }

        let mut preamble_end = content.len();

        if chapter_borders.len() > 0 {
            preamble_end = chapter_borders[0].0;
        }

        let preamble: Vec<Paragraph>;
        
        if preamble_end > 0 {      // => there is a preamble
            
            log::debug!("preamble found in document '{}'", document_name);

            let s = String::from(content.get(0..preamble_end).unwrap());

            preamble = self.load_paragraphs_from_str(codex, &s)?;
        
        } else {

            log::debug!("preamble not found in document '{}'", document_name);

            preamble = Vec::new();
        }

        log::info!("document '{}' loaded", document_name);

        Ok(Document::new(document_name.to_string(), preamble, document_chapters))

    }

    pub fn load_document_from_path(&self, codex: &Codex, path_buf: &PathBuf) -> Result<Document, LoadError> {

        if !path_buf.exists() {
            return Err(LoadError::ResourceError(ResourceError::InvalidResourceVerbose(format!("{} not exists", path_buf.to_string_lossy())))) 
        }

        let resource = DiskResource::try_from(path_buf.clone())?;

        let content = resource.content()?;

        let document_name = resource.name();

        match self.load_document_from_str(codex, document_name, &content) {
            Ok(document) => {
                return Ok(document)
            },
            Err(err) => return Err(LoadError::ElaborationError(err.to_string()))
        }
    }

    /// Split a string in the corresponding vector of paragraphs
    pub fn load_paragraphs_from_str(&self, codex: &Codex, content: &str) -> Result<Vec<Paragraph>, LoadError> {

        if content.trim().is_empty() {
            log::debug!("skip paragraphs loading: empty content");
            return Ok(Vec::new());
        }

        log::debug!("loading paragraph:\n{}", content);

        let mut paragraphs: Vec<(usize, usize, Paragraph)> = Vec::new();
        let mut content = String::from(content);

        content = content.replace(&(*DOUBLE_NEW_LINES), &(*TRIPLE_NEW_LINES));

        // work-around to fix paragraph matching end line
        while !content.starts_with(&(*DOUBLE_NEW_LINES)) {
            content.insert_str(0, NEW_LINE);
        }

        while !content.ends_with(&(*DOUBLE_NEW_LINES)) {
            content.push_str(NEW_LINE);
        }

        for paragraph_modifier in codex.configuration().ordered_paragraph_modifiers() {

            let search_pattern = paragraph_modifier.modifier_pattern();

            log::debug!("test paragraph modifier '{}': {:?}", paragraph_modifier.identifier(), search_pattern);

            paragraph_modifier.modifier_pattern_regex().find_iter(content.clone().as_str()).for_each(|m| {

                let matched_str = String::from(&content[m.start()..m.end()]);

                let start = m.start() + Self::count_newlines_at_start(&matched_str);
                let mut end = m.end() - 1;

                let nl_at_end = Self::count_newlines_at_end(&matched_str);
                if end > nl_at_end {
                    end -= nl_at_end;
                }

                log::debug!("found paragraph using '{}': {:?} between {} and {}:\n{}", paragraph_modifier.identifier(), search_pattern, start, end, matched_str);

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

                if matched_str.is_empty() {
                    log::debug!("paragraph discarded because empty");
                    return;
                }

                let matched_str = matched_str.replace(&(*TRIPLE_NEW_LINES), &(*DOUBLE_NEW_LINES));

                let paragraph = Paragraph::new(matched_str, paragraph_modifier.identifier().clone());

                if !paragraph.contains_only_newlines() {
                    log::debug!("added paragraph to paragraphs list:\n{:#?}", paragraph);
                    
                    paragraphs.push((start, end, paragraph));
                }

            });
        }

        paragraphs.par_sort_by(|a, b| a.0.cmp(&b.1));

        Ok(paragraphs.iter().map(|p| p.2.to_owned()).collect())
    }


    fn load_chapter_tags_from_raw_str(content: &str) -> Vec<ChapterTag> {
        
        let mut tags: Vec<ChapterTag> = Vec::new();
        
        for line in content.lines() {
            
            let tag = ChapterTag::from_str(line);

            if let Ok(tag) = tag {

                tags.push(tag);

            }
        }

        tags
    }

    fn load_chapter_style_from_raw_str(&self, content: &str) -> Option<String> {
        
        let mut style: Option<String> = None;

        if let Some(captures) = CHAPTER_STYLE_PATTERN_REGEX.captures(content) {
            if let Some(s) = captures.get(1) {
                style = Some(s.as_str().to_string())
            }
        }

        style
    }

    fn load_chapter_metadata_from_raw_str(&self, codex: &Codex, content: &str, last_heading_level: HeadingLevel) -> (Option<Heading>, Vec<ChapterTag>) {

        log::debug!("load chapter metadata from (last heading level: {}):\n{}", last_heading_level, content);

        let chapter_modifiers = codex.configuration().ordered_chapter_modifier();

        for chapter_modifier in chapter_modifiers {

            if !chapter_modifier.modifier_pattern_regex().is_match(content) {
                continue
            }

            // ==== MinorHeading ====
            if chapter_modifier.identifier().eq(&StandardChapterModifier::MinorHeading.identifier()) {
                let matched = chapter_modifier.modifier_pattern_regex().captures(content).unwrap();

                let level: HeadingLevel;

                if last_heading_level < 1 {
                    log::warn!("{} found, but last heading has level {}, so it is set as 1", StandardChapterModifier::MinorHeading.identifier(), last_heading_level);
                    level = 1;

                } else {

                    level = last_heading_level - 1;
                }

                let title = matched.get(1).unwrap().as_str();


                let tags = Self::load_chapter_tags_from_raw_str(content);

                return (Some(Heading::new(level, String::from(title))), tags);
            }

            // ==== MajorHeading ====
            if chapter_modifier.identifier().eq(&StandardChapterModifier::MajorHeading.identifier()) {
                let matched = chapter_modifier.modifier_pattern_regex().captures(content).unwrap();

                let mut level: HeadingLevel = last_heading_level + 1;

                if level < 1 {
                    log::warn!("level {} < 0, so it is set as 1", level);
                    level = 1;
                }

                let title = matched.get(1).unwrap().as_str();

                let tags = Self::load_chapter_tags_from_raw_str(content);

                return (Some(Heading::new(level, String::from(title))), tags);
            }

            // ==== SameHeading ====
            if chapter_modifier.identifier().eq(&StandardChapterModifier::SameHeading.identifier()) {
                let matched = chapter_modifier.modifier_pattern_regex().captures(content).unwrap();

                let level: HeadingLevel;
                if last_heading_level < 1 {
                    log::warn!("{} found, but last heading has level {}, so it is set as 1", StandardChapterModifier::MinorHeading.identifier(), last_heading_level);
                    level = 1;

                } else {

                    level = last_heading_level;
                }
                
                let title = matched.get(1).unwrap().as_str();

                let tags = Self::load_chapter_tags_from_raw_str(content);

                return (Some(Heading::new(level, String::from(title))), tags);
            }

            // ==== Extended version heading ====
            if FIND_EXTENDED_VERSION_REGEX.is_match(chapter_modifier.identifier()) {

                let level: u32 = content.chars().take_while(|&c| c == '#').count() as u32;

                let matched = chapter_modifier.modifier_pattern_regex().captures(content).unwrap();

                let title = matched.get(1).unwrap().as_str();

                let tags = Self::load_chapter_tags_from_raw_str(content);

                return (Some(Heading::new(level, String::from(title))), tags);
            }

            // ==== Compact version heading ====
            if FIND_COMPACT_VERSION_REGEX.is_match(chapter_modifier.identifier()) {
                let matched = chapter_modifier.modifier_pattern_regex().captures(content).unwrap();

                let level: HeadingLevel = matched.get(1).unwrap().as_str().parse().unwrap();
                let title = matched.get(2).unwrap().as_str();

                let tags = Self::load_chapter_tags_from_raw_str(content);

                return (Some(Heading::new(level, String::from(title))), tags);
            }

        }

        (None, Vec::new())
    }

    pub fn load_dossier_from_path_buf(&self, codex: &Codex, path_buf: &PathBuf) -> Result<Dossier, LoadError> {
        let dossier_configuration = DossierConfiguration::try_from(path_buf)?;

        self.load_dossier_from_dossier_configuration(codex, &dossier_configuration)
    }

    pub fn load_dossier_from_path_buf_only_documents(&self, codex: &Codex, path_buf: &PathBuf, only_documents: &HashSet<String>) -> Result<Dossier, LoadError> {
        let mut dossier_configuration = DossierConfiguration::try_from(path_buf)?;

        let d: Vec<String> = dossier_configuration.raw_documents_paths().iter()
                                                    .filter(|item| {

                                                        let file_name = PathBuf::from(*item).file_name().unwrap().to_string_lossy().to_string();

                                                        only_documents.contains(file_name.as_str())
                                                    })
                                                    .map(|item| item.clone())
                                                    .collect();

        dossier_configuration.set_raw_documents_paths(d);

        self.load_dossier_from_dossier_configuration(codex, &dossier_configuration)
    }

    pub fn load_dossier_from_dossier_configuration(&self, codex: &Codex, dossier_configuration: &DossierConfiguration) -> Result<Dossier, LoadError> {

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
                self.load_document_from_path(codex, &PathBuf::from(document_path))
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
    
                let document = self.load_document_from_path(codex, &PathBuf::from(document_path))?;
    
                documents.push(document)
            }

            return Ok(Dossier::new(dossier_configuration.clone(), documents))
        }
    }
}



#[cfg(test)]
mod test {

    use crate::compiler::{codex::codex_configuration::CodexConfiguration, loader::Loader};

    use super::*;

    #[test]
    fn chapters_from_str() {

        let codex = Codex::of_html(CodexConfiguration::default());

        let content: String = 
r#"
# title 1a

paragraph 1a

## title 2a

paragraph 2a

# title 1b

paragraph 1b
"#.trim().to_string();

        let loader = Loader::new();

        let document = loader.load_document_from_str(&codex, "test", &content).unwrap();

        assert_eq!(document.preamble().len(), 0);

        assert_eq!(document.chapters().len(), 3);


        
    }

    #[test]
    fn paragraphs_from_str() {
        let content = concat!(
            "paragraph1",
            "\n\n",
            "paragraph2a\nparagraph2b",
            "\n\n",
            "paragraph3",
        );

        let codex = Codex::of_html(CodexConfiguration::default());

        let loader = Loader::new();

        let paragraphs = loader.load_paragraphs_from_str(&codex, content).unwrap();

        assert_eq!(paragraphs.len(), 3)
    }
}