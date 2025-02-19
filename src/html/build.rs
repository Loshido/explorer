use std::path::PathBuf;

use crate::{auth::Directory, html::template::HTML};

// relative + path -> ./data/public/..suffix
// prefix + path -> /public/..suffix 
pub fn folder(prefix: PathBuf, path: PathBuf, directory: Directory) -> String {

    // Name of the current folder
    let combined = prefix.join(&path);
    let name = combined
        .file_name().expect("should have a valid name")
        .to_str().unwrap();
    
    // Each directorie name and link
    let mut path_template = HTML::path(
        prefix
            .file_name().unwrap()
            .to_str().unwrap(), 
        prefix.to_str().unwrap()
    );

    let mut subpath = String::new();
    for slice in path.iter() {
        let name = slice.to_str().unwrap().to_string();
        if subpath.len() == 0 {
            subpath = name.clone();
        } else {
            subpath += &format!("/{}", &name);
        }

        let template = HTML::path(
            &name, 
            prefix.join(&subpath).to_str().unwrap()
        );

        path_template += &template;
    }

    // Add a link for each file/directory
    let mut files = String::new();
    for (name, is_directory) in directory.0.iter() {
        let mut combined = prefix.join(&path);
        combined.push(name);
        if *is_directory {
            let folder = HTML::folder(
                &name,
                combined.to_str().unwrap(), 
            );
            
            files += &folder;
        } else {
            let file = HTML::file(
                &name, 
                combined.to_str().unwrap()
            );

            files += &file;
        }
    }

    HTML::folder_page(name, path_template, path.to_str().unwrap().to_string(), files)
}