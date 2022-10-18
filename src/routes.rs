use crate::apps::{
    auth::routes::routes as auth_routes, demo::routes::routes as demo_routes, health::get_health,
};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_health).service(
        web::scope("/api/v1")
            .service(auth_routes())
            .service(demo_routes()),
    );
}
