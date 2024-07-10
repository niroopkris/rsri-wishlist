// @generated automatically by Diesel CLI.

diesel::table! {
    friendship (id) {
        id -> Int4,
        user1 -> Int4,
        user2 -> Int4,
        friend_status -> Int4,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        passwd -> Varchar,
    }
}

diesel::table! {
    wish (id) {
        id -> Int4,
        wish_owner -> Int4,
        title -> Varchar,
        descr -> Text,
        access_level -> Int4,
    }
}

diesel::joinable!(wish -> users (wish_owner));

diesel::allow_tables_to_appear_in_same_query!(
    friendship,
    users,
    wish,
);
