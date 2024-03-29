use std::{path::PathBuf, sync::Arc};

use log;
use regex::{Regex, Captures};

use crate::compiler::dossier;
use crate::compiler::parsable::codex::Codex;
use crate::resource::{image::Image, remote_resource::RemoteResource};
use crate::compiler::parsable::ParsingConfiguration;

use super::{Modifier, ParsingRule, parsing_outcome::{ParsingOutcome, ParsingError}};


/// Rule to replace a NMD text based on a specific pattern matching rule
pub struct HtmlImageRule {
}

impl HtmlImageRule {
    
    pub fn new() -> Self {
        Self {}
    }

    fn create_img_tag(src: &str, label: &str) -> String {

        let id = Codex::create_id(label);

        format!(r#"<figure class="figure" id="{}">
                    <img src="{}" alt="{}" class="image" />
                    <figcaption class="image-caption">{}</figcaption>
                </figure>"#, id, src, label, label)
    }
}

impl ParsingRule for HtmlImageRule {

    /// Parse the content using internal search and replacement pattern
    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {

        let regex = match Regex::new(&self.modifier().search_pattern()) {
            Ok(r) => r,
            Err(_) => return Err(ParsingError::InvalidPattern(self.modifier().search_pattern()))  
        };

        let parsed_content = regex.replace_all(content, |captures: &Captures| {
            
            if let Some(label) = captures.get(1) {
                if let Some(src) = captures.get(2) {

                    let src = src.as_str();

                    if RemoteResource::is_valid_remote_resource(src) {

                        if parsing_configuration.embed_remote_image() {

                            todo!()

                        } else {
                            return Self::create_img_tag(src, label.as_str())
                        }

                    } else {

                        let mut src_path_buf = PathBuf::from(src);

                        if src_path_buf.is_relative() {
                            src_path_buf = parsing_configuration.input_location().clone().join(src_path_buf);

                            if !src_path_buf.exists() {

                                log::debug!("'{}' not found, try adding images directory path", src_path_buf.to_string_lossy());

                                src_path_buf = parsing_configuration.input_location().clone().join(dossier::ASSETS_DIR).join(dossier::IMAGES_DIR);
                            }
                        }

                        if src_path_buf.exists() {
                        
                            let mut image = Image::try_from(src_path_buf).unwrap();

                            if parsing_configuration.compress_embed_image() {
                                image.compress().unwrap();
                            }
    
                            let base64_image = image.to_base64();

                            return Self::create_img_tag(format!("data:image/png;base64,{}", base64_image.unwrap()).as_str(), label.as_str());

                        } else if parsing_configuration.strict_image_src_check() {

                            log::error!("{}", ParsingError::InvalidSource(String::from(src)));

                            panic!("invalid src")

                        } else {
                            return Self::create_img_tag(src, label.as_str())
                        }

                    }

                    // return Self::get_standard_img_tag(src, label.as_str())
                }
            }

            captures.get(0).unwrap().as_str().to_string()

        }).to_string();
        
        Ok(ParsingOutcome::new(parsed_content))
    }

    fn modifier(&self) -> &Modifier {
        &Modifier::Image
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_all_in_one() {

        let img_src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test-resources").join("wikipedia-logo.png");

        let image_rule = HtmlImageRule::new();

        let nmd_text = format!(r"![image1]({})", img_src.as_os_str().to_string_lossy());
        let result = r#"<img src="data:image/png;base64,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" alt="image1" class="img" />"#;

        let parsed_content = image_rule.parse(nmd_text.as_str(), Arc::new(ParsingConfiguration::default())).unwrap();

        assert_eq!(parsed_content.parsed_content(), result)
    }
}