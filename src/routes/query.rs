use axum::response;
use axum_baby::ValidatedQuery;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct Pagination {
  #[validate(range(min = 1))]
  page: Option<usize>,

  #[validate(range(max = 100))]
  per_page: Option<usize>,
}

#[axum_macros::debug_handler]
pub async fn get_query_string_as_a_struct(
  ValidatedQuery(pagination): ValidatedQuery<Pagination>,
) -> response::Json<Pagination> {
  response::Json(pagination)
}
