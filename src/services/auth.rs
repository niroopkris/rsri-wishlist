extern crate diesel;
extern crate rocket;

use crate::schema;
use crate::models::{User, UserDto, Wish, Session};

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post, FromForm};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};
use rocket::http::{CookieJar, Cookie};

use crate::services::establish_connection_pg;
use diesel::prelude::*;

//use flux_rs::*;
//use rdiesel::Expr;

//put all authorization stuff like create_user and logout into here

//create User 
#[post("/create_user", format = "form", data = "<user>")]
pub fn create_user(jar: &CookieJar<'_>, user: Form<UserDto>) -> Template {
    //create a new user and add it to the database. 
    //once they register, we need the user_id to be stored in the user session somewhere so we can get it anytime
    use schema::users::dsl::*;

    let connection = &mut establish_connection_pg();

    let new_user = UserDto {
        user_id: user.user_id.to_string(),
        user_name: user.user_name.to_string(),
        passwd: user.passwd.to_string(),
    };

    //add to database, no rdiesel since the user's Session hasn't been created yet
    diesel::insert_into(users)
        .values(new_user)
        .execute(connection)
        .expect("Error saving new user");

    let session_usr_id = user.user_id.to_string();
    jar.add(("user_id", session_usr_id.clone()));

    Template::render("wishes",  context!{})
}


//impl rdiesel::Expr<User, String> for schema::users::user_id {}

#[post("/login", data = "<user>")]
pub fn login(jar: &CookieJar<'_>, user: Form<UserDto>) -> Template {
    use self::schema::wish::wish_owner;
    use self::schema::users::user_id;
    use self::schema::users::passwd;

    let connection = &mut establish_connection_pg();

    // checks to see if user exists 

    /* NEW CODE - doesn't work since rdiesel now requires context

    let user_q1 = user_id.eq(user.user_id.to_string());
    let is_user = rdiesel::select_list(connection, user_q1);

    if is_user.expect("USER DOESN'T EXIST").is_empty() {
        Template::render("users", context! {})
    } else {
        let session_user_id = user.user_id.to_string();
        jar.add(("user_id", session_user_id.clone()));

        println!("{}", session_user_id);
    
        let results_q1 = wish_owner.eq(session_user_id);
        let results = rdiesel::select_list(connection, results_q1);
    
        Template::render("wishes", context! {wishes: &results.expect("ERROR LOADING WISHES")})    
    }
    */

    // OLD CODE - USE FOR NOW 

    // checks to see if user exists 
    let is_user = schema::users::dsl::users
        .filter(user_id.eq(user.user_id.to_string()))
        .load::<User>(connection)
        .expect("Error loading users");

    if is_user.is_empty() {
        Template::render("users", context! {})
    } else {
        let session_user_id = user.user_id.to_string();
        jar.add(("user_id", session_user_id.clone()));

        println!("{}", session_user_id);
    
        let results = schema::wish::dsl::wish
            .filter(wish_owner.eq(session_user_id))
            .load::<Wish>(connection)
            .expect("Error loading posts");
    
        Template::render("wishes", context! {wishes: &results})
    }
    
}

#[post("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Template {
    jar.remove("user_id");

    Template::render("users", context! {})
}