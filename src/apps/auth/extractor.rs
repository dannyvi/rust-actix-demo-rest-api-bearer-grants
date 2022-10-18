use actix_web::dev::ServiceRequest;
use actix_web::{http::header::AUTHORIZATION, Error};

use crate::jwt::claims;

use super::roles::Role;

pub async fn extract(req: &ServiceRequest) -> Result<Vec<String>, Error> {
    let auth = req.headers().get(AUTHORIZATION);
    match auth {
        Some(val) => {
            let token = val
                .to_str()
                .unwrap()
                .split("Bearer ")
                .collect::<Vec<&str>>()
                .pop()
                .unwrap();
            let result = claims::decode_jwt(token);
            match result {
                Ok(data) => Ok(data.permissions),
                Err(e) => Err(Error::from(e)),
            }
        }
        None => Ok(vec![Role::Any.to_string()]),
    }
}
