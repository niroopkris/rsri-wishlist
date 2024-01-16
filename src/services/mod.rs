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
use crate::models::FriendshipDto; 
use crate::models::UserSession; 
use crate::schema::friendship::{user1, user2, friend_status};
use crate::schema::wish::access_level;

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
#[get("/home")]
pub fn home() -> Template {
    Template::render("users",  context!{})
}

//create User 
#[post("/create_user", format = "form", data = "<user>")]
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

    Template::render("wishes",  context!{})
}

#[post("/login", data = "<user>")]
pub fn login(jar: &CookieJar<'_>, user: Form<UserDto>) -> Template {
    use self::models::Wish;
    use self::models::User;
    use self::schema::wish::wish_owner;
    use self::schema::users::user_id;
    use self::schema::users::passwd;

    let connection = &mut establish_connection_pg();

    // checks to see if user exists 
    let is_user = self::schema::users::dsl::users
        .filter(user_id.eq(user.user_id.to_string()))
        .load::<User>(connection)
        .expect("Error loading users");

    if is_user.is_empty() {
        Template::render("users", context! {})
    } else {
        let session_user_id = user.user_id.to_string();
        jar.add(("user_id", session_user_id.clone()));

        println!("{}", session_user_id);
    
        let results = self::schema::wish::dsl::wish
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

#[get("/")]
pub fn get_wishes(usr_session: UserSession) -> Template {
    use self::models::Wish;
    use self::schema::wish::wish_owner; 

    use self::models::Friendship;


    let connection = &mut establish_connection_pg();

    let usr_token = &usr_session.usr_token;

    // retrieves vector of user's friends
    let friendships = self::schema::friendship::dsl::friendship
        .filter(friend_status.eq("Accepted"))
        .filter((user1.eq(usr_token)).or(user2.eq(usr_token)))
        .load::<Friendship>(connection)
        .expect("Error loading friendships");

    //creates vector of user's friends' ids
    let mut friend_ids:Vec<String> = Vec::new();

    for i in &friendships {
        friend_ids.push(i.user1.to_string());
        friend_ids.push(i.user2.to_string());
    }

    println!("{:?}", friend_ids);

    let your_results = self::schema::wish::dsl::wish
        .filter(wish_owner.eq(usr_token)) 
        .load::<Wish>(connection)
        .expect("Error loading posts");

    //separate friend results and print them separately 
    let other_results = self::schema::wish::dsl::wish
        .filter(access_level.eq("public")) 
        .or_filter((wish_owner.eq_any(friend_ids)).and(access_level.eq("friends")))
        .load::<Wish>(connection)
        .expect("Error loading wishes");
    

    Template::render("wishes", context! {wishes: &your_results, friend_wishes: &other_results})
}


#[post("/delete/<my_id>")]
pub fn delete_wish(usr_session: UserSession, my_id: i32) -> Template{
    use self::models::Wish;
    //use self::schema::wish::wish_owner; 
    use self::schema::wish::dsl::*;

    let connection = &mut establish_connection_pg();

    let usr_token = &usr_session.usr_token;

    let deleted = diesel::delete(wish.filter(id.eq(my_id)))
        .execute(connection)
        .expect("Error deleting posts");
    
    get_wishes(usr_session)
}

/* 
#[post("/edit/<my_id>", format = "form", data = "<a_wish>")]
pub fn edit_wish(a_wish: Form<WishDto>, usr_session: UserSession, my_id: i32) -> Template {
    use self::schema::wish::dsl::*;
    use crate::models::WishDto;
    use self::models::Wish;

    let connection = &mut establish_connection_pg();

    let usr_token = &usr_session.usr_token;

    let new_wish = WishDto {
        wish_owner: usr_token.to_string(),
        title: a_wish.title.to_string(),
        descr: a_wish.descr.to_string(),
        access_level:a_wish.to_string()
    };

    diesel::update(wish)
        .filter(id.eq(my_id))
        .set(new_wish)
        .execute(connection)
        .expect("Error updating posts");

    Template::render("wish_edit", context! {})
}
 */

 




#[get("/friendships")]
pub fn get_friendships(usr_session: UserSession) -> Template {
    use self::models::Friendship;
    use self::schema::friendship::user1;

    let connection = &mut establish_connection_pg();

    let usr_token = &usr_session.usr_token;

    let results = self::schema::friendship::dsl::friendship
        .filter(user1.eq(usr_token))
        .or_filter(user2.eq(usr_token))
        .load::<Friendship>(connection)
        .expect("Error loading friendships");

    let requests = self::schema::friendship::dsl::friendship
        .filter((user2.eq(usr_token)).and(friend_status.eq("pending")))
        .load::<Friendship>(connection)
        .expect("Error loading friendships");

    Template::render("friendships", context! {friendships: &results, requests: &requests})
}

#[post("/post_friendship", format="form", data="<a_friendship>")]
pub fn create_friendship_request(a_friendship: Form<FriendshipDto>, usr_session: UserSession) -> Template {
    use self::models::User;
    use crate::schema::users::user_id;
    use self::models::Friendship;
    use self::schema::friendship::dsl::friendship;

    let connection = &mut establish_connection_pg();

    let usr_token = usr_session.usr_token;

    // checks to see if requested user exists
    let requested_user = self::schema::users::dsl::users
        .filter(user_id.eq(a_friendship.user2.to_string()))
        .load::<User>(connection)
        .expect("Error retrieving user");

    if requested_user.is_empty() {
        Template::render("friendships", context! {})
    } else  {
        let new_friendship = FriendshipDto {
            user1: usr_token.to_string(),
            user2: a_friendship.user2.to_string(),
            friend_status: a_friendship.friend_status.to_string()
        };

        diesel::insert_into(friendship)
            .values(new_friendship)
            .execute(connection)
            .expect("Friendship failed");

        let results = self::schema::friendship::dsl::friendship
            .filter(user1.eq(usr_token))
            .load::<Friendship>(connection)
            .expect("Error loading friendships");
    
        Template::render("friendships", context! {friendships: &results})
    }
}

#[post("/change_friendship", format="form", data="<a_friendship>")]
pub fn change_friendship_status(a_friendship: Form<FriendshipDto>, usr_session: UserSession) -> Template {
    use self::schema::friendship::dsl::*;

    let connection = &mut establish_connection_pg();

    diesel::update(friendship)
        .filter((user1.eq(a_friendship.user1.to_string())).and(user2.eq(a_friendship.user2.to_string()))) //matches to friendship in table
        .set(friend_status.eq(&a_friendship.friend_status))
        .execute(connection)
        .expect("Error updating status");

    get_friendships(usr_session)
}