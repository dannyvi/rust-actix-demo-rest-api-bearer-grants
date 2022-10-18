#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

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
}
