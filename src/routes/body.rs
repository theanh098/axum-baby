use axum::response;
use axum_baby::ValidatedJson;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize)]
enum Category {
  #[serde(rename = "all")]
  All,
  #[serde(rename = "dev")]
  Dex,
  #[serde(rename = "cex")]
  Cex,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct MokeBody {
  #[validate(length(min = 3))]
  name: String,

  #[validate(range(min = 18))]
  age: usize,

  category: Category,
}

#[axum_macros::debug_handler]
pub async fn get_json_body(
  ValidatedJson(body): ValidatedJson<MokeBody>,
) -> response::Json<MokeBody> {
  response::Json(body)
}
