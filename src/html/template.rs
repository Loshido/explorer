use std::fs::read;
use std::path::Path;

pub fn remap(html: &str, values: Vec<String>) -> String {
    let mut result = String::new();
    for i in 0..values.len() {
        result = html.replace(&format!("${}", i), &values.get(i).unwrap());
    }
    result
}

pub fn read_template(template: &str) -> String {
    let path = Path::new("./assets/template").join(template);
    if !path.exists() {
        panic!("Template {} doesn't exist in assets/template", template);
    }

    let bytes = read(path).expect("Failed to read the template");
    String::from_utf8(bytes)
        .expect("The template is not an html file!")
}