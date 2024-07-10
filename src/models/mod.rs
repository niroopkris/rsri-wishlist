use std::error::Error;
use crate::schema::{users, wish, friendship};
use crate::services::establish_connection_pg;

use dotenvy::dotenv;
use std::env;

use diesel::{associations::HasTable, associations::Identifiable, Insertable, Queryable, Selectable};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use rocket::{self, http::Status, Request, routes, FromForm};
use rocket::request::{self, FromRequest, Outcome};

use crate::models;

use rdiesel::ContextImpl;

impl HasTable for User {
    type Table = crate::schema::users::table;

    fn table() -> Self::Table {
        crate::schema::users::table
    }
}

//NEED TO MODIFY USER_ID TO BE AN INTEGER, THEN MODIFY LOGIN CODE + AUTHORIZATION
//create User object
#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub passwd: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = users)]
pub struct UserDto {
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
    pub wish_owner: i32,
    pub title: String,
    pub descr: String,
    pub access_level: i32,
}

impl HasTable for WishDto {
    type Table = crate::schema::wish::table;

    fn table() -> Self::Table {
        crate::schema::wish::table
    }
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Associations, FromForm)]
#[diesel(belongs_to(User, foreign_key = wish_owner))]
#[diesel(table_name = wish)]
pub struct WishDto {
    pub wish_owner: i32,
    pub title: String,
    pub descr: String,
    pub access_level: i32,
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
    pub user1: i32,
    pub user2: i32,
    pub friend_status: i32,
}

impl HasTable for FriendshipDto {
    type Table = crate::schema::friendship::table;

    fn table() -> Self::Table {
        crate::schema::friendship::table
    }
}

#[derive(Queryable, Insertable, Serialize, Deserialize, FromForm)]
#[diesel(table_name = friendship)]
pub struct FriendshipDto {
    pub user1: i32,
    pub user2: i32,
    pub friend_status: i32,
}

pub struct Session {
    pub conn: diesel::pg::PgConnection,
    pub user: models::User
    //pub usr_token: String,
}

impl Session {
    pub fn into_context(self) -> Context {
        Context::new(self)
    }
}

type Context = rdiesel::Context<Session, models::User>;

impl ContextImpl for Session {
    type User = models::User;
    type Conn = diesel::pg::PgConnection;

    fn auth_user(&self) -> models::User {
        self.user.clone()
    }

    fn conn(&mut self) -> &mut Self::Conn {
        &mut self.conn
    }
}

const _: () = {
    #[rocket::async_trait]
    impl<'r> FromRequest<'r> for Session {
        type Error = ();

        async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
            use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
            use crate::schema::users;

            //let token: &str = req.cookies().get("user_id").unwrap().value();

            let Some(user_id) = req
                .cookies()
                .get("user_id")
                .and_then(|it| it.value().parse::<i32>().ok())
            else {
                return Outcome::Error((Status::Unauthorized, ()))
            };

            let mut conn = establish_connection_pg();
            let Some(user) = users::table
                .filter(users::user_id.eq(user_id))
                .first(&mut conn)
                .ok()
            else {
                return Outcome::Error((Status::Unauthorized, ()));
            };
            
            Outcome::Success(Session { conn, user })
            /* 
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
            */
        }
    }
};

