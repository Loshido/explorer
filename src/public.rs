use std::path::{Path, PathBuf};

use rocket::{fs::NamedFile, serde::json::Json};

use crate::files::{Directory, read_directory};

#[derive(Responder)]
pub enum PublicResponse {
    #[response(status = 200, content_type = "json")]
    Dir(Json<Directory>),
    #[response(status = 200)]
    File(Option<NamedFile>),
    #[response(status = 404)]
    NotFound(String)
}

#[get("/public/<suffix..>")]
pub async fn handler(suffix: PathBuf) -> PublicResponse {
    let prefix = Path::new("./data/public/");
    let path = prefix.join(suffix);

    match path.to_str() {
        Some(v) => {
            if !path.exists() {
                return PublicResponse::NotFound(format!("{} doesn't exist!", v))
            } else if path.is_dir() {
                let dir = read_directory(path);
                
                return PublicResponse::Dir(Json(dir))
            }
            PublicResponse::File(
                NamedFile::open(v).await.ok()
            )
        },
        _ => PublicResponse::NotFound(String::from("Not found"))
    }
}