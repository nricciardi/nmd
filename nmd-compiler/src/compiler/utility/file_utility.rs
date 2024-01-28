use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;


pub fn read_file_content(file_pathbuf: &PathBuf) -> Result<String, io::Error> {

    let mut file = File::open(file_pathbuf)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}