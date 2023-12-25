extern crate rocket;
extern crate log;

use rocket::{launch, routes};
use rocket_dyn_templates::Template;
mod services;
pub mod models;
pub mod schema;

#[launch]
fn rocket() -> _ {
    // env_logger::init();

    rocket::build()
        .mount("/", routes![services::portal])
        .mount("/", routes![services::create_user])
        .mount("/", routes![services::login])
        .mount("/", routes![services::create_wish])
        .attach(Template::fairing())
}
