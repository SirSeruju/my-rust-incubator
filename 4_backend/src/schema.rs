// @generated automatically by Diesel CLI.

diesel::table! {
    /// User table
    users (id) {
        /// User id
        id -> Int4,
        /// User name
        #[max_length = 64]
        name -> Varchar,
        /// User password hash
        password_hash -> Varchar,
    }
}

diesel::table! {
    /// User to user friendship connection
    /// user1 may have user2 in friends, but user2 may not have - could be behavior of subscribes
    users_friends (user_id, friend_id) {
        /// User id that have friend
        user_id -> Int4,
        /// Friend user id
        friend_id -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(users, users_friends,);
