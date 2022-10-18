use actix_web::{web, post, Error};

use serde::Deserialize;

use crate::jwt::claims::{Claims, self};



#[derive(Deserialize)]
pub struct UserPermissions {
    pub id: String,
    pub permissions: Vec<String>,
}


// An additional handler for generating a token.
// Thus, you can try to create your own tokens and check the operation of the permissions system.
// cURL example:
// ```sh
//  curl --location --request POST 'http://localhost:8080/token' \
//   --header 'Content-Type: application/json' \
//   --data-raw '{
//       "username": "Lorem-Ipsum",
//       "permissions": ["OP_GET_SECURED_INFO"]
//   }'
// ```
#[post("/token")]
pub async fn create_token(info: web::Json<UserPermissions>) -> Result<String, Error> {
    
    let user_info = info.into_inner();
    // Create a JWT
    let claims = Claims::new(user_info.id, user_info.permissions);
    let jwt = claims::encode_jwt(claims)?;

    // Return token for work with example handlers
    Ok(jwt)
}
