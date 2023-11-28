use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};

use super::context::Context;
use super::models::{AuthUser, Friendship, User};

use crate::token_storage::TokenStorage;

/// /// Implements GraphQL mutations
pub struct MutationRoot;

#[graphql_object(context = Context)]
impl MutationRoot {
    /// Allows to register user
    pub async fn register(
        context: &Context,
        user_name: String,
        password: String,
    ) -> FieldResult<User> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let pass_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                FieldError::new(
                    format!("Password hash error {}", e),
                    graphql_value!({"code": "PASSWORD_HASH"}),
                )
            })?
            .to_string();

        use crate::schema::users::dsl::*;
        let mut conn = context.pool.get().await.map_err(|e| {
            FieldError::new(
                format!("Database error {}", e),
                graphql_value!({"code": "DATABASE_ERROR"}),
            )
        })?;
        diesel::insert_into(users)
            .values(&AuthUser {
                name: user_name,
                password_hash: pass_hash,
            })
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| {
                FieldError::new(
                    format!("Database error {}", e),
                    graphql_value!({"code": "DATABASE_ERROR"}),
                )
            })
    }
    /// Allows to authenticate user
    pub async fn authenticate(
        context: &Context,
        user_name: String,
        password: String,
    ) -> FieldResult<String> {
        let argon2 = Argon2::default();

        use crate::schema::users::dsl::*;
        let mut conn = context.get_db_connection().await?;
        let auth_user = users
            .filter(name.eq(user_name.clone()))
            .select(AuthUser::as_select())
            .first(&mut conn)
            .await
            .map_err(|e| match e {
                Error::NotFound => FieldError::new(
                    format!("Wrong username or password {}", e),
                    graphql_value!({"code": "LOGIN_ERROR"}),
                ),
                _ => FieldError::new(
                    format!("Database error {}", e),
                    graphql_value!({"code": "DATABASE_ERROR"}),
                ),
            })?;
        let db_pass_hash = PasswordHash::new(&auth_user.password_hash).map_err(|e| {
            FieldError::new(
                format!("Password hash error {}", e),
                graphql_value!({"code": "PASSWORD_HASH"}),
            )
        })?;

        argon2.verify_password(&password.into_bytes(), &db_pass_hash)?;
        Ok(context.auth_tokens.new_token(user_name))
    }
    /// Allows to add friend to authenticated user by user name
    pub async fn add_friend(
        context: &Context,
        auth_token: String,
        user_name: String,
    ) -> FieldResult<User> {
        let current_user_name = match context.auth_tokens.validate(&auth_token) {
            Some(u) => u,
            None => {
                return Err(FieldError::new(
                    "Invalid auth token",
                    graphql_value!({"code": "INVALID_AUTH_TOKEN"}),
                ))
            }
        };

        use crate::schema::users::dsl::*;
        use crate::schema::users_friends::dsl::*;
        let mut conn = context.get_db_connection().await?;
        let current_user = users
            .filter(name.eq(current_user_name))
            .select(User::as_select())
            .first(&mut conn)
            .await
            .map_err(|e| match e {
                Error::NotFound => FieldError::new(
                    format!("Current user not found {}", e),
                    graphql_value!({"code": "CURRENT_USER_NOT_FOUND"}),
                ),
                _ => FieldError::new(
                    format!("Database error {}", e),
                    graphql_value!({"code": "DATABASE_ERROR"}),
                ),
            })?;
        let friend_user_id: i32 = users
            .filter(name.eq(user_name))
            .select(id)
            .first(&mut conn)
            .await
            .map_err(|e| match e {
                Error::NotFound => FieldError::new(
                    format!("Friend user not found {}", e),
                    graphql_value!({"code": "FRIEND_USER_NOT_FOUND"}),
                ),
                _ => FieldError::new(
                    format!("Database error {}", e),
                    graphql_value!({"code": "DATABASE_ERROR"}),
                ),
            })?;
        diesel::insert_into(users_friends)
            .values(&Friendship {
                user_id: current_user.id,
                friend_id: friend_user_id,
            })
            .execute(&mut conn)
            .await
            .map_err(|e| {
                FieldError::new(
                    format!("Database error {}", e),
                    graphql_value!({"code": "DATABASE_ERROR"}),
                )
            })?;
        Ok(current_user)
    }
    /// Allows to remove friend from authenticated user by user name
    pub async fn remove_friend(
        context: &Context,
        auth_token: String,
        user_name: String,
    ) -> FieldResult<User> {
        let current_user_name = match context.auth_tokens.validate(&auth_token) {
            Some(u) => u,
            None => {
                return Err(FieldError::new(
                    "Invalid auth token",
                    graphql_value!({"code": "INVALID_AUTH_TOKEN"}),
                ))
            }
        };

        use crate::schema::users::dsl::*;
        use crate::schema::users_friends::dsl::*;
        let mut conn = context.get_db_connection().await?;
        let current_user = users
            .filter(name.eq(current_user_name))
            .select(User::as_select())
            .first(&mut conn)
            .await
            .map_err(|e| match e {
                Error::NotFound => FieldError::new(
                    format!("Current user not found {}", e),
                    graphql_value!({"code": "CURRENT_USER_NOT_FOUND"}),
                ),
                _ => FieldError::new(
                    format!("Database error {}", e),
                    graphql_value!({"code": "DATABASE_ERROR"}),
                ),
            })?;
        let friend_user_id: i32 = users
            .filter(name.eq(user_name))
            .select(id)
            .first(&mut conn)
            .await
            .map_err(|e| match e {
                Error::NotFound => FieldError::new(
                    format!("Friend user not found {}", e),
                    graphql_value!({"code": "FRIEND_USER_NOT_FOUND"}),
                ),
                _ => FieldError::new(
                    format!("Database error {}", e),
                    graphql_value!({"code": "DATABASE_ERROR"}),
                ),
            })?;
        diesel::delete(users_friends)
            .filter(user_id.eq(current_user.id))
            .filter(friend_id.eq(friend_user_id))
            .execute(&mut conn)
            .await
            .map_err(|e| match e {
                Error::NotFound => FieldError::new(
                    format!("Friendship not found {}", e),
                    graphql_value!({"code": "FRIENDSHIP_NOT_FOUND"}),
                ),
                _ => FieldError::new(
                    format!("Database error {}", e),
                    graphql_value!({"code": "DATABASE_ERROR"}),
                ),
            })?;
        Ok(current_user)
    }
}
