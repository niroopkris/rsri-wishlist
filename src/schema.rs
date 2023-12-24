// @generated automatically by Diesel CLI.

diesel::table! {
    friendship (id) {
        id -> Int4,
        user1 -> Varchar,
        user2 -> Varchar,
        friend_status -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Varchar,
        user_name -> Varchar,
        passwd -> Varchar,
    }
}

diesel::table! {
    wish (id) {
        id -> Int4,
        wish_owner -> Varchar,
        title -> Varchar,
        descr -> Text,
        access_level -> Varchar,
    }
}

diesel::joinable!(wish -> users (wish_owner));

diesel::allow_tables_to_appear_in_same_query!(
    friendship,
    users,
    wish,
);
