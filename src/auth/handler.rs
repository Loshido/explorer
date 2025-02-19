use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use super::fs::passwd;

#[derive(FromForm, Debug)]
pub struct AuthAttempt {
    username: String,
    password: String,
}

#[derive(Responder)]
pub enum LoginResponse {
    #[response(status = 302)]
    Ok(Redirect),
    #[response(status = 303)]
    No(Redirect),
}

fn failed() -> LoginResponse {
    return LoginResponse::No(
        Redirect::to(uri!("/?failed"))
    )
} 

fn success() -> LoginResponse {
    return LoginResponse::Ok(
        Redirect::to(uri!("/private")))
}

#[post("/login", data = "<auth>")]
pub fn post(auth: Form<AuthAttempt>, cookies: &CookieJar) -> LoginResponse {
    let mut users = passwd();
    let cookie = cookies.get("token");
    if let Some(token) = cookie {
        let parse = token.value().parse::<u64>();
        if parse.is_ok() {
            let hash = parse.unwrap();
            if users.0.iter().any(|v| v.auth(hash)) {
                return success()
            }
        }
    }

    if let Some(user) = users.exists(&auth.username) {
        let token = user.login(&auth.username, &auth.password);

        return match token {
            Some(v) => {
                users.save();
                let mut cookie = Cookie::new("token", v.to_string());
                cookie.make_permanent();
                cookies.add(cookie);

                success()
            },
            _ => failed()
        }
    }
    failed()
}