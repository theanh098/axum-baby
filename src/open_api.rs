use crate::services::user::__path_who_am_i;
use utoipa::{
  openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
  Modify, OpenApi, ToResponse,
};

#[derive(OpenApi)]
#[openapi(
  info(
    version = "1.1.0",
    license (name = "MIT", url = ""),
    description = "Axum baby api interface",
    contact (name = "thoanh098", url = "https://github.com/theanh098")
  ),
  paths(
      who_am_i,
    ),
    components(
      schemas(),
      responses(App)
    ),
    modifiers(&BearerSecurity),
    tags(
      (name = "Baby dashboard"),
    )
  )]
pub struct ApiDoc;

struct BearerSecurity;
// Just trigger modify for BearerSecurity security
#[derive(ToResponse)]
struct App;

impl Modify for BearerSecurity {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    if let Some(components) = openapi.components.as_mut() {
      components.add_security_scheme(
        "BearerAuth",
        SecurityScheme::Http(
          HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            .bearer_format("JWT")
            .build(),
        ),
      );
    }
  }
}
