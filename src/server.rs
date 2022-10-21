use crate::apps::auth::extractor::extract;
use crate::cache::add_cache;
use crate::database::add_pool;
use crate::openapi::ApiDoc;
use crate::{config::CONFIG};
// use crate::database::add_pool;
use crate::routes::routes;
use crate::state::new_state;
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use actix_web_grants::GrantsMiddleware;
use listenfd::ListenFd;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn server() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    // Create the application state
    // String is used here, but it can be anything
    // Invoke in hanlders using data: AppState<'_, String>

    let data = new_state::<String>();

    let mut listenfd = ListenFd::from_env();

    let openapi = ApiDoc::openapi();

    let mut server = HttpServer::new(move || {
        let auth = GrantsMiddleware::with_extractor(extract);
        App::new()
            .configure(add_cache)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
            .wrap(
                Cors::default()
                .allowed_origin(&format!("http://{}", &CONFIG.server))
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS",])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .supports_credentials()
                .max_age(3600),
            )
            .wrap(Logger::default())
            .wrap(auth)
            .configure(add_pool)
            .app_data(data.clone())
            .configure(routes)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind(&CONFIG.server)?
    };

    server.run().await
}
