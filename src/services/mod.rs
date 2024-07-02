pub mod auth;

extern crate diesel;
extern crate rocket;

use crate::services::diesel::Connection;
use diesel::pg::PgConnection;
use diesel::{connection, RunQueryDsl};
//use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post, FromForm};
use crate::models::{User, UserDto, Wish, WishDto, Friendship, FriendshipDto, Session};
use crate::schema::{users, wish, friendship};

use crate::schema;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};
use std::env;

use rocket::{Request, Response};
use rocket::request::{FromRequest, Outcome};
use rocket::http::{CookieJar, Cookie};

use flux_rs::*;
use rdiesel::Expr;
//use rdiesel::{select_list, update_where, Expr, Field};

//TEMPORARY PLACE: needs to be moved somewhere else and used like: use crate::PUBLIC
#[constant]
pub const PUBLIC: i32 = 0;
#[constant]
pub const FRIENDS: i32 = 1;

//move this from here to auth.rs???
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

impl rdiesel::Expr<Wish, String> for schema::wish::wish_owner {}

//CRUD functions for a User's posts
#[post("/create_wish", format = "form", data = "<a_wish>")]
pub fn create_wish(a_wish: Form<WishDto>, sess: Session) -> Template {
    //use schema::wish::dsl::*;
    
    let mut cx = sess.into_context();

    let auth_user = cx.auth_user();

    let new_wish = WishDto {
        wish_owner: auth_user.user_id.to_string(),
        title: a_wish.title.to_string(),
        descr: a_wish.descr.to_string(),
        access_level: a_wish.access_level.to_string(),
    };

    let _ = cx.insert(new_wish);

    let results_q1 = wish::wish_owner.eq(auth_user.user_id.to_string());
    //let results = rdiesel::select_list(connection, results_q1);
    let results = cx.select_list(results_q1);

    Template::render("wishes", context! {wishes: &results.expect("ERROR LOADING WISHES")})

    /* 
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

    let results_q1 = wish_owner.eq(usr_token.to_string());
    let results = rdiesel::select_list(connection, results_q1);

    /* OLD METHOD OF ACCESSING DATABASE    
    let results = self::schema::wish::dsl::wish
        .filter(wish_owner.eq(usr_token))
        .load::<Wish>(connection)
        .expect("Error loading posts");
    */

    Template::render("wishes", context! {wishes: &results.expect("ERROR LOADING WISHES")})
   */
}


impl rdiesel::Expr<Friendship, String> for schema::friendship::friend_status {}
impl rdiesel::Expr<Friendship, String> for schema::friendship::user1 {}
impl rdiesel::Expr<Friendship, String> for schema::friendship::user2 {}

impl rdiesel::Expr<Wish, String> for schema::wish::access_level {}

#[get("/")]
pub fn get_wishes(sess: Session) -> Template {
    //use diesel::dsl::not;

    //all database queries should go through CX context now
    let mut cx = sess.into_context();

    let auth_user = cx.auth_user();

    let id = auth_user.user_id;

    //retrive your personal wishes and friend/public wishes
    //to allow default .neq() to work, can move this func before create_wish and just implement Expr for wish_owner after this func 
    let wishes = cx.select_list(wish::wish_owner.eq(id));
    let other_wishes = cx.select_list(
        //wish::wish_owner.neq(id).and(
            (friendship::user1.eq(id).or(friendship::user2.eq(id))).and(
                wish::access_level
                    .eq(PUBLIC)
                    .or(wish::access_level.eq(FRIENDS)),
            ),
        //),
    );

    

    // unwrap verification is slower
    // let wishes = wishes.unwrap();
    let Ok(wishes) = wishes else {
        panic!("Error retrieving wishes");
    };

    let Ok(other_wishes) = other_wishes else {
        panic!("Error retrieving friend wishes");
    };

    /* 
    for w in wishes {
        assert(w.wish_owner == id);
    }
    */

    Template::render("wishes", context! {wishes: &wishes, other_wishes: &other_wishes})

    /* OLD RDIESEL CODE 
    let connection = &mut establish_connection_pg();
    let usr_token = &usr_session.usr_token;

    let acceptedStr = "Accepted";

    // retrieves vector of user's friends
    let friend_q1 = friend_status.eq(acceptedStr.to_string());
    let friend_q2 = user1.eq(usr_token.to_string());
    let friend_q3 = user2.eq(usr_token.to_string());
    let friend_q4 = friend_q2.or(friend_q3);
    let friend_q5 = friend_q1.and(friend_q4);
    let friendships = rdiesel::select_list(connection, friend_q5);

    //create vector of user's friends' ids 
    let mut friend_ids:Vec<String> = Vec::new();

    for i in friendships.expect("ERROR RETRIEVING FRIENDSHIPS") {
        friend_ids.push(i.user1.to_string());
        friend_ids.push(i.user2.to_string());
    }

    //retrive your own wish results
    let results_q1 = wish_owner.eq(usr_token.to_string());
    let your_results = rdiesel::select_list(connection, results_q1);

    //separate friend + public results and print them separately 
    let other_q1 = access_level.eq("public".to_string());
    //let other_q2 = rdiesel::Expr::not(wish_owner, usr_token);         //need not function in Expr
    //let other_q3 = rdiesel::Expr::and(other_q1, other_q2);
    let other_q4 = wish_owner.eq_any(friend_ids);
    let other_q5 = access_level.eq("friends".to_string());
    let other_q6 = other_q4.and(other_q5);
    //let other_q7 = rdiesel::Expr::or(other_q3, other_q6);
    let other_q7 = other_q1.or(other_q6);
    let other_results = rdiesel::select_list(connection, other_q7);

    Template::render("wishes", context! {wishes: &your_results.expect("ERROR LOADING WISHES"), 
        friend_wishes: &other_results.expect("ERROR LOADING WISHES")})
    */


    /* EVEN OLDER CODE
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
        .filter(access_level.eq("public").and(not(wish_owner.eq(usr_token)))) 
        .or_filter((wish_owner.eq_any(friend_ids)).and(access_level.eq("friends")))
        .load::<Wish>(connection)
        .expect("Error loading wishes");
    

    Template::render("wishes", context! {wishes: &your_results, friend_wishes: &other_results})
    */
}


