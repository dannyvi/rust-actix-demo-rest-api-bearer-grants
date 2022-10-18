use actix_web::Error;

use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_grants::permissions::{AttachPermissions};

use crate::jwt::claims;



pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // We just get permissions from JWT
    println!("cre , {:?}", credentials);
    let token = credentials.token();
    println!("token , {:?}", token);
    let result = claims::decode_jwt(credentials.token());
    println!("token , {:?}", result);
    match result {
        Ok(claims) => {
            println!("what?: {:?}", claims.permissions);
            req.attach(claims.permissions);
            Ok(req)
        }
        // required by `actix-web-httpauth` validator signature
        // Err(e) => Err((Error::from(e), req))
        Err(e) => {
            let permissions: Vec<String> = vec!["some".to_string()];
            println!("error permission: {:?}", permissions);
            req.attach(permissions);
            Ok(req)
        }
    }
}
