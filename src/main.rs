#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

// use actix_web::{web, App, HttpServer};

// use actix_web_httpauth::middleware::HttpAuthentication;
// use apps::demo::handlers::{manager_secured, permission_secured, error_internal};
// use auth::handlers::create_token;
// use auth::validator::validator;
// use config::CONFIG;
use crate::server::server;
mod apps;
// mod auth;
mod config;
mod jwt;
mod errors;
mod helpers;
mod routes;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
    // HttpServer::new(|| {
    //     let auth = HttpAuthentication::bearer(validator);
    //     App::new().service(create_token).service(
    //         web::scope("/api")
    //             .wrap(auth)
    //             .service(permission_secured)
    //             .service(manager_secured)
    //             .service(error_internal),
    //     )
    // })
    // .bind(&CONFIG.server)?
    // .workers(1)
    // .run()
    // .await
}
