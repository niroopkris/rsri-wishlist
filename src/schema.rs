// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Varchar,
        user_name -> Varchar,
        pass -> Varchar,
    }
}

diesel::table! {
    wish_item (id) {
        id -> Int4,
        item_name -> Varchar,
        notes -> Text,
        wishlist -> Varchar,
    }
}

diesel::table! {
    wishlists (title) {
        title -> Varchar,
        list_desc -> Text,
        published -> Bool,
        user_id -> Varchar,
    }
}

diesel::joinable!(wish_item -> wishlists (wishlist));
diesel::joinable!(wishlists -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    wish_item,
    wishlists,
);
