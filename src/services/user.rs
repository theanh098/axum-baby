// use crate::intercept::sercurity::Guard;
// use axum::Json;
use axum_baby::Postgres;
use error::AppError;

// use error::AppError;
// use crate::database::{model::User, schema::user};
use crate::database::schema::rate_business;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(serde::Serialize, Selectable, Queryable, Debug)]
#[diesel(table_name = rate_business)]
pub struct RateBusines {
  pub valuer_id: i32,
  pub business_id: i32,
  pub rating: i32,
}

// #[axum_macros::debug_handler]
#[utoipa::path(
  get,
  path = "/users",
  tag = "user",
  responses(
      (status = 200, description = "return your information")
  ),
  security(
    ("BearerAuth" = []),
  )
)]
pub async fn who_am_i(Postgres(mut conn): Postgres) -> Result<String, AppError> {
  let users = rate_business::table
    .select(RateBusines::as_select())
    .limit(12)
    .load(&mut conn)
    .await
    .unwrap();

  dbg!(users);

  Ok("fsa".into())
}
