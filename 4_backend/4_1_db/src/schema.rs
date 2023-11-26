// @generated automatically by Diesel CLI.

diesel::table! {
    roles (slug) {
        slug -> Varchar,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 64]
        permissions -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 256]
        bio -> Varchar,
    }
}

diesel::table! {
    users_roles (role_slug, user_id) {
        role_slug -> Varchar,
        user_id -> Int4,
    }
}

diesel::joinable!(users_roles -> roles (role_slug));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(roles, users, users_roles,);
