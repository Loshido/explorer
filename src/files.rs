use std::path::PathBuf;
use serde::Serialize;

#[derive(Serialize)]
pub struct Directory(pub Vec<(String, bool)>);

pub fn read_directory(path: PathBuf) -> Directory {
    let mut directory: Vec<(String, bool)> = Vec::new();
    
    for entry in path
        .read_dir()
        .expect(&format!("reading directory {:?} failed", path)) {
        if let Ok(entry_path) = entry {
            let p = entry_path.path()
                .file_name().unwrap()
                .to_str().unwrap()
                .to_string(); 


            directory.push((
                p, 
                entry_path.path().is_dir()
            ));
        }
    }
    
    Directory(directory)
}