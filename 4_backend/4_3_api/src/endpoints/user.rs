use actix_web::{delete, error, get, post, put, web, Responder};
use diesel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::crud;
use crate::crud::DbPool;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct NewUser {
    pub name: String,
    pub bio: String,
    pub role_slug: String,
}

/// Allows to create user
/// specify name, bio and role_slug needed
#[utoipa::path(
    request_body = NewUser,
    responses(
        (status = 200, description = "Success", body = User),
        (status = 409, description = "Conflict"),
        (status = 505, description = "Internal server error"),
    ),
)]
#[post("/users")]
async fn create(
    pool: web::Data<DbPool>,
    user: web::Json<NewUser>,
) -> actix_web::Result<impl Responder> {
    let NewUser {
        name,
        bio,
        role_slug,
    } = user.into_inner();
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    serde_json::to_string(
        &crud::user::create(&mut conn, name, bio, role_slug)
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

/// Allows to get all users without roles
#[utoipa::path(
    responses(
        (status = 200, description = "Success", body = Vec<User>),
        (status = 505, description = "Internal server error"),
    ),
)]
#[get("/users")]
async fn reads(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    serde_json::to_string(
        &crud::user::reads(&mut conn)
            .await
            .map_err(error::ErrorInternalServerError)?,
    )
    .map_err(error::ErrorInternalServerError)
}

/// Allows to get one specific user with all assigned roles
#[utoipa::path(
    responses(
        (status = 200, description = "Success", body=UserWithRoles),
        (status = 404, description = "User not found"),
        (status = 505, description = "Internal server error"),
    ),
)]
#[get("/users/{user_id}")]
async fn read(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    serde_json::to_string(
        &crud::user::read(&mut conn, user_id.into_inner())
            .await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => error::ErrorNotFound(e),
                _ => error::ErrorInternalServerError(e),
            })?,
    )
    .map_err(error::ErrorInternalServerError)
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUser {
    pub new_name: Option<String>,
    pub new_bio: Option<String>,
}

/// Allows to update user fields
#[utoipa::path(
    request_body = UpdateUser,
    responses(
        (status = 200, description = "Success", body = User),
        (status = 404, description = "User not found"),
        (status = 505, description = "Internal server error"),
    ),
)]
#[put("/users/{user_id}")]
async fn update(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
    user_updates: web::Json<UpdateUser>,
) -> actix_web::Result<impl Responder> {
    let UpdateUser { new_name, new_bio } = user_updates.into_inner();
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    serde_json::to_string(
        &crud::user::update(&mut conn, user_id.into_inner(), new_name, new_bio)
            .await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => error::ErrorNotFound(e),
                _ => error::ErrorInternalServerError(e),
            })?,
    )
    .map_err(error::ErrorInternalServerError)
}

/// Allows to delete specific user
#[utoipa::path(
    responses(
        (status = 200, description = "Success", body = User),
        (status = 409, description = "Conflict"),
        (status = 404, description = "User not found"),
        (status = 505, description = "Internal server error"),
    ),
)]
#[delete("/users/{user_id}")]
async fn delete(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().await.map_err(error::ErrorInternalServerError)?;
    serde_json::to_string(
        &crud::user::delete(&mut conn, user_id.into_inner())
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
