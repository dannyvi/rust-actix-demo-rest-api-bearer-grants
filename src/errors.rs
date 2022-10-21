use actix_web::{error, http::StatusCode, HttpResponse, ResponseError};
use chrono::Local;
use derive_more::{Display, TryInto};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Display, TryInto)]
pub enum ApiError {
    CacheError(String),
    #[display(fmt = "Internal Error")]
    InternalError,
    #[display(fmt = "Unauthorized")]
    Unauthorized(String),
    #[display(fmt = "Validation Error")]
    ValidationError(String),
    #[display(fmt = "Permission Denied")]
    PermissionDenied(String),
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::InternalError | Self::CacheError(_) => {
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
            _ => HttpResponse::build(self.status_code()).json(ErrorResponse::from(self)),
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::PermissionDenied(_) => StatusCode::FORBIDDEN,
            Self::ValidationError(_) => StatusCode::EXPECTATION_FAILED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    code: u16,
    error: String,
    timestamp: String,
    detail: String,
}

impl From<&ApiError> for ErrorResponse {
    fn from(error: &ApiError) -> Self {
        let date = Local::now();
        ErrorResponse {
            code: error.status_code().as_u16(),
            error: error.to_string(),
            timestamp: date.format("%Y-%m-%d %H:%M:%S").to_string(),
            detail: match error {
                ApiError::InternalError => String::from(""),
                _ => String::try_from(error.clone()).unwrap(),
            },
        }
    }
}
