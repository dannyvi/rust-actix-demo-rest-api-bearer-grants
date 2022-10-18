//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::apps::{auth::routes::routes as auth_routes, health::get_health};
use actix_web::{web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_health)
        .service(web::scope("/api/v1").service(auth_routes()));
    // Healthcheck
    //.route("/health", web::get().to(get_health))
    // /api/v1 routes
    // .service(
    //     web::scope("/api/v1")
    //         .service(
    //             web::scope("/auth")
    //                 .route("/login", web::post().to(login))
    //                 .route("/logout", web::get().to(logout).guard(IsAuthenticated)),
    //         )
    //         // USER routes
    //         .service(
    //             web::scope("/user")
    //                 .route("/{id}", web::get().to(get_user))
    //                 .route("/{id}", web::put().to(update_user))
    //                 .route("/{id}", web::delete().to(delete_user))
    //                 .route("", web::get().to(get_users))
    //                 .route("", web::post().to(create_user)),
    //         ),
    // )
    // // Serve secure static files from the static-private folder
    // .service(
    //     web::scope("/secure")
    //         // .wrap(AuthMiddleware)
    //         .service(
    //             Files::new("", "./static-secure")
    //                 .index_file("index.html")
    //                 .use_last_modified(true),
    //         ),
    // )
    // // Serve public static files from the static folder
    // .service(
    //     web::scope("").default_service(
    //         Files::new("", "./static")
    //             .index_file("index.html")
    //             .use_last_modified(true),
    //     ),
    // );
}
