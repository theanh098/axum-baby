use super::sercurity::{decode_jwt, Claims};
use crate::database::schema::{did, user};
use axum::{
  async_trait,
  extract::{FromRef, FromRequestParts},
  http::request::Parts,
};
use axum_baby::{internal_error, PgPool};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use std::env;

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
  PgPool: FromRef<S>,
{
  type Rejection = ();

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
              let pg_pool = PgPool::from_ref(state);
              let mut conn = pg_pool.get().await.map_err(internal_error).unwrap();

              let did: Option<(i32, String)> = did::table
                .inner_join(user::table.on(user::id.eq(claims.id)))
                .select((did::id, did::controller))
                .first(&mut conn)
                .await
                .optional()
                .unwrap();

              if let Some((did_id, controller_address)) = did {
                let users = user::table
                  .select(user::id)
                  .filter(user::did_id.eq(did_id))
                  .load::<i32>(&mut conn)
                  .await
                  .unwrap();

                Ok(OptionalGuard(Some(UserIdentifier {
                  controller_address,
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
