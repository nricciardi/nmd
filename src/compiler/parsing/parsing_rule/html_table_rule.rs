use std::{collections::HashMap, fmt::Debug, sync::{Arc, RwLock}};

use build_html::{Container, ContainerType, Html, HtmlContainer};
use regex::{Captures, Regex};

use crate::{compiler::{codex::modifier::{constants::IDENTIFIER_PATTERN, standard_paragraph_modifier::StandardParagraphModifier, standard_text_modifier::StandardTextModifier}, parsing::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome}}, resource::{resource_reference::ResourceReference, table::{self, Table, TableCell, TableCellAlignment}}};

use super::ParsingRule;


type TableMetadata = (Option<String>, Option<String>, Option<String>);



pub struct HtmlTableRule {
    searching_pattern: String
}

impl HtmlTableRule {
    pub fn new() -> Self {
        Self {
            searching_pattern: StandardParagraphModifier::Table.modifier_pattern()
        }
    }

    fn extract_table_row_content_from_line(line: &str) -> Option<Vec<String>> {
        if line.trim().is_empty() {
            return None;
        }

        let line = line.trim_start();

        if !line.starts_with('|') {
            return None;
        }

        let line = &line[1..];      // remove first |

        let mut row: Vec<String> = Vec::new();

        let cells: Vec<&str> = line.split("|").collect();
        let cells_n = cells.len();
        for (index, cell) in cells.iter().enumerate() {

            if index == cells_n - 1 {
                break;
            }

            row.push(String::from(*cell));
        }

        Some(row)
    }

    fn extract_table_alignments_from_row(row: &Vec<String>) -> Option<Vec<TableCellAlignment>> {

        let mut alignments = vec![TableCellAlignment::default(); row.len()];

        for (index, cell) in row.iter().enumerate() {
            let cell = cell.trim();

            if cell.starts_with(":-") && cell.ends_with("-:") {
                alignments[index] = TableCellAlignment::Center;
                continue;
            }
            
            if cell.starts_with(":-") && cell.ends_with("-") {
                alignments[index] = TableCellAlignment::Left;
                continue;
            }
            
            if cell.starts_with("-") && cell.ends_with("-:") {
                alignments[index] = TableCellAlignment::Right;
                continue;
            }

            return None;
        }

        Some(alignments)
    }

    fn build_row(row: &Vec<String>, alignments: &Vec<TableCellAlignment>) -> Vec<TableCell> {

        let mut cells: Vec<TableCell> = Vec::new();

        for (index, cell) in row.iter().enumerate() {

            if cell.is_empty() {

                cells.push(TableCell::None);

            } else {

                let mut align = alignments.get(index).unwrap_or(&TableCellAlignment::default()).clone();

                if cell.starts_with(":") && cell.ends_with(":") {
                    align = TableCellAlignment::Center
                }
                
                if cell.starts_with(":") && !cell.ends_with(":") {
                    align = TableCellAlignment::Left
                }

                if !cell.starts_with(":") && cell.ends_with(":") {
                    align = TableCellAlignment::Right
                }

                cells.push(TableCell::ContentCell { content: String::from(cell), alignment: align});
            }
        }

        cells
    }

    fn load_html_row(html_row: &mut Container, cells: &Vec<TableCell>) {

        for cell in cells {
            match cell {
                TableCell::None => {
                    let html_cell = Container::new(ContainerType::Div)
                                                            .with_attributes(vec![
                                                                ("class", "table-cell table-empty-cell")
                                                            ]);

                    html_row.add_container(html_cell);
                },
                TableCell::ContentCell { content, alignment } => {

                    let align_class = match alignment {
                        TableCellAlignment::Left => String::from("table-left-cell"),
                        TableCellAlignment::Center => String::from("table-center-cell"),
                        TableCellAlignment::Right => String::from("table-right-cell"),
                    };

                    let html_cell = Container::new(ContainerType::Div)
                                                            .with_attributes(vec![
                                                                ("class", format!("table-cell {}", align_class).as_str())
                                                            ])
                                                            .with_raw(content);

                    html_row.add_container(html_cell);
                },
            }
        }
    }

    fn extract_table_metadata(s: &str, document_name: &str) -> TableMetadata {
        let regex = Regex::new(&format!(r"(?:\[\((.*)\)\])?(?:{})?(?:\{{(.*)\}})?", IDENTIFIER_PATTERN)).unwrap();

        let captures = regex.captures(s);

        if captures.is_none() {
            log::warn!("invalid table metadata: '{}'", s);
            return (None, None, None);
        }

        let captures = captures.unwrap();

        let mut caption: Option<String> = None;
        let mut id: Option<String> = None;
        let mut style: Option<String> = None;

        if let Some(_caption) = captures.get(1) {
            caption = Some(_caption.as_str().to_string());
        }

        if let Some(_id) = captures.get(2) {
            id = Some(ResourceReference::of_internal_without_sharp(_id.as_str(), Some(document_name)).unwrap().build());
        }

        if let Some(_style) = captures.get(3) {
            style = Some(_style.as_str().to_string());
        }


        (id, caption, style)
    }

