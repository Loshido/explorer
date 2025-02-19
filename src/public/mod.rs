use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use crate::{files::read_directory, html::build};

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
    let relative = Path::new("./data/public/");

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
                let folder = build::folder(relative.to_path_buf(), prefix.to_path_buf(), suffix, dir);

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