use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};

use super::context::Context;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[graphql_object(context = Context)]
impl User {
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
    async fn name(&self) -> &str {
        self.name.as_str()
    }
    async fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Debug, Insertable, Selectable, Queryable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthUser {
    pub name: String,
    pub password_hash: String,
}

#[derive(Debug, Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users_friends)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Friendship {
    pub user_id: i32,
    pub friend_id: i32,
}
