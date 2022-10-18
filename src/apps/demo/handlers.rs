use actix_web::{get, HttpResponse};
use actix_web_grants::proc_macro::{has_permissions, has_any_role};

use crate::apps::{auth::permissions::is_user, };
use crate::errors::ApiError;

#[get("/admin")]
#[has_permissions("OP_GET_SECURED_INFO")]
// For the user with permission `OP_GET_SECURED_INFO` - endpoint will give the HTTP status 200, otherwise - 403
// You can check via cURL (for generate you own token, use `/token` handler):
// ```sh
//  curl --location --request GET 'http://localhost:8080/api/admin' \
//  --header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IkxvcmVtLUlwc3VtIiwicGVybWlzc2lvbnMiOlsiT1BfR0VUX1NFQ1VSRURfSU5GTyJdLCJleHAiOjE5MjY2ODk3MTF9.vFZ6qYRhJ4KY3trXvIAnhTed8UXxCw2tCSB4Qz5D7So'
// ```
async fn permission_secured() -> HttpResponse {
    HttpResponse::Ok().json("aa")
}

// #[get("/access")]
#[get("/manager")]
#[has_any_role("ADMIN", error = "is_user")]
// #[has_any_role("ADMIN", "MANAGER")]
// For the `ADMIN` or `MANAGER` - endpoint will give the HTTP status 200, otherwise - 403
// You can check via cURL (for generate you own token, use `/token` handler):
// ```sh
//  curl --location --request GET 'http://localhost:8080/api/manager' \
//  --header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IkxvcmVtLUlwc3VtIiwicGVybWlzc2lvbnMiOlsiUk9MRV9NQU5BR0VSIl0sImV4cCI6MTkyNjY5MDYxN30.AihInANG_8gp5IZk5gak9-Sv_ankb740FfNepzhZojw'
// ```
async fn manager_secured() -> HttpResponse {
    HttpResponse::Ok().json("bb")
}

#[get("/error")]
pub async fn error_internal() -> Result<String, ApiError> {
    println!("gg");
    Err(ApiError::InternalError)
}
