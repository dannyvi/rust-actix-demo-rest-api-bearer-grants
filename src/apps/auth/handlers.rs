use actix_web::{web, post, Error};

use serde::Deserialize;

use crate::jwt::claims::{Claims, self};



#[derive(Deserialize, Serialize)]
pub struct UserPermissions {
    pub id: String,
    pub permissions: Vec<String>,
}

#[post("/token")]
pub async fn create_token(info: web::Json<UserPermissions>) -> Result<String, Error> {

    let user_info = info.into_inner();
    let claims = Claims::new(user_info.id, user_info.permissions);
    let jwt = claims::encode_jwt(claims)?;

    Ok(jwt)
}
