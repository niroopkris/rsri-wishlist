extern crate diesel;
extern crate rocket;

use crate::schema;
use crate::schema::{users, wish, friendship};
use crate::models::{User, UserDto, Wish, Session};

use rocket::{get, post, FromForm};
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};
use rocket::http::{CookieJar, Cookie};

use crate::services::establish_connection_pg;
use crate::services::get_wishes;

//use rdiesel::{Expr};
use diesel::prelude::*;
use diesel::{RunQueryDsl, Connection};

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
        user_name: user.user_name.to_string(),
        passwd: user.passwd.to_string(),
    };

    //add to database, no rdiesel since the user's Session hasn't been created yet
    diesel::insert_into(users)
        .values(new_user)
        .execute(connection)
        .expect("Error saving new user");

    //TODO - create a (get_user) function which returns a specific user object + id based on username/passwd
    let user_obj = get_user(user.user_name.clone(), user.passwd.clone())[0].clone();
    let session_user_id = user_obj.user_id;
    jar.add(("user_id", session_user_id.to_string()));

    Template::render("wishes",  context!{})
}


//impl rdiesel::Expr<User, String> for schema::users::user_id {}

#[post("/login", data = "<user>")]
pub fn login(jar: &CookieJar<'_>, user: Form<UserDto>) -> Template {
    let connection: &mut PgConnection = &mut establish_connection_pg();

    // checks to see if user exists 

    /* NEW CODE - doesn't work?

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

    // checks to see if user exists by calling get_user
    let user_vec = get_user(user.user_name.clone(), user.passwd.clone());

    if user_vec.is_empty() {
        Template::render("users", context! {})
    } else {
        let user_obj = user_vec[0].clone();
        let session_user_id = user_obj.user_id;
        jar.add(("user_id", session_user_id.to_string()));
    
        let sess = Session {
            conn: establish_connection_pg(),
            user: user_obj.clone(),
        };

        get_wishes(sess)
    }
    
    
}

#[post("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Template {
    jar.remove("user_id");

    Template::render("users", context! {})
}

//helper function
pub fn get_user(username: String, password: String) -> Vec<User> {
    let connection: &mut PgConnection = &mut establish_connection_pg();

    let user = schema::users::dsl::users
        .filter(users::user_name.eq(username).and(users::passwd.eq(password)))
        .load::<User>(connection)
        .expect("Error finding user");

    return user
}