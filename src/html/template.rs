use std::fs::read;
use std::path::Path;

// put values inside template.
pub fn remap(html: &str, values: Vec<String>) -> String {
    let mut result = html.to_string();
    for i in 0..values.len() {
        result = result.replace(&format!("${}", i + 1), &values.get(i).unwrap());
    }
    result
}

// read a template from path.
pub fn read_template(template: &str) -> String {
    let path = Path::new("./assets/template").join(template);
    if !path.exists() {
        panic!("Template {} doesn't exist in assets/template", template);
    }

    let bytes = read(path).expect("Failed to read the template");
    String::from_utf8(bytes)
        .expect("The template is not an html file!")
}

pub struct HTML;

impl HTML {
    pub fn path(name: &str, link: &str) -> String {
        let template = read_template("path.html");

        remap(&template, vec![
            link.to_string(),
            name.to_string(),
        ])
    }
    pub fn file(name: &str, link: &str) -> String {
        let template = read_template("file.html");
    
        remap(&template, vec![
            link.to_string(),
            name.to_string(),
        ])
    }
    pub fn folder(name: &str, link: &str) -> String {
        let template = read_template("folder.html");
        
        remap(&template, vec![
            link.to_string(),
            name.to_string(),
            ])
        }
    pub fn folder_page(title: &str, paths: String, input: String, files: String) -> String {
        let template = read_template("folder_page.html");
    
        remap(&template, vec![
            title.to_string(),
            paths,
            input,
            files
        ])
    }
}