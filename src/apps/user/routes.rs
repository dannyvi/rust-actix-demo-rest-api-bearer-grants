use actix_web::web;

use super::handlers::*;

pub fn user_scope() -> actix_web::Scope {
    web::scope("")
        .service(find_all)
}
