use super::sercurity::{decode_jwt, Claims};
use crate::AppState;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use std::env;

#[derive(Debug)]
pub struct Did {
  pub controller_address: String,
  pub ids: Vec<i32>,
}

#[async_trait]
impl FromRequestParts<AppState> for Option<Did> {
  type Rejection = ();

  async fn from_request_parts(
    parts: &mut Parts,
    state: &AppState,
  ) -> Result<Self, Self::Rejection> {
    match parts.headers.get("Authorization") {
      Some(authoration_header) => {
        if authoration_header.is_empty() {
          Ok(None)
        } else {
          let token = authoration_header
            .to_str()
            .unwrap()
            .trim_start_matches("Bearer")
            .trim();

          let access_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

          match decode_jwt::<Claims>(token, access_secret) {
            Ok(claims) => {
              let prisma_client = &state.prisma_client;
              use crate::database::prisma::{did, user};

              let (users, did) = tokio::join!(
                prisma_client
                  .user()
                  .find_many(vec![user::did::is(vec![did::users::some(vec![
                    user::id::equals(claims.id),
                  ])])])
                  .select(user::select!({ id }))
                  .exec(),
                prisma_client
                  .did()
                  .find_first(vec![did::users::some(vec![user::id::equals(claims.id)])])
                  .select(did::select!({ controller }))
                  .exec()
              );

              let users = users.unwrap();
              let did = did.unwrap();

              Ok(Some(Did {
                controller_address: if let Some(did) = did {
                  did.controller
                } else {
                  claims.wallet_address
                },
                ids: if users.len() > 0 {
                  users.iter().map(|u| u.id).collect()
                } else {
                  vec![claims.id]
                },
              }))
            }
            Err(_) => Ok(None),
          }
        }
      }
      None => Ok(None),
    }
  }
}
