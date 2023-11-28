// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        password_hash -> Varchar,
    }
}

diesel::table! {
    users_friends (user_id, friend_id) {
        user_id -> Int4,
        friend_id -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(users, users_friends,);
