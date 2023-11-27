use actix_web::{delete, error, get, post, put, web, Responder};
use diesel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::crud;
use crate::crud::DbPool;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewRole {
    name: String,
    permissions: String,
}

/// Allows to create role
/// specify name and permissions
#[utoipa::path(
    request_body = NewRole,
    responses(
        (status = 200, description = "Success", body = Role),
        (status = 409, description = "Conflict"),
        (status = 505, description = "Internal server error"),
    ),
)]
#[post("/roles")]
async fn create(
    pool: web::Data<DbPool>,
    role: web::Json<NewRole>,
) -> actix_web::Result<impl Responder> {
    let NewRole { name, permissions } = role.into_inner();
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    serde_json::to_string(
        &crud::role::create(&mut conn, name, permissions)
            .await
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::CheckViolation,
                    _,
                ) => error::ErrorConflict(e),
                _ => error::ErrorInternalServerError(e),
            })?,
    )
    .map_err(error::ErrorInternalServerError)
}

/// Allows to get all roles
#[utoipa::path(
    responses(
        (status = 200, description = "Success", body = Vec<Role>),
        (status = 505, description = "Internal server error"),
    ),
)]
#[get("/roles")]
async fn reads(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    serde_json::to_string(
        &crud::role::reads(&mut conn)
            .await
            .map_err(error::ErrorInternalServerError)?,
    )
    .map_err(error::ErrorInternalServerError)
}

/// Allows to get one specific role
#[utoipa::path(
    responses(
        (status = 200, description = "Success", body=Role),
        (status = 404, description = "Role not found"),
        (status = 505, description = "Internal server error"),
    ),
)]
#[get("/roles/{role_slug}")]
async fn read(
    pool: web::Data<DbPool>,
    role_slug: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    serde_json::to_string(
        &crud::role::read(&mut conn, role_slug.into_inner())
            .await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => error::ErrorNotFound(e),
                _ => error::ErrorInternalServerError(e),
            })?,
    )
    .map_err(error::ErrorInternalServerError)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateRole {
    new_name: Option<String>,
    new_permissions: Option<String>,
}

/// Allows to update role fields
#[utoipa::path(
    request_body = UpdateRole,
    responses(
        (status = 200, description = "Success", body = Role),
        (status = 404, description = "Role not found"),
        (status = 505, description = "Internal server error"),
    ),
)]
#[put("/roles/{role_slug}")]
async fn update(
    pool: web::Data<DbPool>,
    role_slug: web::Path<String>,
    role_updates: web::Json<UpdateRole>,
) -> actix_web::Result<impl Responder> {
    let UpdateRole {
        new_name,
        new_permissions,
    } = role_updates.into_inner();
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    serde_json::to_string(
        &crud::role::update(&mut conn, role_slug.into_inner(), new_name, new_permissions)
            .await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => error::ErrorNotFound(e),
                _ => error::ErrorInternalServerError(e),
            })?,
    )
    .map_err(error::ErrorInternalServerError)
}

/// Allows to delete specific role
#[utoipa::path(
    responses(
        (status = 200, description = "Success", body = Role),
        (status = 409, description = "Conflict"),
        (status = 404, description = "Role not found"),
        (status = 505, description = "Internal server error"),
    ),
)]
#[delete("/roles/{role_slug}")]
async fn delete(
    pool: web::Data<DbPool>,
    role_slug: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    serde_json::to_string(
        &crud::role::delete(&mut conn, role_slug.into_inner())
            .await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => error::ErrorNotFound(e),
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::CheckViolation,
                    _,
                ) => error::ErrorConflict(e),
                _ => error::ErrorInternalServerError(e),
            })?,
    )
    .map_err(error::ErrorInternalServerError)
}

/// Allows to assign specific role to user
#[utoipa::path(
    responses(
        (status = 200, description = "Success"),
        (status = 409, description = "Conflict"),
        (status = 404, description = "Not found"),
        (status = 505, description = "Internal server error"),
    ),
)]
#[post("/assign/{role_slug}/{user_id}")]
async fn assign(
    pool: web::Data<DbPool>,
    params: web::Path<(String, i32)>,
) -> actix_web::Result<impl Responder> {
    let (role_slug, user_id) = params.into_inner();
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    serde_json::to_string(
        &crud::role::assign(&mut conn, role_slug, user_id)
            .await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => error::ErrorNotFound(e),
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation,
                    _,
                ) => error::ErrorNotFound(e),
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::CheckViolation,
                    _,
                ) => error::ErrorConflict(e),
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => error::ErrorConflict(e),
                _ => error::ErrorInternalServerError(e),
            })?,
    )
    .map_err(error::ErrorInternalServerError)
}

/// Allows to unassign specific role to user
#[utoipa::path(
    responses(
        (status = 200, description = "Success"),
        (status = 409, description = "Conflict"),
        (status = 404, description = "Not found"),
        (status = 505, description = "Internal server error"),
    ),
)]
#[delete("/assign/{role_slug}/{user_id}")]
async fn unassign(
    pool: web::Data<DbPool>,
    params: web::Path<(String, i32)>,
) -> actix_web::Result<impl Responder> {
    let (role_slug, user_id) = params.into_inner();
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    serde_json::to_string(
        &crud::role::unassign(&mut conn, role_slug, user_id)
            .await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => error::ErrorNotFound(e),
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation,
                    _,
                ) => error::ErrorNotFound(e),
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::CheckViolation,
                    _,
                ) => error::ErrorConflict(e),
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => error::ErrorConflict(e),
                _ => error::ErrorInternalServerError(e),
            })?,
    )
    .map_err(error::ErrorInternalServerError)
}
