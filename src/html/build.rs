use std::path::PathBuf;

use crate::{files::Directory, html::HTML};

fn folder(path: PathBuf, directory: Directory) {
    assert!(path.is_dir());

    let name = path.file_name()
        .expect("Should have a name")
        .to_str().unwrap();

    let mut path_templates: Vec<String> = Vec::new();

    for p in path.iter() {
        let subpath = p.to_str().unwrap();

        let template = HTML::path(subpath, "fleemme");
        path_templates.push(template);
    }
    let path_template = path_templates.join("\n");
    
}