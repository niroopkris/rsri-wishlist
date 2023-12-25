extern crate diesel;
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post, FromForm};
use crate::models;
use crate::models::UserDto; 
use crate::models::WishDto; 
//use crate::models::FriendshipDto; 
use crate::models::UserSession; 

use crate::schema;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};
use std::env;

use rocket::{Request, Response};
use rocket::request::{FromRequest, Outcome};
use rocket::http::{CookieJar, Cookie};

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

//User home page
#[get("/portal")]
pub fn portal() -> Template {
    use self::models::User;
    //let connection = &mut establish_connection_pg();
    //let results = self::schema::users::dsl::users
        //.load::<User>(connection)
        //.expect("Error loading user");
    Template::render("users",  context!{})
}

//create User 
#[post("/register", format = "form", data = "<user>")]
pub fn create_user(jar: &CookieJar<'_>, user: Form<UserDto>) -> Template {
    //create a new user and add it to the database. 
    //once they register, we need the user_id to be stored in the user session somewhere so we can get it anytime
    use self::schema::users::dsl::*;
    use crate::models::UserDto;
    let connection = &mut establish_connection_pg();


    let new_user = UserDto {
        user_id: user.user_id.to_string(),
        user_name: user.user_name.to_string(),
        passwd: user.passwd.to_string(),
    };

    diesel::insert_into(users)
        .values(new_user)
        .execute(connection)
        .expect("Error saving new user");

    let session_usr_id = user.user_id.to_string();
    println!("Your user_id: {}", session_usr_id);
    jar.add(("user_id", session_usr_id.clone()));

    Template::render("wishes", context! {})
}

#[post("/login", data = "<user>")]
pub fn login(jar: &CookieJar<'_>, user: Form<UserDto>) -> Template {
    use self::models::Wish;
    use self::models::User;
    use self::schema::wish::wish_owner;

    //add if/else: check that user exists and load posts, otherwise return userportal template
    //let isUser = self::schema::users::dsl::users
    //  .filter(user_id.eq(session_usr_id))

    let session_usr_id = user.user_id.to_string();
    jar.add(("user_id", session_usr_id.clone()));
        
    let connection = &mut establish_connection_pg();
    let results = self::schema::wish::dsl::wish
        .filter(wish_owner.eq(session_usr_id))
        .load::<Wish>(connection)
        .expect("Error loading posts");

    Template::render("wishes", context! {wishes: &results})
}

//CRUD functions for a User's posts
#[post("/create_wish", format = "form", data = "<a_wish>")]
pub fn create_wish(a_wish: Form<WishDto>, usrSession: UserSession) -> Template {
    use self::schema::wish::dsl::*;
    use crate::models::WishDto;
    use self::models::Wish;

    let connection = &mut establish_connection_pg();

    let usr_token = &usrSession.usr_token;  //user id taken from our usrSession parameter (a cookie thing)

    let new_wish = WishDto {
        wish_owner: usr_token.to_string(),
        title: a_wish.title.to_string(),
        descr: a_wish.descr.to_string(),
        access_level: a_wish.access_level.to_string(),
    };

    diesel::insert_into(wish)
        .values(new_wish)
        .execute(connection)
        .expect("Error saving new wish");

    let results = self::schema::wish::dsl::wish
        .filter(wish_owner.eq(usr_token))
        .load::<Wish>(connection)
        .expect("Error loading posts");
    
    Template::render("wishes", context! {wishes: &results})
   // list();
}