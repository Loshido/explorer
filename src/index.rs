use std::fs::read;

#[derive(Responder)]
pub enum Response {
    #[response(status = 200, content_type = "html")]
    HTML(String),
}

#[get("/")]
pub fn handler() -> Response {
    let bytes = read("./assets/template/login.html")
        .expect("Failed to load login.html template");

    let html = String::from_utf8(bytes).unwrap();

    Response::HTML(html)
}