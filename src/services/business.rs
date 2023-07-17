use crate::database::prisma;
use crate::{
  database::query_buider::QueryBuider,
  intercept::{did::Did, validate::ValidatedQuery},
  AppState,
};
use axum::extract::State;
use axum::Json;
use error::AppError;
use futures::future;
use prisma_client_rust::raw;
use serde::Deserialize;
use utoipa::IntoParams;
use validator::Validate;

prisma::business::select!(rand_business {
  id
  name
  types
  overview
  logo
  main_category
  token
  cmc_id
  medias(vec![prisma::media::source::equals(prisma::MediaSoucres::Photo)]).take(3) : select {
    url
  }
});

#[derive(Deserialize, IntoParams, Validate)]
#[into_params(parameter_in = Query)]
pub struct RandomBusinessesQuery {
  #[validate(range(min = 1))]
  limit: u32,

  r#type: Option<String>,

  main_category: Option<String>,

  banner_only: Option<bool>,
}

#[axum_macros::debug_handler]
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
  _did: Option<Did>,
  State(state): State<AppState>,
) -> Result<Json<Vec<Option<rand_business::Data>>>, AppError> {
  let prisma_client = state.prisma_client;
  let RandomBusinessesQuery {
    limit,
    r#type,
    main_category,
    banner_only,
  } = query;

  #[derive(Deserialize)]
  struct BusinessId {
    id: i32,
  }

  let mut query_builder = QueryBuider::new();
  query_builder.r#where(r#" "b"."status" = 'approved' "#);

  if let Some(b_type) = r#type {
    query_builder.and_where(format!(r#" '{b_type}' = ANY("b"."types") "#))
  }

  if let Some(main_category) = main_category {
    query_builder.and_where(format!(r#" "b"."main_category" = '{main_category}' "#))
  }

  if banner_only.unwrap_or_default() {
    query_builder.and_where(
      r#"
      (
        SELECT COUNT("m"."id") FROM "media" "m"
        WHERE "m"."business_id" = "b"."id"
        AND "m"."source" = 'Photo'
      ) > 0
      "#,
    )
  }

  let data = prisma_client
    ._query_raw::<BusinessId>(raw!(format!(
      r#"
      SELECT
       "b"."id"
      FROM "business" "b"
      {}
      ORDER BY random()
      LIMIT {limit}
      "#,
      query_builder.get_query()
    )
    .as_str()))
    .exec()
    .await?;

  let tasks = future::join_all(data.iter().map(|b| async {
    prisma_client
      .business()
      .find_unique(prisma::business::id::equals(b.id))
      .select(rand_business::select())
      .exec()
      .await
      .unwrap()
  }))
  .await;

  Ok(Json(tasks))
}
