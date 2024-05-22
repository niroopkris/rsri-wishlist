use std::error::Error;
use crate::schema::{users, wish, friendship};

use diesel::associations::HasTable;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use rocket::{get, post, FromForm};

use rocket::{Request, Response};
use rocket::request::{FromRequest, Outcome};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::http::Cookie;


impl HasTable for User {
    type Table = crate::schema::users::table;

    fn table() -> Self::Table {
        crate::schema::users::table
    }
}
//create User object
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub passwd: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = users)]
pub struct UserDto {
    pub user_id: String,
    pub user_name: String,
    pub passwd: String,
}

impl HasTable for Wish {
    type Table = crate::schema::wish::table;

    fn table() -> Self::Table {
        crate::schema::wish::table
    }
}
#[derive(Queryable, Insertable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(User, foreign_key = wish_owner))]
#[diesel(table_name = wish)]
pub struct Wish {
    pub id: i32,
    pub wish_owner: String,
    pub title: String,
    pub descr: String,
    pub access_level: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Associations, FromForm)]
#[diesel(belongs_to(User, foreign_key = wish_owner))]
#[diesel(table_name = wish)]
pub struct WishDto {
    pub wish_owner: String,
    pub title: String,
    pub descr: String,
    pub access_level: String,
}


impl HasTable for Friendship {
    type Table = crate::schema::friendship::table;

    fn table() -> Self::Table {
        crate::schema::friendship::table
    }
}
#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = friendship)]
pub struct Friendship {
    pub id: i32,
    pub user1: String,
    pub user2: String,
    pub friend_status: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = friendship)]
pub struct FriendshipDto {
    pub user1: String,
    pub user2: String,
    pub friend_status: String,
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

