use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};

use super::context::Context;

/// User from database for public view
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    /// User id from [`users::columns`][`crate::schema::users::columns`]
    pub id: i32,
    /// User name from [`users::columns`][`crate::schema::users::columns`]
    pub name: String,
}

#[graphql_object(context = Context)]
impl User {
    /// Allows to collect user friends from database
    async fn friends(&self, context: &Context) -> FieldResult<Vec<User>> {
        use crate::schema::users::dsl::*;
        use crate::schema::users_friends::dsl::*;

        let mut conn = context.get_db_connection().await?;
        users_friends
            .inner_join(users.on(id.eq(friend_id)))
            .filter(user_id.eq(self.id))
            .select(User::as_select())
            .load(&mut conn)
            .await
            .map_err(|e| {
                FieldError::new(
                    format!("Database error {}", e),
                    graphql_value!({"code": "DATABASE_ERROR"}),
                )
            })
    }
    /// User name for GraphQL query
    fn name(&self) -> &str {
        self.name.as_str()
    }
    /// User id for GraphQL query
    fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Debug, Insertable, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// User type for authenticate  process
pub struct AuthUser {
    /// User name from [`users::columns`][`crate::schema::users::columns`]
    pub name: String,
    /// User password hash from [`users::columns`][`crate::schema::users::columns`]
    pub password_hash: String,
}

#[derive(Debug, Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users_friends)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// User to user [`friendship link`][`crate::schema::users_friends`]
pub struct Friendship {
    /// User that have friend from [`users_friends::columns`][`crate::schema::users_friends::columns`]
    pub user_id: i32,
    /// Friend user id from [`users_friends::columns`][`crate::schema::users_friends::columns`]
    pub friend_id: i32,
}
