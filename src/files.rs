use std::path::PathBuf;
use serde::Serialize;


#[derive(Serialize)]
pub struct Directory(Vec<(String, bool)>);

pub fn read_directory(path: PathBuf) -> Directory {
    let mut directory: Vec<(String, bool)> = Vec::new();
    
    for entry in path
        .read_dir()
        .expect(&format!("reading directory {:?} failed", path)) {
        if let Ok(path) = entry {
            let p = path.path()
                .to_str().unwrap().to_string(); 
            directory.push((
                p, 
                path.path().is_dir()
            ));
        }
    }
    
    Directory(directory)
}