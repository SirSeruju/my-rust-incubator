use utoipa_swagger_ui::SwaggerUi;

use actix_web::{web, App, HttpServer};
use tracing::info;
use tracing_actix_web::TracingLogger;

use step_4_3::crud::initialize_db_pool;
use step_4_3::endpoints;
use step_4_3::endpoints::{ApiDoc, OpenApi};
use step_4_3::init_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger().expect("failed to set up logger");

    info!("started");
    let pool = initialize_db_pool().await;
    info!("db pool initialized");

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(TracingLogger::default())
            // User endpoints
            .service(endpoints::user::create)
            .service(endpoints::user::reads)
            .service(endpoints::user::read)
            .service(endpoints::user::update)
            .service(endpoints::user::delete)
            // Role endpoints
            .service(endpoints::role::create)
            .service(endpoints::role::reads)
            .service(endpoints::role::read)
            .service(endpoints::role::update)
            .service(endpoints::role::delete)
            .service(endpoints::role::assign)
            .service(endpoints::role::unassign)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
