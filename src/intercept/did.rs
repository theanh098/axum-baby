// use super::sercurity::{decode_jwt, Claims};
use axum::{
  async_trait,
  extract::{FromRef, FromRequestParts},
  http::request::Parts,
};
use database::prelude::{Did, User};
use database::{did, user};
use error::AppError;
use sea_orm::{
  sea_query::{Expr, IntoCondition},
  ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, JoinType, QueryFilter,
  QuerySelect, RelationTrait,
};
use std::env;

use super::sercurity::{decode_jwt, Claims};

#[derive(FromQueryResult)]
pub struct UserIdentifier {
  pub controller_address: String,
  pub ids: Vec<i32>,
  pub id: i32,
  pub wallet_address: String,
}

pub struct OptionalGuard(pub Option<UserIdentifier>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalGuard
where
  S: Send + Sync,
  DatabaseConnection: FromRef<S>,
{
  type Rejection = AppError;

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    match parts.headers.get("Authorization") {
      Some(authoration_header) => {
        if authoration_header.is_empty() {
          Ok(OptionalGuard(None))
        } else {
          let token = authoration_header
            .to_str()
            .unwrap()
            .trim_start_matches("Bearer")
            .trim();

          let access_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

          match decode_jwt::<Claims>(token, access_secret) {
            Ok(claims) => {
              let conn = DatabaseConnection::from_ref(state);

              let did = Did::find()
                .select_only()
                .columns([did::Column::Id, did::Column::Controller])
                .join(
                  JoinType::InnerJoin,
                  user::Relation::Did
                    .def()
                    .rev()
                    .on_condition(move |_did_table, user_table| {
                      Expr::col((user_table, user::Column::Id))
                        .eq(claims.id)
                        .into_condition()
                    }),
                )
                .into_tuple::<(i32, String)>()
                .one(&conn)
                .await?;

              if let Some((did_id, controller)) = did {
                let users = User::find()
                  .filter(user::Column::DidId.eq(did_id))
                  .select_only()
                  .column(user::Column::Id)
                  .into_tuple::<i32>()
                  .all(&conn)
                  .await?;

                Ok(OptionalGuard(Some(UserIdentifier {
                  controller_address: controller,
                  ids: users,
                  id: claims.id,
                  wallet_address: claims.wallet_address,
                })))
              } else {
                Ok(OptionalGuard(Some(UserIdentifier {
                  controller_address: claims.wallet_address.clone(),
                  ids: vec![claims.id],
                  id: claims.id,
                  wallet_address: claims.wallet_address,
                })))
              }
            }
            Err(_) => Ok(OptionalGuard(None)),
          }
        }
      }
      None => Ok(OptionalGuard(None)),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use database::{did, user};
  use sea_orm::{DbBackend, QueryTrait};

  #[test]
  fn query_statement_check() {
    let join_stament = Did::find()
      .select_only()
      .columns([did::Column::Id, did::Column::Controller])
      .join(
        JoinType::InnerJoin,
        user::Relation::Did
          .def()
          .rev()
          .on_condition(|_left, right| {
            Expr::col((right, user::Column::Id)).eq(12).into_condition()
          }),
      )
      .build(DbBackend::Postgres)
      .to_string();

    assert_eq!(join_stament, "SELECT \"did\".\"id\", \"did\".\"controller\" FROM \"did\" INNER JOIN \"user\" ON \"did\".\"id\" = \"user\".\"did_id\" AND \"user\".\"id\" = 12");
  }
}
