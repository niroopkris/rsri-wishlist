use std::error::Error;
use crate::schema::{users, posts};

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use rocket::{get, post, FromForm};

use rocket::{Request, Response};
use rocket::request::{FromRequest, Outcome};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::http::Cookie;


//create User object
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub pass: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = users)]
pub struct UserDto {
    pub user_id: String,
    pub user_name: String,
    pub pass: String,
}


#[derive(Queryable, Insertable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = wishlists)]
pub struct Wishlist {
    pub title: String,
    pub list_desc: String,
    pub published: bool,
    pub user_id: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Associations, FromForm)]
#[diesel(belongs_to(User))]
#[diesel(table_name = wishlists)]
pub struct WishlistDto {
    pub title: String,
    pub list_desc: String,
    pub published: bool,
    pub user_id: String,
}

[derive(Queryable, Insertable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(Wishlist))]
#[diesel(table_name = wish_item)]
pub struct WishItem {
    pub id: i32,
    pub item_name: String,
    pub notes: String,
    pub wishlist: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Associations, FromForm)]
#[diesel(belongs_to(Wishlist))]
#[diesel(table_name = wish_item)]
pub struct WishItemDto {
    pub item_name: String,
    pub notes: String,
    pub wishlist: String,
}


pub struct UserSession{
    pub usr_token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserSession {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<UserSession, Self::Error> {
        let token = req.cookies().get("user_id").unwrap().value();

        let usr_token1 = token.to_string();
        println!("Your id: {}", usr_token1);

        if usr_token1.is_empty() {
            Outcome::Error((Status::Unauthorized, ()))
        } else {
            let session_user = UserSession {
                usr_token: usr_token1,
            };
            Outcome::Success(session_user)
        }
    }
}