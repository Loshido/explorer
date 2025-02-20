use std::{path::{Path, PathBuf}, env};
use rocket::fs::NamedFile;
use crate::{auth::fs::read_directory, html::build};

#[derive(Responder)]
pub enum PublicResponse {
    #[response(status = 200, content_type = "html")]
    Dir(String),
    #[response(status = 200)]
    File(Option<NamedFile>),
    #[response(status = 404)]
    NotFound(String)
}

#[get("/public/<suffix..>")]
pub async fn handler(suffix: PathBuf) -> PublicResponse {
    // relative path to get files from the disk
    let explorer_data = env::var("EXPLORER_DATA").unwrap_or(String::from("./data"));
    let relative = Path::new(explorer_data.as_str()).join("public");

    // Prefix for redirection
    let prefix = Path::new("/public/");
    let path = relative.join(suffix.clone());

    match path.to_str() {
        Some(v) => {
            if !path.exists() {
                // Page d'erreur
                return PublicResponse::NotFound(format!("{} doesn't exist!", v))
            } else if path.is_dir() {
                // Ouvre la page du dossier
                let dir = read_directory(
                    path.to_path_buf()
                );
                let folder = build::folder(prefix.to_path_buf(), suffix, dir);

                return PublicResponse::Dir(folder)
            }
            // Ouvrir le fichier
            PublicResponse::File(
                NamedFile::open(v).await.ok()
            )
        },
        // Page d'erreur
        _ => PublicResponse::NotFound(String::from("Not found"))
    }
}