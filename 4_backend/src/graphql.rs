use juniper::{
    graphql_object, graphql_value, EmptySubscription, FieldError, FieldResult, RootNode,
};

use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::token_storage::{Storage, TokenStorage};

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

pub struct Context {
    pub pool: Pool<AsyncPgConnection>,
    pub auth_tokens: Storage,
}
impl juniper::Context for Context {}
impl Context {
    pub fn new(pool: Pool<AsyncPgConnection>) -> Self {
        Context {
            pool,
            auth_tokens: Storage::new(),
        }
    }
    pub async fn get_db_connection(
        &self,
    ) -> FieldResult<bb8::PooledConnection<AsyncDieselConnectionManager<AsyncPgConnection>>> {
        self.pool.get().await.map_err(|e| {
            FieldError::new(
                format!("Database connection error {}", e),
                graphql_value!({"code": "DATABASE_CONNECTION_ERROR"}),
            )
        })
    }
}

// AsyncDieselConnectionManager
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
                    graphql_value!({"code": "INVALID_AUTH_TOKEN"}),
                ))
            }
        };

        use crate::schema::users::dsl::*;
        let mut conn = context.get_db_connection().await?;
        users
            .filter(name.eq(user_name))
            .select(User::as_select())
            .first(&mut conn)
            .await
            .map_err(|e| {
                FieldError::new(
                    format!("Database error {}", e),
                    graphql_value!({"code": "DATABASE_ERROR"}),
                )
            })
    }
}

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

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

pub struct MutationRoot;
#[graphql_object(context = Context)]
impl MutationRoot {
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
                    graphql_value!({"code": "PASSWORD_HASH_ERROR"}),
                )
            })?
            .to_string();

        use crate::schema::users::dsl::*;
        let mut conn = context.get_db_connection().await?;
        diesel::insert_into(users)
            .values(&AuthUser {
                name: user_name,
                password_hash: pass_hash,
            })
            .returning(User::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(|e| match e {
                Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => FieldError::new(
                    format!("User already exist {}", e),
                    graphql_value!({"code": "USER_ALREADY_EXIST"}),
                ),
                _ => FieldError::new(
                    format!("Database error {}", e),
                    graphql_value!({"code": "DATABASE_ERROR"}),
                ),
            })
    }
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
                graphql_value!({"code": "PASSWORD_HASH_ERROR"}),
            )
        })?;

        argon2.verify_password(&password.into_bytes(), &db_pass_hash)?;
        Ok(context.auth_tokens.new_token(user_name))
    }
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

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
