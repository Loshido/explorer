mod template;
mod build;

use template::{remap, read_template};

pub struct HTML;

impl HTML {
    fn path(name: &str, link: &str) -> String {
        let template = read_template("path.html");

        remap(&template, vec![
            link.to_string(),
            name.to_string(),
        ])
    }
    fn file(name: &str, link: &str) -> String {
        let template = read_template("file.html");
    
        remap(&template, vec![
            link.to_string(),
            name.to_string(),
        ])
    }
    fn folder(name: &str, link: &str) -> String {
        let template = read_template("folder.html");
    
        remap(&template, vec![
            name.to_string(),
            link.to_string()
        ])
    }
    fn folder_page(title: &str, paths: String, files: String) -> String {
        let template = read_template("folder_page.html");
    
        remap(&template, vec![
            title.to_string(),
            paths,
            files
        ])
    }
}