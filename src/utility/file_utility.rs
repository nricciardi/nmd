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

    create_file_with_content(file_path, "")
}

pub fn create_file_with_content(file_path: &PathBuf, content: &str) -> Result<(), io::Error> {

    let mut file = File::create(&file_path)?;

    file.write_all(content.as_bytes())
}

/// Generate a new file name String using passed base and extension arguments
pub fn build_output_file_name(base: &str, ext: Option<&str>) -> String {

    let base: Vec<char> = base.chars()
                                .filter(|c| c.is_alphanumeric() || c.eq(&'_') || c.eq(&'-') || c.eq(&' ') || c.eq(&'.'))
                                .map(|c| c.to_ascii_lowercase())
                                .collect();

    let base = String::from_iter(base);

    let base: Vec<char> = base.trim().chars().map(|c| {
                                    if c.eq(&' ') {
                                        return '-';
                                    }

                                    c
                                })
                                .collect();

    let base = String::from_iter(base);

    if let Some(ext) = ext {

        return format!("{}.{}", base, ext);

    } else {

        return base;
    }
}

pub fn all_files_in_dir(dir_path: &PathBuf, exts: &Vec<String>) -> Result<Vec<PathBuf>, io::Error> {
    if !dir_path.is_dir() {

        let e = format!("{:?} must be a directory", dir_path);

        return Err(io::Error::new(io::ErrorKind::NotFound, e));
    }

    let mut files: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        if let Ok(entry) = entry {
            let path = entry.path();
            
            if let Some(extension) = path.extension() {

                for ext in exts {
                    if extension.to_string_lossy().eq(ext) {
                        if let Some(_) = path.file_name() {
                            files.push(path);
                            break;
                        }
                    }
                }
            }
        }
    }   

    Ok(files)
}