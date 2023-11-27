/// CRUD for roles
pub mod role;
/// CRUD for users
pub mod user;

use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

pub type DbPool = Pool<AsyncPgConnection>;

/// Initialize pool from DATABASE_URL
pub async fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(conn_spec);
    Pool::builder()
        .build(config)
        .await
        .expect("failed to create pool")
}
