#![allow(clippy::single_component_path_imports)]

use serde::Serialize;
use serde_yaml;
use std::fmt;
/// Formats Err to simple string and Ok to yaml
fn format_result<O: Serialize, E: fmt::Display>(r: Result<O, E>) -> String {
    match r {
        Ok(o) => format!("Result:\n{}", serde_yaml::to_string(&o).unwrap()),
        Err(e) => format!("Error: {}", e),
    }
}

use step_4_2::cli::{Cli, Command, RoleCommand, StructOpt, UserCommand};
use step_4_2::crud;
/// Implements CRUD logic, returns formatted result
async fn command_logic(conn: &mut AsyncPgConnection, opt: Cli) -> String {
    match opt.cmd {
        Command::User { cmd } => match cmd {
            UserCommand::List => format_result(crud::user::reads(conn).await),
            UserCommand::Create {
                name,
                bio,
                role_slug,
            } => format_result(crud::user::create(conn, name, bio, role_slug).await),
            UserCommand::Delete { user_id } => {
                format_result(crud::user::delete(conn, user_id).await)
            }
            UserCommand::Update { user_id, name, bio } => {
                format_result(crud::user::update(conn, user_id, name, bio).await)
            }
            UserCommand::Get { user_id } => format_result(crud::user::read(conn, user_id).await),
        },
        Command::Role { cmd } => match cmd {
            RoleCommand::Create { name, permissions } => {
                format_result(crud::role::create(conn, name, permissions).await)
            }
            RoleCommand::Delete { role_slug } => {
                format_result(crud::role::delete(conn, role_slug).await)
            }
            RoleCommand::Update {
                role_slug,
                name,
                permissions,
            } => format_result(crud::role::update(conn, role_slug, name, permissions).await),
            RoleCommand::Unassign { role_slug, user_id } => {
                format_result(crud::role::unassign(conn, role_slug, user_id).await)
            }
            RoleCommand::Assign { role_slug, user_id } => {
                format_result(crud::role::assign(conn, role_slug, user_id).await)
            }
            RoleCommand::List => format_result(crud::role::reads(conn).await),
            RoleCommand::Get { role_slug } => {
                format_result(crud::role::read(conn, role_slug).await)
            }
        },
    }
}

use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
type DbPool = Pool<AsyncPgConnection>;
/// Initialize pool from DATABASE_URL
async fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(conn_spec);
    Pool::builder()
        .build(config)
        .await
        .expect("failed to create pool")
}

use actix_web::{error, post, web, App, HttpServer, Responder};
/// Parse Cli commands from Vec<String>, run and return result
#[post("/command")]
async fn command(
    pool: web::Data<DbPool>,
    command: web::Json<Vec<String>>,
) -> actix_web::Result<impl Responder> {
    let opt = Cli::from_iter_safe(command.iter());
    match opt {
        Ok(o) => {
            let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
            Ok(command_logic(&mut conn, o).await)
        }
        Err(e) => Ok(e.to_string()),
    }
}

use step_4_2::set_global_subscriber;
use tracing::{info, Level};
use tracing_actix_web::TracingLogger;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_global_subscriber(Level::DEBUG).expect("failed to set up logger");
    info!("started");
    let pool = initialize_db_pool().await;
    info!("db pool initialized");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(TracingLogger::default())
            .service(command)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
