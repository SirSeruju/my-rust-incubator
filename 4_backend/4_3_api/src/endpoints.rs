/// Roles endpoints
pub mod role;
/// Users endpoints
pub mod user;

pub use utoipa::OpenApi;

/// OpenApi schema
#[derive(OpenApi)]
#[openapi(
    paths(
        // User endpoints
        user::create,
        user::reads,
        user::read,
        user::update,
        user::delete,

        // Role endpoints
        role::create,
        role::reads,
        role::read,
        role::update,
        role::delete,
        role::assign,
        role::unassign,
    ),
    components(schemas(
        // User models
        crate::crud::user::UserWithRoles,
        crate::models::User,
        user::NewUser,
        user::UpdateUser,

        // Role models
        crate::models::Role,
        role::NewRole,
        role::UpdateRole,
    ))
)]
pub struct ApiDoc;
