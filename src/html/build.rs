use std::{path::PathBuf, str::from_utf8};

use crate::{files::Directory, html::template::HTML};

pub fn folder(path: PathBuf, directory: Directory) -> String {
    assert!(path.is_dir());

    // Name of the current folder
    let name = path.file_name()
        .expect("Should have a name")
        .to_str().unwrap();
    
    let mut path_template = String::new();
    let path_str = path.to_str().unwrap();
    let mut stripped_path = path_str.strip_prefix("./data/").unwrap().to_string();
    stripped_path.push_str("/");
    let subpath = stripped_path.as_bytes();

    let mut i = 0;
    for j in 0..subpath.len() {
        if subpath[j] != b'/' {
            continue;
        }

        let lpath = &subpath[0..j];
        let name = &subpath[i..j];
        if name.len() == 0 {
            continue;
        }
        let template = HTML::path(
            from_utf8(name).unwrap(), 
            from_utf8(lpath).unwrap()
        );
        path_template += &template;
        i = j + 1;
    }

    let mut files = String::new();
    for (name, is_directory) in directory.0.iter() {
        if *is_directory {
            let folder = HTML::folder(
                &name, 
                &format!("/{}{}", stripped_path, name)
            );

            files += &folder;
        } else {
            let file = HTML::file(
                &name, 
                &format!("/{}{}", stripped_path, name)
            );

            files += &file;
        }
    }

    HTML::folder_page(name, path_template, files)
}