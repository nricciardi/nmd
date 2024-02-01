use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};


pub fn read_file_content(file_pathbuf: &PathBuf) -> Result<String, io::Error> {

    let mut file = File::open(file_pathbuf)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

pub fn is_file_path(s: &str) -> bool {
    
    Path::new(s).is_absolute() || Path::new(s).is_relative()
}