use std::{path::{Path, PathBuf}, env};
use rocket::{fs::NamedFile, http::CookieJar, response::Redirect};
use crate::{html::build, auth::fs::{passwd, read_directory}};

#[derive(Responder)]
pub enum PrivateResponse {
    #[response(status = 200, content_type = "html")]
    Dir(String),
    #[response(status = 200)]
    File(Option<NamedFile>),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 303)]
    Unauthorized(Redirect)
}

fn unauthorized() -> PrivateResponse {
    return PrivateResponse::Unauthorized(
        Redirect::to(uri!("/"))
    )
}

#[get("/private/<suffix..>")]
pub async fn handler<'u>(suffix: PathBuf, cookie: &CookieJar<'u>) -> PrivateResponse {
    let token = cookie.get("token");
    if token.is_none() {
        // Should have a cookie
        return unauthorized();
    } else {
        let hash = match token.unwrap().value().parse::<u64>() {
            Ok(v) => v,
            // should be a u64
            _ => return unauthorized()
        };

        let users = passwd();

        match users.0.iter().any(|v| v.auth(hash)) {
            // need a match
            false => return unauthorized(),
            true => ()
        }
    }
    
    let explorer_data = env::var("EXPLORER_DATA").unwrap_or(String::from("./data"));
    let relative = Path::new(explorer_data.as_str());
    let prefix = Path::new("/private/");
    let path = relative.join(suffix.clone());

    match path.to_str() {
        Some(v) => {
            if !path.exists() {
                // Page d'erreur
                return PrivateResponse::NotFound(format!("{} doesn't exist!", v))
            } else if path.is_dir() {
                // Ouvre la page du dossier
                let dir = read_directory(
                    path.to_path_buf()
                );
                let folder = build::folder(prefix.to_path_buf(), suffix, dir);

                return PrivateResponse::Dir(folder)
            }
            // Ouvrir le fichier
            PrivateResponse::File(
                NamedFile::open(v).await.ok()
            )
        },
        // Page d'erreur
        _ => PrivateResponse::NotFound(String::from("Not found"))
    }
}