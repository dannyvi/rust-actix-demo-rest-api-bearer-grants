use actix_web::{get, HttpResponse};
use actix_web_grants::proc_macro::{has_permissions, has_any_role};

use crate::apps::{auth::permissions::is_user, };
use crate::errors::ApiError;

#[get("/admin")]
#[has_permissions("OP_GET_SECURED_INFO", error="is_user")]
async fn permission_secured() -> HttpResponse {
    HttpResponse::Ok().json("aa")
}

#[get("/manager")]
#[has_any_role("ADMIN", error = "is_user")]
async fn manager_secured() -> HttpResponse {
    HttpResponse::Ok().json("bb")
}

#[get("/error")]
pub async fn error_internal() -> Result<String, ApiError> {
    println!("gg");
    Err(ApiError::InternalError)
}
