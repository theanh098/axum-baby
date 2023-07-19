use crate::database::enum_type::{BusinessStatus, MediaSoucre};
use crate::database::func::random;
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

#[derive(Queryable, Debug, Serialize, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(RandBusiness, foreign_key = business_id))]
#[diesel(table_name = crate::database::schema::media)]
pub struct MediasOnRandBusiness {
  #[serde(skip)]
  id: i32,
  url: String,
  source: MediaSoucre,
  #[serde(skip)]
  business_id: i32,
}

#[derive(Queryable, Debug, Identifiable, Serialize, Selectable)]
#[diesel(table_name = crate::database::schema::business)]
pub struct RandBusiness {
  pub id: i32,
  pub name: String,
  pub overview: String,
  pub token: Option<String>,
  pub logo: Option<String>,
  pub types: Option<Vec<Option<String>>>,
  pub main_category: String,
  pub cmc_id: Option<i32>,
}

#[derive(Serialize)]
pub struct RandBusiessWithMedias {
  #[serde(flatten)]
  business: RandBusiness,
  medias: Vec<MediasOnRandBusiness>,
}
// #[axum_macros::debug_handler]
#[utoipa::path(
  get,
  params(
    RandomBusinessesQuery
  ),
  path = "/rand-businesses",
  tag = "business",
  responses(
      (status = 200, description = "return list random businesses")
  ),
  security(
    ("BearerAuth" = []),
  )
)]
pub async fn get_rand_businesses(
  ValidatedQuery(query): ValidatedQuery<RandomBusinessesQuery>,
  Postgres(mut conn): Postgres,
) -> Result<Json<Vec<RandBusiessWithMedias>>, AppError> {
  let RandomBusinessesQuery {
    limit,
    r#type,
    main_category,
    banner_only,
  } = query;

  let mut query_builder = business::table
    .left_join(media::table)
    .into_boxed()
    .filter(business::status.eq(BusinessStatus::Approved));

  if let Some(b_type) = r#type {
    query_builder = query_builder.filter(business::types.contains(vec![b_type]));
  }

  if let Some(b_main_category) = main_category {
    query_builder = query_builder.filter(business::main_category.eq(b_main_category));
  }

  // if banner_only.unwrap_or_default() {
  //   query_builder = query_builder.
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

  let rand_businesses: Vec<RandBusiness> = query_builder
    .select(RandBusiness::as_select())
    .limit(limit as i64)
    .order(random())
    .load::<RandBusiness>(&mut conn)
    .await
    .unwrap();

  let medias: Vec<MediasOnRandBusiness> = MediasOnRandBusiness::belonging_to(&rand_businesses)
    .select(MediasOnRandBusiness::as_select())
    .filter(media::source.eq(MediaSoucre::Photo))
    .limit(3)
    .load::<MediasOnRandBusiness>(&mut conn)
    .await
    .unwrap();

  let rand_busiesses_with_medias = medias
    .grouped_by(&rand_businesses)
    .into_iter()
    .zip(rand_businesses)
    .map(|(medias, business)| RandBusiessWithMedias { business, medias })
    .collect::<Vec<RandBusiessWithMedias>>();

  Ok(Json(rand_busiesses_with_medias))
}
