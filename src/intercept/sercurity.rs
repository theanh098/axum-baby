use crate::services::auth::user_claims;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use chrono::Utc;
use error::AuthError;
use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
  pub exp: u32,
  pub id: i32,
  pub wallet_address: String,
  pub is_admin: bool,
}
pub struct Guard(pub Claims);

impl Claims {
  pub fn new_access(user_claims: &user_claims::Data) -> Self {
    Self {
      exp: Utc::now()
        .checked_add_signed(chrono::Duration::days(3))
        .unwrap()
        .timestamp() as u32,
      id: user_claims.id,
      wallet_address: user_claims.wallet_address.to_owned(),
      is_admin: user_claims.is_admin,
    }
  }
  pub fn new_refresh(user_claims: &user_claims::Data) -> Self {
    Self {
      exp: Utc::now()
        .checked_add_signed(chrono::Duration::days(60))
        .unwrap()
        .timestamp() as u32,
      id: user_claims.id,
      wallet_address: user_claims.wallet_address.to_owned(),
      is_admin: user_claims.is_admin,
    }
  }
}

#[async_trait]
impl<S> FromRequestParts<S> for Guard
where
  S: Send + Sync,
{
  type Rejection = AuthError;

  async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
    match parts.headers.get("Authorization") {
      Some(authoration_header) => {
        if authoration_header.is_empty() {
          Err(AuthError::MissingCredentials)
        } else {
          let token = authoration_header
            .to_str()
            .unwrap()
            .trim_start_matches("Bearer")
            .trim();

          let access_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

          match decode_jwt::<Claims>(token, access_secret) {
            Ok(claims) => Ok(Guard(claims)),
            Err(err) => {
              if let ErrorKind::ExpiredSignature = err.kind() {
                Err(AuthError::ExpriedCredentials)
              } else {
                Err(AuthError::WrongCredentials)
              }
            }
          }
        }
      }
      None => Err(AuthError::MissingCredentials),
    }
  }
}

pub fn decode_jwt<T: DeserializeOwned>(
  token: &str,
  secret: String,
) -> Result<T, jsonwebtoken::errors::Error> {
  match decode::<T>(
    &token,
    &DecodingKey::from_secret(secret.as_bytes()),
    &Validation::new(Algorithm::HS256),
  ) {
    Ok(decoded) => Ok(decoded.claims),
    Err(err) => Err(err),
  }
}
