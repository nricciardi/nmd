use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};


/// Return entirely file content 
pub fn read_file_content(file_path_buf: &PathBuf) -> Result<String, io::Error> {

    let mut file = File::open(file_path_buf)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}


/// Return true if &str passed is a valid file path
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

/// Generate a new file name String using passed base and extension arguments
pub fn build_output_file_name(base: &str, ext: &str) -> String {
    format!("{}.{}", base, ext).replace(" ", "-").to_ascii_lowercase()
}