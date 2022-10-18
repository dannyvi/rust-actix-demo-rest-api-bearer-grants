use crate::errors::ApiError;
use crate::helpers::respond_json;
use actix_web::{web::{Json}, get};
use actix_web_grants::permissions::AuthDetails;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[get("/health")]
pub async fn get_health(details: AuthDetails) -> Result<Json<HealthResponse>, ApiError> {
    println!("detail: {:?}", details.permissions);
    respond_json(HealthResponse {
        status: "ok".into(),
        version: env!("CARGO_PKG_VERSION").into(),
    })
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[actix_rt::test]
//     async fn test_get_health() {
//         let response = get_health().await.unwrap();
//         assert_eq!(response.into_inner().status, "ok".to_string());
//     }
// }