    fn build_html_table(caption: Option<String>, id: Option<String>, style: Option<String>, table: Table) -> String {

        let mut html_table_attrs: Vec<(String, String)> = vec![(String::from("class"), String::from("table"))];

        if let Some(id) = id {

            html_table_attrs.push((String::from("id"), String::from(id)));
        }

        if let Some(style) = style {
            html_table_attrs.push((String::from("style"), String::from(style.as_str())));
        }

        let mut html_table = Container::new(ContainerType::Div)
                                        .with_attributes(html_table_attrs);

        if let Some(header_cells) = table.header() {

            let mut html_table_header = Container::new(ContainerType::Div)
                                                .with_attributes(vec![
                                                    ("class", "table-header")
                                                ]);
            
            Self::load_html_row(&mut html_table_header, header_cells);
            
            html_table.add_container(html_table_header);
        }

        let mut html_table_body = Container::new(ContainerType::Div)
                                                    .with_attributes(vec![
                                                        ("class", "table-body")
                                                    ]);
        
        for row in table.body() {

            let mut html_body_row = Container::new(ContainerType::Div)
                                                            .with_attributes(vec![
                                                                ("class", "table-body-row")
                                                            ]);

            Self::load_html_row(&mut html_body_row, row);

            html_table_body.add_container(html_body_row);
        }

        html_table.add_container(html_table_body);

        if let Some(footer_cells) = table.footer() {
            let mut html_table_footer = Container::new(ContainerType::Div)
            .with_attributes(vec![
                ("class", "table-footer")
            ]);

            Self::load_html_row(&mut html_table_footer, footer_cells);

            html_table.add_container(html_table_footer);
        }

        if let Some(c) = caption {

            let html_caption = Container::new(ContainerType::Div)
                                                .with_attributes(vec![
                                                    ("class", "table-caption")
                                                ])
                                                .with_raw(c);

            html_table.add_container(html_caption);
        }

        html_table.to_html_string()
    }
}

impl Debug for HtmlTableRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HtmlTableRule").field("searching_pattern", &self.searching_pattern).finish()
    }
}

impl ParsingRule for HtmlTableRule {
    fn searching_pattern(&self) -> &String {
        &self.searching_pattern
    }

    fn parse(&self, content: &str, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {

        //return Ok(ParsingOutcome::new(format!("table: {}", content.lines().nth(2).unwrap())));

        let mut table: Table = Table::new();
        let mut alignments: Option<Vec<TableCellAlignment>> = None;
        let mut max_row_len: usize = 0;
        let mut there_is_header: bool = false;
        let mut id: Option<String> = None;
        let mut caption: Option<String> = None;
        let mut style: Option<String> = None;
        

        for (index, line) in content.lines().enumerate() {

            // check if there is caption
            let trim_line = line.trim_start();
            if trim_line.starts_with("[") || trim_line.starts_with("{") || trim_line.starts_with("#") {

                if let Ok(pc) = parsing_configuration.read() {

                    let document_name = pc.metadata().document_name().as_ref().unwrap();
                    
                    (caption, id, style) = Self::extract_table_metadata(line, document_name);

                    if id.is_none() && caption.is_some() {
                        id = Some(ResourceReference::of_internal_without_sharp(&caption.clone().unwrap(), Some(document_name)).unwrap().build());
                    }
                }

            }

            let row = Self::extract_table_row_content_from_line(line);

            if row.is_none() {
                continue;
            }

            let row = row.unwrap();

            max_row_len = max_row_len.max(row.len());

            if alignments.is_none() {
                alignments = Some(vec![TableCellAlignment::default(); max_row_len])
            }

            if let Some(mut aligns) = Self::extract_table_alignments_from_row(&row) {

                if table.body().len() == 1 {
                    there_is_header = true;
                }

                while aligns.len() < max_row_len {
                    aligns.push(TableCellAlignment::default());
                }

                alignments = Some(aligns);
                
                continue;
            }

            let row = Self::build_row(&row, alignments.as_ref().unwrap());

            table.append_to_body(row);
        }

        if there_is_header {
            table.shift_first_body_row_to_header();
        }

        // check if there is footer
        if table.body().len() > 2 {

            let second_last_row = table.body().get(table.body().len() - 2).unwrap();

            if  second_last_row.len() == 1 {

                let first_cell = second_last_row.get(0).unwrap();

                match first_cell {
                    TableCell::None => (),
                    TableCell::ContentCell { content, alignment: _ } => {
                        if content.chars().all(|c| c.eq(&'-')) {
                            table.shift_last_body_row_to_footer()
                        }
                    },
                }                
            }
        }

        
        Ok(ParsingOutcome::new(Self::build_html_table(caption, id, style, table)))
    }
}
