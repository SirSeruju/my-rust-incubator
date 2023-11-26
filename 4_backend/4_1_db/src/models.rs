use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub bio: String,
}

#[derive(Debug, Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Role {
    pub slug: String,
    pub name: String,
    pub permissions: String,
}

#[derive(Debug, Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users_roles)]
#[diesel(primary_key(role_slug, user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRole {
    pub role_slug: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub name: String,
    pub bio: String,
}
