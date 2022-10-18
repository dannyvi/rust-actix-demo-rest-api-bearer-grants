use actix_web::web;

use super::handlers::*;

pub fn routes() -> actix_web::Scope {
    web::scope("/demo")
        .service(permission_secured)
        .service(manager_secured)
        .service(error_internal)
}
