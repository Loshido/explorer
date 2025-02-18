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
    #[response(status = 200)]
    Ok(Redirect),
    No(Redirect),
}

#[post("/login", data = "<auth>")]
pub fn post(auth: Form<AuthAttempt>, cookies: &CookieJar) -> LoginResponse {
    let mut users = passwd();

    if let Some(user) = users.exists(&auth.username) {
        let token = user.login(&auth.username, &auth.password);

        return match token {
            Some(v) => {
                users.save();
                let mut cookie = Cookie::new("token", v.to_string());
                cookie.make_permanent();
                cookies.add(cookie);

                LoginResponse::Ok(Redirect::to(uri!("/private")))
            },
            _ => LoginResponse::No(Redirect::to(uri!("/?failed")))
        }
    }
    LoginResponse::No(Redirect::to(uri!("/?failed")))
}