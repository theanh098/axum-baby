use crate::intercept::validate::ValidatedQuery;
use axum::Json;
use axum_baby::Postgres;
use database::native_enum::{BusinessStatus, MediaSoucre};
use database::{
  prelude::{Business, Media},
  {business, media},
};
use error::AppError;
use sea_orm::DeriveModel;
use sea_orm::{
  entity::*,
  query::*,
  sea_query::{Expr, IntoCondition},
  ColumnTrait, EntityTrait, FromQueryResult, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
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

#[derive(Debug, Serialize, FromQueryResult)]
pub struct MediasOnRandBusiness {
  #[serde(skip)]
  id: i32,
  url: String,
  source: MediaSoucre,
  #[serde(skip)]
  business_id: i32,
}

#[derive(Debug, Serialize, FromQueryResult)]
pub struct RandBusiness {
  pub id: i32,
  pub name: String,
  pub overview: String,
  pub token: Option<String>,
  pub logo: Option<String>,
  pub types: Option<Vec<String>>,
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
  Postgres(conn): Postgres,
) -> Result<Json<Vec<RandBusiessWithMedias>>, AppError> {
  let RandomBusinessesQuery {
    limit,
    r#type,
    main_category,
    banner_only,
  } = query;

  // let mut query_builder = business::table
  //   .left_join(media::table)
  //   .into_boxed()
  //   .filter(business::status.eq(BusinessStatus::Approved));

  let rand_businesses = Business::find()
    .filter(business::Column::Status.eq(BusinessStatus::Approved))
    .select_only()
    .columns([
      business::Column::Id,
      business::Column::Name,
      business::Column::Overview,
      business::Column::Token,
      business::Column::Logo,
      business::Column::Types,
      business::Column::MainCategory,
      business::Column::CmcId,
    ])
    .apply_if(main_category, |query, v| {
      query.filter(business::Column::MainCategory.eq(v))
    })
    .apply_if(r#type, |query, v| {
      query.filter(business::Column::Types.contains(&v))
    })
    .apply_if(banner_only, |query, v| {
      query
        .join(
          JoinType::InnerJoin,
          media::Relation::Business
            .def()
            .rev()
            .on_condition(|_left, right| {
              Expr::col((right, media::Column::Source))
                .eq(MediaSoucre::Photo)
                .into_condition()
            }),
        )
        .group_by(business::Column::Id)
    })
    // .into_model::<RandBusiness>()
    // .into_tuple::<(i32, String)>()
    .all(&conn)
    .await?;

  let medias = rand_businesses.load_many(Media, &conn).await?;

  todo!()

  // let rand_businesses: Vec<RandBusiness> = query_builder
  //   .select(RandBusiness::as_select())
  //   .limit(limit as i64)
  //   .order(random())
  //   .load::<RandBusiness>(&mut conn)
  //   .await
  //   .unwrap();

  // let medias: Vec<MediasOnRandBusiness> = MediasOnRandBusiness::belonging_to(&rand_businesses)
  //   .select(MediasOnRandBusiness::as_select())
  //   .filter(media::source.eq(MediaSoucre::Photo))
  //   .limit(3)
  //   .load::<MediasOnRandBusiness>(&mut conn)
  //   .await
  //   .unwrap();

  // let rand_busiesses_with_medias = medias
  //   .grouped_by(&rand_businesses)
  //   .into_iter()
  //   .zip(rand_businesses)
  //   .map(|(medias, business)| RandBusiessWithMedias { business, medias })
  //   .collect::<Vec<RandBusiessWithMedias>>();

  // Ok(Json(rand_busiesses_with_medias))
}
