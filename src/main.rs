#[macro_use] extern crate rocket;

mod public;
mod html;
mod index;
pub mod files;

use rocket::fs::FileServer;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index::handler, public::handler])
        .mount("/assets", FileServer::from("./assets"))
}