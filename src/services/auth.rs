use crate::intercept::sercurity::Claims;
use crate::utils::refresh_token_generate;
use anyhow::Result;
use axum::Json;
use axum_baby::{Postgres, Redis, RedisConnection};
use database::prelude::{Social, User};
use database::{social, user};
use deadpool_redis::redis::cmd;
use error::AuthError;
use ethers::types::Signature;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sea_orm::{
  ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, QueryFilter,
};
use serde::{Deserialize, Serialize};
use siwe::Message;
use std::env;

#[derive(FromQueryResult, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserClaims {
  pub id: i32,
  pub wallet_address: String,
  pub is_admin: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
  access_token: String,
  refresh_token: String,
  user: UserClaims,
}

#[derive(Deserialize)]
pub struct AuthPayload {
  message: String,
  signature: String,
}

#[axum_macros::debug_handler]
pub async fn get_nonce() -> String {
  siwe::generate_nonce()
}

// #[axum_macros::debug_handler]
pub async fn login(
  Postgres(mut pg_conn): Postgres,
  Redis(mut redis_conn): Redis,
  Json(payload): Json<AuthPayload>,
) -> Result<Json<Tokens>, AuthError> {
  let AuthPayload { signature, message } = payload;

  match message.parse::<Message>() {
    Ok(siwe_message) => {
      if let Ok(signature) = signature.as_str().parse::<Signature>() {
        match signature.verify(message, siwe_message.address) {
          Ok(_) => {
            let wallet_address = siwe::eip55(&siwe_message.address);
            let user_claims = handle_address(wallet_address, &mut pg_conn).await.unwrap();
            let tokens = generate_tokens(user_claims, &mut redis_conn).await.unwrap();
            Ok(Json(tokens))
          }
          Err(err) => {
            dbg!(err);
            Err(AuthError::WrongSignature)
          }
        }
      } else {
        Err(AuthError::WrongSignature)
      }
    }
    Err(_err) => Err(AuthError::WrongSignature),
  }
}

async fn handle_address(
  wallet_address: String,
  conn: &DatabaseConnection,
) -> anyhow::Result<UserClaims> {
  let user = User::find()
    .filter(user::Column::WalletAddress.eq(&wallet_address))
    .into_model::<UserClaims>()
    .one(conn)
    .await?;

  match user {
    Some(user) => Ok(user),
    None => {
      let new_user = User::insert(user::ActiveModel {
        wallet_address: Set(wallet_address.clone()),
        ..Default::default()
      })
      .exec(conn)
      .await?;

      Social::insert(social::ActiveModel {
        user_id: Set(new_user.last_insert_id),
        ..Default::default()
      })
      .exec(conn)
      .await?;

      Ok(UserClaims {
        id: new_user.last_insert_id,
        wallet_address,
        is_admin: false,
      })
    }
  }
}

async fn generate_tokens(
  user_claims: UserClaims,
  redis_conn: &mut RedisConnection,
) -> Result<Tokens> {
  let secret = env::var("JWT_SECRET")?;
  let refresh_secret = env::var("JWT_REFRESH_SECRET")?;

  let header = Header::new(Algorithm::HS256);

  let secret_key = EncodingKey::from_secret(secret.as_bytes());
  let refresh_key = EncodingKey::from_secret(refresh_secret.as_bytes());

  let access_token = encode(&header, &Claims::new_access(&user_claims), &secret_key)?;
  let refresh_token = encode(&header, &Claims::new_refresh(&user_claims), &refresh_key)?;

  cmd("SET")
    .arg(refresh_token_generate((&user_claims).id))
    .arg(&refresh_token)
    .query_async(redis_conn)
    .await?;

  Ok(Tokens {
    access_token,
    refresh_token,
    user: user_claims,
  })
}
