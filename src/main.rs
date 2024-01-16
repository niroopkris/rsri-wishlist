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
        .mount("/", routes![services::home])
        .mount("/", routes![services::create_user])
        .mount("/", routes![services::login])
        .mount("/", routes![services::logout])
        .mount("/", routes![services::create_wish])
        .mount("/", routes![services::delete_wish])
        //.mount("/", routes![services::edit_wish])
        .mount("/", routes![services::get_wishes])
        .mount("/", routes![services::get_friendships])
        .mount("/", routes![services::create_friendship_request])
        .mount("/", routes![services::change_friendship_status])
        .attach(Template::fairing())
}