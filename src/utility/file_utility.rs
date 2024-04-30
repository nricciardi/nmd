use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};


pub fn read_file_content(file_pathbuf: &PathBuf) -> Result<String, io::Error> {

    let mut file = File::open(file_pathbuf)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}


#[allow(dead_code)]
pub fn is_file_path(s: &str) -> bool {
    
    Path::new(s).is_absolute() || Path::new(s).is_relative()
}

pub fn create_directory(path: &PathBuf) -> Result<(), io::Error> {
   fs::create_dir(path)
}

pub fn create_empty_file(file_path: &PathBuf) -> Result<(), io::Error> {

    let mut file = File::create(&file_path)?;

    file.write_all(b"")
}

pub fn build_output_file_name(base: &str, ext: &str) -> String {
    format!("{}.{}", base, ext).replace(" ", "-").to_ascii_lowercase()
}