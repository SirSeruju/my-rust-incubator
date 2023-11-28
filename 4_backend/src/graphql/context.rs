use juniper::{graphql_value, FieldError, FieldResult};

use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

use crate::token_storage::Storage;

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
