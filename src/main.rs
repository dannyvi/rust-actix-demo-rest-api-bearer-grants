#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;


use crate::server::server;

mod apps;
mod cache;
mod config;
mod database;
mod errors;
mod helpers;
mod jwt;
mod routes;
mod server;
mod state;
mod schema;
mod openapi;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
