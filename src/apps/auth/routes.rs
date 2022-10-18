use actix_web::web;

use super::handlers::create_token;


pub fn routes () -> actix_web::Scope {
    web::scope("/auth").service(create_token)
}