use std::path::{Path, PathBuf};

use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::tokio::io::AsyncReadExt;

use crate::auth::fs::passwd;

#[derive(Responder)]
pub enum ImportResponse {
    #[response(status = 200)]
    Ok(Redirect),
    #[response(status = 303)]
    No(Redirect)
}

#[derive(FromForm)]
pub struct Upload<'r> {
    file: TempFile<'r>,
}

fn failed() -> ImportResponse {
    return ImportResponse::No(
        Redirect::to(uri!("/?failed"))
    )
}


#[post("/import/<path..>", data = "<upload>")]
pub async fn handler(path: PathBuf, cookies: &CookieJar<'_>, upload: Form<Upload<'_>>) -> ImportResponse {
    println!("import for {:?}", path);

    let cookie = cookies.get("token");
    if let Some(token) = cookie {
        let parse = token.value().parse::<u64>();
        if parse.is_ok() {
            let users = passwd();
            let hash = parse.unwrap();

            if users.0.iter().any(|v| v.auth(hash)) {
                println!("authentificated!");
                let result = upload.file.open().await;
                println!("file opened");
                
                let filename = "aaa.png";   
                if result.is_ok() {
                    println!("file uploaded & filename provided");
                    let filepath = Path::new("./data/import").join(filename);
                    let mut buf = Vec::new();
                    result.unwrap().read_buf(&mut buf).await.ok();
                    println!("result inbuffered");
                    std::fs::write(filepath, &buf).unwrap();
                    println!("result wrote");
                    return ImportResponse::Ok(
                        Redirect::to(uri!("/"))
                    )
                }
            }
        }
    }
    return failed()
}