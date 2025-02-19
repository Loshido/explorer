#[macro_use] extern crate rocket;

mod index;
mod public;
mod private;
mod auth;
mod import;

mod html;

use rocket::fs::FileServer;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index::handler, 
            public::handler, 
            private::handler,
            import::handler,
            auth::handler::post
        ])
        .mount("/assets", FileServer::from("./assets"))
}