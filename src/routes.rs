use crate::apps::{
    auth::routes::routes as auth_routes, 
    demo::routes::routes as demo_routes,
    user::routes::user_scope,
    health::get_health,
};
use actix_web::web::{self, service};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_health).service(
        web::scope("/api/v1")
            .service(auth_routes())
            .service(demo_routes())
            .service(user_scope())
    );
}
