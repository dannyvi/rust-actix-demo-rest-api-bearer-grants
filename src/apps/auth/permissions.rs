use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};

use crate::errors::ApiError;

pub(crate) fn access_denied() -> HttpResponse {
    HttpResponse::with_body(
        StatusCode::FORBIDDEN,
        BoxBody::new("This resource allowed only for ADMIN"),
    )
}

/// Create a json web token (JWT)
pub(crate) fn is_user() -> HttpResponse {
    ApiError::PermissionDenied {
        detail: "Wrong".to_string(),
    }
    .error_response()
}
