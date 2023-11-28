use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};

use super::context::Context;
use super::models::User;

use crate::token_storage::TokenStorage;

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    pub async fn get_user(
        context: &Context,
        auth_token: String,
        user_name: String,
    ) -> FieldResult<User> {
        let _user = match context.auth_tokens.validate(&auth_token) {
            Some(u) => u,
            None => {
                return Err(FieldError::new(
                    "Invalid auth token",
                    graphql_value!({
                        "type": "INVALID_AUTH_TOKEN"
                    }),
                ))
            }
        };

        use crate::schema::users::dsl::*;
        let mut conn = context.pool.get().await.map_err(|e| {
            FieldError::new(
                format!("Internal database error {}", e),
                graphql_value!({
                    "type": "DATABASE_ERROR"
                }),
            )
        })?;
        users
            .filter(name.eq(user_name))
            .select(User::as_select())
            .first(&mut conn)
            .await
            .map_err(|e| {
                FieldError::new(
                    format!("Internal database error {}", e),
                    graphql_value!({
                        "type": "DATABASE_ERROR"
                    }),
                )
            })
    }
}
