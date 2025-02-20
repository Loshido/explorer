use std::{env, path::{Path, PathBuf}};

use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::CookieJar;
use rocket::response::Redirect;

use crate::auth::fs::passwd;

#[derive(Responder)]
pub enum ImportResponse {
    #[response(status = 200)]
    Ok(Redirect),
    #[response(status = 303)]
    No(Redirect)
}

fn failed() -> ImportResponse {
    return ImportResponse::No(
        Redirect::to(uri!("/?failed"))
    )
}

#[derive(FromForm)]
pub struct Upload<'f> {
    file: TempFile<'f>
}

#[post("/import/<path..>", data = "<form>")]
pub async fn handler(path: PathBuf, cookies: &CookieJar<'_>, form: Form<Upload<'_>>) -> ImportResponse {
    let cookie = cookies.get("token");
    if let Some(token) = cookie {
        let parse = token.value().parse::<u64>();
        if parse.is_ok() {
            let users = passwd();
            let hash = parse.unwrap();

            if users.0.iter().any(|v| v.auth(hash)) {
                
                let mut filename = form.file.name()
                    .unwrap_or("imported").to_string();
                let content_type = form.file.content_type();
                if let Some(content_type) = content_type {
                    if let Some(ext) = content_type.extension() {
                        filename = format!("{}.{}", filename, ext);
                    }
                }

                let explorer_data = env::var("EXPLORER_DATA").unwrap_or(String::from("./data"));
                let relative = Path::new(explorer_data.as_str());
                let filepath = relative
                    .join(path)
                    .join(filename);
    
                if form.file.path().is_none() {
                    return failed();
                }
                match std::fs::copy(form.file.path().unwrap(), filepath) {
                    Ok(_) => {
                        return ImportResponse::Ok(
                            Redirect::to(uri!("/private/"))
                        )
                    },
                    Err(..) => println!("couldn't copy due to: {:?}", ..)
                }
                return failed()
            }
        }
    }
    return failed()
}