//! Actix Web juniper example
//!
//! A simple example integrating juniper in Actix Web

use std::{io, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

mod schema;

use step_4::graphql::{create_schema, Context, Schema};

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    st: web::Data<Schema>,
    context: web::Data<Context>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    HttpResponse::Ok().json(data.execute(&st, &context).await)
}

use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;

/// Initialize pool from DATABASE_URL
pub async fn initialize_db_pool() -> Pool<AsyncPgConnection> {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(conn_spec);
    Pool::builder()
        .build(config)
        .await
        .expect("failed to create pool")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Create Juniper schema
    let schema = Arc::new(create_schema());

    let context = Arc::new(Context::new(initialize_db_pool().await));

    log::info!("starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .app_data(Data::from(context.clone()))
            .service(graphql)
            .service(graphql_playground)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
