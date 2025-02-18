#[macro_use] extern crate rocket;

mod public;
mod html;
pub mod files;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, public::handler])
}