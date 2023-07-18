use crate::database::schema::{business, media};
use crate::intercept::validate::ValidatedQuery;
use axum::Json;
use axum_baby::Postgres;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use error::AppError;
use serde::Deserialize;
use serde::Serialize;
use utoipa::IntoParams;
use validator::Validate;

#[derive(Deserialize, IntoParams, Validate)]
#[into_params(parameter_in = Query)]
pub struct RandomBusinessesQuery {
  #[validate(range(min = 1))]
  limit: u32,

  r#type: Option<String>,

  main_category: Option<String>,

  banner_only: Option<bool>,
}

#[derive(Queryable, Debug, Identifiable, Serialize, Selectable)]
#[diesel(table_name = crate::database::schema::business)]
pub struct RandBusines {
  pub id: i32,
  pub name: String,
  pub overview: String,
  pub token: Option<String>,
  pub logo: Option<String>,
  pub types: Option<Vec<Option<String>>>,
  pub main_category: String,
  pub cmc_id: Option<i32>,
}

// #[axum_macros::debug_handler]
#[utoipa::path(
  get,
  params(
    RandomBusinessesQuery
  ),
  path = "/businesses",
  tag = "business",
  responses(
      (status = 200, description = "return list businesses")
  ),
  security(
    ("BearerAuth" = []),
  )
)]
pub async fn get_businesses(
  ValidatedQuery(query): ValidatedQuery<RandomBusinessesQuery>,
  Postgres(mut conn): Postgres,
) -> Result<Json<Vec<RandBusines>>, AppError> {
  let RandomBusinessesQuery {
    limit,
    r#type,
    main_category,
    banner_only,
  } = query;

  let mut query_builder = business::table.into_boxed();

  // let mut query_builder = QueryBuider::new();
  // query_builder.r#where(r#" "b"."status" = 'approved' "#);

  // if let Some(b_type) = r#type {
  //   query_builder.and_where(format!(r#" '{b_type}' = ANY("b"."types") "#))
  // }

  // if let Some(main_category) = main_category {
  //   query_builder.and_where(format!(r#" "b"."main_category" = '{main_category}' "#))
  // }

  // if banner_only.unwrap_or_default() {
  //   query_builder.and_where(
  //     r#"
  //     (
  //       SELECT COUNT("m"."id") FROM "media" "m"
  //       WHERE "m"."business_id" = "b"."id"
  //       AND "m"."source" = 'Photo'
  //     ) > 0
  //     "#,
  //   )
  // }

  // let data = prisma_client
  //   ._query_raw::<BusinessId>(raw!(format!(
  //     r#"
  //     SELECT
  //      "b"."id"
  //     FROM "business" "b"
  //     {}
  //     ORDER BY random()
  //     LIMIT {limit}
  //     "#,
  //     query_builder.get_query()
  //   )
  //   .as_str()))
  //   .exec()
  //   .await?;

  // let tasks = future::join_all(data.iter().map(|b| async {
  //   prisma_client
  //     .business()
  //     .find_unique(prisma::business::id::equals(b.id))
  //     .select(rand_business::select())
  //     .exec()
  //     .await
  //     .unwrap()
  // }))
  // .await;

  // Ok(Json(tasks))

  todo!()
}
