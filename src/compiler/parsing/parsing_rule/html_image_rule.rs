use std::sync::{RwLock, RwLockReadGuard};
use std::{path::PathBuf, sync::Arc};

use build_html::{Container, Html, HtmlContainer};
use log;
use once_cell::sync::Lazy;
use regex::{Regex, Captures};
use url::Url;

use crate::compiler::codex::modifier::standard_paragraph_modifier::StandardParagraphModifier;
use crate::compiler::codex::modifier::ModifierIdentifier;
use crate::compiler::codex::Codex;
use crate::compiler::dossier;
use crate::compiler::parser::Parser;
use crate::compiler::parsing::parsing_configuration::ParsingConfiguration;
use crate::compiler::parsing::parsing_error::ParsingError;
use crate::compiler::parsing::parsing_outcome::ParsingOutcome;
use crate::resource::resource_reference::ResourceReference;
use crate::resource::{image_resource::ImageResource, remote_resource::RemoteResource};

use super::ParsingRule;

static ALIGN_ITEM_PATTERN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(ALIGN_ITEM_PATTERN).unwrap());

const MULTI_IMAGE_PERMITTED_MODIFIER: &'static [StandardParagraphModifier] = &[StandardParagraphModifier::Image, StandardParagraphModifier::AbridgedImage];
const DEFAULT_JUSTIFY_CONTENT: &str = "normal";
const DEFAULT_ALIGN_SELF: &str = "center";
const ALIGN_ITEM_PATTERN: &str = r":([\w-]*):";


#[derive(Debug)]
/// Rule to replace a NMD text based on a specific pattern matching rule
pub struct HtmlImageRule {
    image_modifier_identifier: ModifierIdentifier,
    search_pattern: String,
    search_pattern_regex: Regex,
}

impl HtmlImageRule {
    
    pub fn new(image_modifier_identifier: ModifierIdentifier) -> Self {

        let searching_pattern = Self::get_searching_pattern(&image_modifier_identifier);

        Self {
            image_modifier_identifier,
            search_pattern_regex: Regex::new(&searching_pattern).unwrap(),
            search_pattern: searching_pattern,
        }
    }

    fn set_searching_pattern(&mut self, searching_pattern: String) {
        self.search_pattern = searching_pattern
    }

    fn get_searching_pattern(image_modifier_identifier: &ModifierIdentifier) -> String {

        if image_modifier_identifier.eq(&StandardParagraphModifier::Image.identifier()) {
            return StandardParagraphModifier::Image.modifier_pattern_with_paragraph_separator()
        }

        if image_modifier_identifier.eq(&StandardParagraphModifier::AbridgedImage.identifier()) {
            return StandardParagraphModifier::AbridgedImage.modifier_pattern_with_paragraph_separator()
        }

        if image_modifier_identifier.eq(&StandardParagraphModifier::MultiImage.identifier()) {
            return StandardParagraphModifier::MultiImage.modifier_pattern_with_paragraph_separator()
        }

        log::error!("'{}' is unsupported image modifier identifier", image_modifier_identifier);

        panic!("unsupported image modifier identifier");
    }