#[post("/delete/<my_id>")]
pub fn delete_wish(sess: Session, my_id: i32) -> Template{
    //use self::schema::wish::wish_owner; 
    let sess_backup = Session {
        conn: establish_connection_pg(),
        user: sess.user.clone(),
    };

    let mut cx = sess.into_context();

    let auth_user = cx.auth_user();

    //update query to rdiesel

    /* 
    let deleted = diesel::delete(wish.filter(id.eq(my_id)))
        .execute(connection)
        .expect("Error deleting posts");
    */
    get_wishes(sess_backup)
}

impl rdiesel::Expr<Wish, i32> for schema::wish::id {}
impl rdiesel::Expr<Wish, String> for schema::wish::title {}
impl rdiesel::Expr<Wish, String> for schema::wish::descr {}

impl rdiesel::Field<Wish, i32, User> for schema::wish::id {}
impl rdiesel::Field<Wish, String, User> for schema::wish::wish_owner {}
impl rdiesel::Field<Wish, String, User> for schema::wish::title {}
impl rdiesel::Field<Wish, String, User> for schema::wish::descr {}
impl rdiesel::Field<Wish, String, User> for schema::wish::access_level {}

#[get("/edit/redirect/<wish_id>")]
pub fn edit_wish_redirect(sess: Session, wish_id: i32) -> Template {
    let mut cx = sess.into_context();

    let auth_user = cx.auth_user();

    let wish = cx.select_first(wish::id.eq(wish_id));
    
    /* 
    let wish_q1 = wish::id.eq(wish_id);
    let find_wish = rdiesel::select_list(connection, wish_q1);
    */

    Template::render("wish_edit", context! {wishes: &wish.expect("ERROR EDITING WISH")})
}

#[post("/edit/<wish_id>", format = "form", data = "<a_wish>")]
pub fn edit_wish(a_wish: Form<WishDto>, sess: Session, wish_id: i32) -> Template {
    //use crate::models::WishDto;
    //use crate::models::Wish;

    let mut cx = sess.into_context();

    let auth_user = cx.auth_user();

    let _ = cx.update_where(wish::id.eq(wish_id), wish::wish_owner.assign(auth_user.user_id.to_string()));
    let _ = cx.update_where(wish::id.eq(wish_id), wish::title.assign(a_wish.title.to_string()));
    let _ = cx.update_where(wish::id.eq(wish_id), wish::descr.assign(a_wish.descr.to_string()));
    let _ = cx.update_where(wish::id.eq(wish_id), wish::access_level.assign(a_wish.access_level.to_string()));


    get_wishes(sess)

    /* OLD RDIESEL CODE
    let wish_q1 = id.eq(wish_id);
    let wish_q2 = id.eq(wish_id);
    let wish_q3 = id.eq(wish_id);
    let wish_q4 = id.eq(wish_id);

    let _ = rdiesel::update_where(connection, wish_q1, wish_owner.assign(usr_token.to_string()));
    let _ = rdiesel::update_where(connection, wish_q2, title.assign(a_wish.title.to_string()));
    let _ = rdiesel::update_where(connection, wish_q3, descr.assign(a_wish.descr.to_string()));
    let _ = rdiesel::update_where(connection, wish_q4, access_level.assign(a_wish.access_level.to_string()));
    */


    /* OLDEST CODE (no rdiesel)
    let new_wish = WishDto {
        wish_owner: usr_token.to_string(),
        title: a_wish.title.to_string(),
        descr: a_wish.descr.to_string(),
        access_level: a_wish.access_level.to_string()
    };
    
    diesel::update(wish)
        .filter(id.eq(wish_id))
        .set(new_wish)
        .execute(connection)
        .expect("Error updating posts");
    */

}
 


