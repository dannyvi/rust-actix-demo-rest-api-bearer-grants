use actix_web::{get, http::StatusCode, web, HttpResponse, ResponseError};

use crate::{
    database::{InferPool, Pool, PoolType},
    errors::{ApiError, ErrorResponse},
};

use super::models::User;

#[utoipa::path(
    tag = "users",
    responses(
        (status = 200, description = "List users", body = [User]),
        (
            status = 409,
            description = "Permission Denied",
            body = ErrorResponse, 
            example = json!(ErrorResponse::from(
                &ApiError::PermissionDenied("You are not allowed".to_string())))
        )
    )
)]
#[get("/users")]
pub async fn find_all(pool: web::Data<PoolType>) -> Result<HttpResponse, ApiError> {
    // Result<Vec<User>, ApiError> {
    match User::find_all(&pool.get().unwrap()) {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(_) => Err(ApiError::InternalError),
    }
}
