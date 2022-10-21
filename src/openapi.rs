use utoipa::{
    openapi::{
        security::{ApiKey, ApiKeyValue, SecurityScheme},
        ServerBuilder, ServerVariableBuilder,
    },
    Modify, OpenApi,
};

use crate::apps::user;

#[derive(OpenApi)]
#[openapi(
    paths(user::handlers::find_all),
    components(schemas(user::models::User)),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "api_key",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );
        openapi.servers = Some(vec![ServerBuilder::new()
            .url("/api/v1/")
            .description(Some("this is description of the server"))
            .parameter(
                "username",
                ServerVariableBuilder::new()
                    .default_value("the_user")
                    .description(Some("this is user")),
            )
            .build()]);
    }
}
