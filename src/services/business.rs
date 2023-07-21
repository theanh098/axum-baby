use self::response::*;
use crate::intercept::validate::ValidatedQuery;
use axum::Json;
use axum_baby::Postgres;
use database::native_enum::{BusinessStatus, MediaSource};
use database::{
  prelude::{Business, Media},
  {business, media},
};
use error::AppError;
use sea_orm::sea_query::{Alias, PgFunc};
use sea_orm::{
  query::*,
  sea_query::{Expr, IntoCondition},
  ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use serde::Deserialize;
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
                .cast_as(Alias::new("text"))
                .eq(MediaSource::Photo)
                .into_condition()
            }),
        )
        .group_by(business::Column::Id)
    })
    .order_by_asc(Expr::cust("random()"))
    .limit(limit as u64)
    .all(&conn)
    .await?;

  let medias = rand_busiesses
    .load_many(
      Media::find().filter(media::Column::Source.eq(MediaSource::Photo)),
      &conn,
    )
    .await?;

  let rs = rand_busiesses
    .into_iter()
    .zip(medias)
    .map(|(business, medias)| RandBusiessWithMedias {
      business: business.into(),
      medias: medias
        .into_iter()
        .take(3)
        .map(|m| m.into())
        .collect::<Vec<MediasOnRandBusiness>>(),
    })
    .collect::<Vec<RandBusiessWithMedias>>();

  Ok(Json(rs))
}

#[derive(Deserialize, IntoParams, Validate)]
#[into_params(parameter_in = Query)]
pub struct BusinessesCategoryQuery {
  types: Option<String>,
  main_category: Option<String>,
  chain: Option<String>,
  sort: Option<Sort>,
}
#[derive(Deserialize)]
enum Sort {
  Lastest,
  BestRating,
  MostReview,
}

impl Default for Sort {
  fn default() -> Self {
    Self::Lastest
  }
}
#[utoipa::path(
  get,
  params(
    BusinessesCategoryQuery
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
pub async fn get_by_categories(
  ValidatedQuery(query): ValidatedQuery<BusinessesCategoryQuery>,
  Postgres(conn): Postgres,
) -> Result<Json<Vec<business::Model>>, AppError> {
  let BusinessesCategoryQuery {
    chain,
    main_category,
    sort,
    types,
  } = query;

  let main_category = if types.is_none() { main_category } else { None };

  let busiesses = Business::find()
    .filter(business::Column::Status.eq(BusinessStatus::Approved))
    .apply_if(main_category, |query, main_category| {
      query.filter(business::Column::MainCategory.eq(main_category))
    })
    .apply_if(chain, |query, chain| {
      query.filter(Expr::eq(
        Expr::val(chain),
        Expr::expr(PgFunc::any(Expr::col((
          business::Entity,
          business::Column::Chains,
        )))),
      ))
    })
    .apply_if(types, |query, types| {
      query.filter(Expr::cust_with_exprs(
        "ARRAY_CAT($1,$2)::text[] && ARRAY[$3]",
        [
          Expr::col(business::Column::Types).into(),
          Expr::col(business::Column::Tags).into(),
          Expr::val(types).into(),
        ],
      ))
    })
    .all(&conn)
    .await?;

  Ok(Json(busiesses))
}

mod response {

  use database::{business, media, native_enum::MediaSource};
  use sea_orm::FromQueryResult;
  use serde::Serialize;

  #[derive(Debug, Serialize, FromQueryResult)]
  pub struct MediasOnRandBusiness {
    url: String,
    source: MediaSource,
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
    pub business: RandBusiness,
    pub medias: Vec<MediasOnRandBusiness>,
  }

  impl From<business::Model> for RandBusiness {
    fn from(bn: business::Model) -> Self {
      Self {
        id: bn.id,
        name: bn.name,
        overview: bn.overview,
        token: bn.token,
        logo: bn.logo,
        types: bn.types,
        main_category: bn.main_category,
        cmc_id: bn.cmc_id,
      }
    }
  }

  impl From<media::Model> for MediasOnRandBusiness {
    fn from(media: media::Model) -> Self {
      Self {
        url: media.url,
        source: media.source,
      }
    }
  }
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
      .apply_if(Some(true), |query, _v: bool| {
        query
          .join(
            JoinType::InnerJoin,
            media::Relation::Business
              .def()
              .rev()
              .on_condition(|_left, right| {
                Expr::col((right, media::Column::Source))
                  .cast_as(Alias::new("text"))
                  .eq(MediaSource::Photo)
                  .into_condition()
              }),
          )
          .group_by(business::Column::Id)
      })
      .order_by_asc(Expr::cust("random()"))
      .limit(100 as u64)
      .build(DbBackend::Postgres)
      .to_string();

    assert_eq!(join_stament, "SELECT \"did\".\"id\", \"did\".\"controller\" FROM \"did\" INNER JOIN \"user\" ON \"did\".\"id\" = \"user\".\"did_id\" AND \"user\".\"id\" = 12");
  }
}