    fn create_figure_img(src: &str, alt: Option<&str>, caption: Option<&str>, id: Option<ResourceReference>, img_classes: Vec<&str>, style: Option<String>) -> String {

        let id_attr: String;

        if let Some(id) = id {
            id_attr = format!(r#"id="{}""#, id.build());
        } else {
            id_attr = String::new();
        }

        let html_alt: String;
        let html_caption: String;

        if let Some(a) = alt {
            html_alt = format!(r#"alt="{}""#, a);
        } else {
            html_alt = String::new();
        }

        if let Some(c) = caption {
            html_caption = format!(r#"<figcaption class="image-caption">{}</figcaption>"#, c);
        } else {
            html_caption = String::new();
        }

        let style_attr: String;

        if let Some(style) = style {
            style_attr = format!(r#"style="{}""#, style);
        } else {
            style_attr = String::new();
        }

        format!(r#"<figure class="figure" {}>
                    <img src="{}" {} class="{}" {} />
                    {}
                </figure>"#, id_attr, src, html_alt, img_classes.join(" "), style_attr, html_caption)
    }

    fn build_img(src: &str, alt: Option<&str>, caption: Option<&str>, id: Option<ResourceReference>, img_classes: Vec<&str>, figure_style: Option<String>, parsing_configuration: &RwLockReadGuard<ParsingConfiguration>) -> String {

        if RemoteResource::is_valid_remote_resource(src) {

            if parsing_configuration.embed_remote_image() {

                todo!()

            } else {
                
                let src = Url::parse(src).unwrap();

                return Self::create_figure_img(src.as_str(), alt, caption, id, img_classes, figure_style)
            }

        } else {

            let mut src_path_buf = PathBuf::from(src);

            if src_path_buf.is_relative() {

                let image_file_name = src;

                src_path_buf = parsing_configuration.input_location().clone().join(image_file_name);

                if !src_path_buf.exists() {

                    log::debug!("'{}' not found, try adding images directory path", src_path_buf.to_string_lossy());

                    src_path_buf = parsing_configuration.input_location().clone().join(dossier::ASSETS_DIR).join(dossier::IMAGES_DIR).join(image_file_name);
                }
            }

            if src_path_buf.exists() {
            
                let image = ImageResource::try_from(src_path_buf).unwrap();

                let base64_image = image.to_base64(parsing_configuration.compress_embed_image());

                return Self::create_figure_img(format!("data:image/png;base64,{}", base64_image.unwrap()).as_str(), alt, caption, id, img_classes, figure_style);

            } else if parsing_configuration.strict_image_src_check() {

                log::error!("{}", ParsingError::InvalidSource(String::from(src)));

                panic!("invalid src")

            } else {
                return Self::create_figure_img(src, alt, caption, id, img_classes, figure_style)       // create image tag of invalid image instead of panic
            }

        }

    }

    fn parse_image(search_pattern_regex: &Regex, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {

        if !search_pattern_regex.is_match(content) {
            return Err(ParsingError::InvalidSource(format!("'{}' do not match using: {}", content, search_pattern_regex)))
        }

        let parsed_content = search_pattern_regex.replace_all(content, |captures: &Captures| {
            
            if let Some(label) = captures.get(1) {

                if let Some(src) = captures.get(3) {

                    let style: Option<String>;

                    if let Some(_style) = captures.get(4) {
                        style = Some(String::from(_style.as_str()));
                    } else {
                        style = None;
                    }

                    let parsed_label = Parser::parse_text(codex, label.as_str(), Arc::clone(&parsing_configuration), Arc::new(None)).unwrap();

                    let parsing_configuration = parsing_configuration.read().unwrap();
                    let document_name = parsing_configuration.metadata().document_name().as_ref().unwrap();

                    if let Some(id) = captures.get(2) {
                        let id = ResourceReference::of_internal_without_sharp(id.as_str(), Some(document_name)).unwrap();

                        return Self::build_img(src.as_str(), Some(label.as_str()), Some(&parsed_label.parsed_content()), Some(id), vec!["image"], style, &parsing_configuration);

                    } else {
                        let id = ResourceReference::of(label.as_str(), Some(document_name)).unwrap();

                        return Self::build_img(src.as_str(), Some(label.as_str()), Some(&parsed_label.parsed_content()), Some(id), vec!["image"], style, &parsing_configuration);
 
                    }
                }
            }

            unreachable!()
            
        }).to_string();
        
        Ok(ParsingOutcome::new(parsed_content))
    }

    fn parse_abridged_image(search_pattern_regex: &Regex, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {

        let parsing_configuration = parsing_configuration.read().unwrap();

        let document_name = parsing_configuration.metadata().document_name().as_ref().unwrap();

        if !search_pattern_regex.is_match(content) {
            return Err(ParsingError::InvalidSource(format!("'{}' do not match using: {}", content, search_pattern_regex)))
        }

        let parsed_content = search_pattern_regex.replace_all(content, |captures: &Captures| {
            
            let src = captures.get(1).unwrap();

            let id: Option<ResourceReference>;

            if let Some(_id) = captures.get(2) {
                id = Some(ResourceReference::of_internal_without_sharp(_id.as_str(), Some(document_name)).unwrap());
            } else {
                id = None;
            }

            let style: Option<String>;

            if let Some(_style) = captures.get(3) {
                style = Some(String::from(_style.as_str()));
            } else {
                style = None;
            }

            return Self::build_img(src.as_str(), None, None, id, vec!["image", "abridged-image"], style, &parsing_configuration);

        }).to_string();
        
        Ok(ParsingOutcome::new(parsed_content))
    }

    fn parse_multi_image(search_pattern_regex: &Regex, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {

        let parsed_content = search_pattern_regex.replace_all(content, |captures: &Captures| {
            
            let justify_content: Option<String>;

            if let Some(jc) = captures.get(1) {
                justify_content = Some(String::from(jc.as_str()));
            } else {
                justify_content = None;
            }

            let raw_images = String::from(captures.get(2).unwrap().as_str());

            let images_container_style: String = format!("display: flex; justify-content: {};", justify_content.unwrap_or(String::from(DEFAULT_JUSTIFY_CONTENT)));
            let mut images_container = build_html::Container::new(build_html::ContainerType::Div)
                                                .with_attributes(vec![
                                                    ("style", images_container_style.as_str()),
                                                    ("class", "images-container")
                                                ]);

            for mut raw_image_line in raw_images.lines() {

                if raw_image_line.trim().is_empty() {
                    continue;
                }

                let align_self_captures = ALIGN_ITEM_PATTERN_REGEX.captures(raw_image_line);

                let align_self = match align_self_captures {
                    Some(ai) => {
                        raw_image_line = raw_image_line.strip_prefix(ai.get(0).unwrap().as_str()).unwrap();

                        ai.get(1).unwrap().as_str()
                    },
                    None => DEFAULT_ALIGN_SELF
                };

                let mut image_container = Container::new(build_html::ContainerType::Div)
                                                    .with_attributes(vec![
                                                        ("style", format!(r"align-self: {}", align_self).as_str()),
                                                        ("class", "image-container")
                                                    ]);

                for modifier in MULTI_IMAGE_PERMITTED_MODIFIER {
                    let parse_res = Self::parse_image_from_identifier(&modifier.identifier(), &Regex::new(&modifier.modifier_pattern()).unwrap(), raw_image_line, codex, Arc::clone(&parsing_configuration));

                    if let Ok(result) = parse_res {
                        image_container = image_container.with_raw(result.parsed_content());
                    }
                }

                images_container = images_container.with_container(image_container);
            }

            images_container.to_html_string()

        }).to_string();
        
        Ok(ParsingOutcome::new(parsed_content))
    }

    fn parse_image_from_identifier(image_modifier_identifier: &ModifierIdentifier, search_pattern_regex: &Regex, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {
        

        if image_modifier_identifier.eq(&StandardParagraphModifier::Image.identifier()) {
            return Self::parse_image(search_pattern_regex, content, codex, Arc::clone(&parsing_configuration));
        }

        if image_modifier_identifier.eq(&StandardParagraphModifier::AbridgedImage.identifier()) {
            return Self::parse_abridged_image(search_pattern_regex, content, codex, Arc::clone(&parsing_configuration));        
        }

        if image_modifier_identifier.eq(&StandardParagraphModifier::MultiImage.identifier()) {
            return Self::parse_multi_image(search_pattern_regex, content, codex, Arc::clone(&parsing_configuration))
        }

        log::error!("'{}' is unsupported image modifier identifier", image_modifier_identifier);

        panic!("unsupported image modifier identifier");
    }
}

impl ParsingRule for HtmlImageRule {

    fn search_pattern(&self) -> &String {
        &self.search_pattern
    }

    fn standard_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {

        Self::parse_image_from_identifier(&self.image_modifier_identifier, &self.search_pattern_regex, content, codex, Arc::clone(&parsing_configuration))
    }

    fn fast_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {
        Ok(ParsingOutcome::new(format!(r#"<img alt="{}" />"#, content)))
    }
    
    fn search_pattern_regex(&self) -> &Regex {
        &self.search_pattern_regex
    }
}

#[cfg(test)]
mod test {
    use crate::compiler::codex::codex_configuration::CodexConfiguration;

    use super::*;

    #[test]
    fn parse_all_in_one() {

        let img_src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test-resources").join("wikipedia-logo.png");

        let image_rule = HtmlImageRule::new(StandardParagraphModifier::Image.identifier());

        let nmd_text = format!(r"![image1]({})", img_src.as_os_str().to_string_lossy());

        let codex = Codex::of_html(CodexConfiguration::default());

        let parsed_content = image_rule.parse(nmd_text.as_str(), &codex, Arc::new(RwLock::new(ParsingConfiguration::default()))).unwrap();
        
        assert!(!parsed_content.parsed_content().is_empty())
    }
}