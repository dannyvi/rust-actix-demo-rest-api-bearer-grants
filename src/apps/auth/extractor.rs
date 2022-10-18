use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::HeaderValue;
use actix_web::{Error, http::header::AUTHORIZATION};

use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_grants::permissions::{AttachPermissions};

use crate::jwt::claims;

pub async fn extract(req: &ServiceRequest) -> Result<Vec<String>, Error> {
    let auth = req.headers().get("Authorization");
    println!("{:?}", auth);
    if let Some(val) = auth {
        let token = val
            .to_str()
            .unwrap()
            .split("Bearer ")
            .collect::<Vec<&str>>()
            .pop()
            .unwrap();
        let result = claims::decode_jwt(token);
        match result {
            Ok(data) => {
                // req.attach(data.permissions);
                Ok(data.permissions)
            },
            Err(e) => Err(Error::from(e))
        }
    } else {
        Ok(vec!["ANY".to_string()])
        //Err(error::ErrorUnauthorized("Invalid Authrization"))
    }
    // let auth_header: Option<&str> = req
    //     .headers()
    //     .get(AUTHORIZATION)
    //     .map(HeaderValue::to_str)
    //     .filter(Result::is_ok)
    //     .map(Result::unwrap);

    // auth_header
    //     .map(|header| {
    //         header
    //             .split(",")
    //             .map(str::to_string)
    //             .collect::<Vec<String>>()
    //     })
    //     .ok_or_else(|| ErrorUnauthorized("Authorization header incorrect!"))
}

// pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
//     // We just get permissions from JWT
//     println!("cre , {:?}", credentials);
//     let token = credentials.token();
//     println!("token , {:?}", token);
//     let result = claims::decode_jwt(credentials.token());
//     println!("token , {:?}", result);
//     match result {
//         Ok(claims) => {
//             println!("what?: {:?}", claims.permissions);
//             req.attach(claims.permissions);
//             Ok(req)
//         }
//         // required by `actix-web-httpauth` validator signature
//         // Err(e) => Err((Error::from(e), req))
//         Err(e) => {
//             let permissions: Vec<String> = vec!["some".to_string()];
//             println!("error permission: {:?}", permissions);
//             req.attach(permissions);
//             Ok(req)
//         }
//     }
// }