#[get("/friendships")]
pub fn get_friendships(sess: Session) -> Template {
    //use self::schema::friendship::user1;

    let mut cx = sess.into_context();

    let auth_user = cx.auth_user();

    let friendships = cx.select_list(
        friendship::user1.eq(auth_user.user_id.to_string()).or(
            friendship::user2.eq(auth_user.user_id.to_string())
        )
    );

    let requests = cx.select_list(
        friendship::user2.eq(auth_user.user_id.to_string()).and(
            friendship::friend_status.eq("pending".to_string())
        )
    );

    Template::render("friendships", context! {friendships: &friendships.expect("ERROR LOADING FRIENDSHIPS"),
        requests: &requests.expect("ERROR LOADING REQUESTS")})
    /* 
    let results_q1 = user1.eq(usr_token.to_string());
    let results_q2 = user2.eq(usr_token.to_string());
    let results_q3 = results_q1.or(results_q2);
    let results = rdiesel::select_list(connection, results_q3);

    let requests_q1 = user2.eq(usr_token.to_string());
    let requests_q2 = friend_status.eq("pending".to_string());
    let requests_q3 = requests_q1.and(requests_q2);
    let requests = rdiesel::select_list(connection, requests_q3);
    
    Template::render("friendships", context! {friendships: &results.expect("ERROR LOADING FRIENDSHIPS"),
        requests: &requests.expect("ERROR LOADING REQUESTS")})
*/

    /* 
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
    */
}

impl rdiesel::Expr<User, String> for schema::users::user_id {}

#[post("/post_friendship", format="form", data="<a_friendship>")]
pub fn create_friendship_request(a_friendship: Form<FriendshipDto>, sess: Session) -> Template {
    //use crate::schema::users::user_id;
    //use self::schema::friendship::dsl::friendship;

    let mut cx = sess.into_context();

    let auth_user = cx.auth_user();

    //check if requested user exists
    let requested_user = cx.select_first(auth_user.user_id.eq(a_friendship.user2.to_string()));

    if requested_user.expect("USER DOESN'T EXIST").is_none() {
        Template::render("friendships", context! {})
    } else {
        let new_friendship = FriendshipDto {
            user1: auth_user.user_id.to_string(),
            user2: a_friendship.user2.to_string(),
            friend_status: a_friendship.friend_status.to_string()
        };

        let _ = cx.insert(new_friendship);

        let results = cx.select_list(friendship::user1.eq(auth_user.user_id.to_string()));

        Template::render("friendships", context! {friendships: &results.expect("ERROR")})       
    }

    /* 
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
    */
}


impl rdiesel::Field<Friendship, String, User> for schema::friendship::user1 {}
impl rdiesel::Field<Friendship, String, User> for schema::friendship::user2 {}
impl rdiesel::Field<Friendship, String, User> for schema::friendship::friend_status {}

#[post("/change_friendship", format="form", data="<a_friendship>")]
pub fn change_friendship_status(a_friendship: Form<FriendshipDto>, sess: Session) -> Template {
    //use self::schema::friendship::dsl::*;

    let mut cx = sess.into_context();

    let auth_user = cx.auth_user();

    let q1 = friendship::user1.eq(a_friendship.user1.to_string()).and(
        friendship::user2.eq(a_friendship.user2.to_string())
    );

    let _ = cx.update_where(q1, friendship::friend_status.assign(a_friendship.friend_status.to_string()));

    /* 
    let q1 = user1.eq(a_friendship.user1.to_string());
    let q2 = user2.eq(a_friendship.user2.to_string());
    let q3 = q1.and(q2);
    
    diesel::update(friendship)
        .filter((user1.eq(a_friendship.user1.to_string())).and(user2.eq(a_friendship.user2.to_string()))) //matches to friendship in table
        .set(friend_status.eq(&a_friendship.friend_status))
        .execute(connection)
        .expect("Error updating status");
    */

    get_friendships(sess)
}