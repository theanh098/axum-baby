use crate::intercept::validate::ValidatedQuery;
use axum::Json;
use axum_baby::Postgres;
use database::native_enum::{BusinessStatus, MediaSoucre};
use database::{
  prelude::{Business, Media},
  {business, media},
};
use error::AppError;
use sea_orm::sea_query::{Func, PgFunc};
use sea_orm::{
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
  url: String,
  source: MediaSoucre,
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
  business: business::Model,
  medias: Vec<media::Model>,
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

  let rand_busiesses = Business::find()
    .filter(business::Column::Status.eq(BusinessStatus::Approved))
    .apply_if(main_category, |query, v| {
      query.filter(business::Column::MainCategory.eq(v))
    })
    .apply_if(r#type, |query, v| {
      query.filter(Expr::eq(
        Expr::val(v),
        Expr::expr(PgFunc::any(Expr::col((
          business::Entity,
          business::Column::Types,
        )))),
      ))
    })
    .apply_if(banner_only, |query, _v| {
      query
        .join(
          JoinType::InnerJoin,
          media::Relation::Business
            .def()
            .rev()
            .on_condition(|_business_table, media_table| {
              Expr::col((media_table, media::Column::Source))
                .eq(MediaSoucre::Photo)
                .into_condition()
            }),
        )
        .group_by(business::Column::Id)
    })
    .limit(limit as u64)
    .all(&conn)
    .await?;

  let medias = rand_busiesses.load_many(Media, &conn).await?;

  let rs = rand_busiesses
    .into_iter()
    .zip(medias)
    .map(|(business, medias)| RandBusiessWithMedias {
      business: business.into(),
      medias: medias.into(),
    })
    .collect::<Vec<RandBusiessWithMedias>>();

  Ok(Json(rs))
}

#[cfg(test)]
mod tests {
  use super::*;
  use sea_orm::{sea_query::PgFunc, DbBackend, QueryTrait};

  #[test]
  fn query_statement_check() {
    let join_stament = Business::find()
      .filter(business::Column::Status.eq(BusinessStatus::Approved))
      .apply_if(Some("cmm_main"), |query, v| {
        query.filter(business::Column::MainCategory.eq(v))
      })
      .apply_if(Some("cmm_type"), |query, v| {
        query.filter(Expr::eq(
          Expr::val(v),
          Expr::expr(PgFunc::any(Expr::col((
            business::Entity,
            business::Column::Types,
          )))),
        ))
      })
      .apply_if(None, |query, v: bool| {
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
      // .into_model::<(RandBusiness, Vec<MediasOnRandBusiness>)>()
      .limit(100 as u64)
      .build(DbBackend::Postgres)
      .to_string();

    assert_eq!(join_stament, "SELECT \"did\".\"id\", \"did\".\"controller\" FROM \"did\" INNER JOIN \"user\" ON \"did\".\"id\" = \"user\".\"did_id\" AND \"user\".\"id\" = 12");
  }
}
