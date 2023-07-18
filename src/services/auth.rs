use crate::database::schema::user;
use crate::intercept::sercurity::Claims;
use crate::utils::refresh_token_generate;
use anyhow::Result;
use axum::Json;
use axum_baby::{PgConnection, Postgres, Redis, RedisConnection};
use deadpool_redis::redis::cmd;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use error::AuthError;
use ethers::types::Signature;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use siwe::Message;
use std::env;

#[derive(Selectable, Queryable, Debug, Serialize)]
#[diesel(table_name = crate::database::schema::user)]
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
            let user_claims = handle_address(wallet_address, &mut pg_conn).await;
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

async fn handle_address(wallet_address: String, conn: &mut PgConnection) -> UserClaims {
  let user: Option<UserClaims> = user::table
    .filter(user::wallet_address.eq(wallet_address.to_owned()))
    .select(UserClaims::as_select())
    .first(conn)
    .await
    .optional()
    .unwrap();

  match user {
    Some(user) => user,
    None => {
      #[derive(Insertable)]
      #[diesel(table_name = crate::database::schema::user)]
      struct NewUser {
        wallet_address: String,
      }

      let new_user = diesel::insert_into(user::table)
        .values(NewUser { wallet_address })
        .returning(UserClaims::as_returning())
        .get_result(conn)
        .await
        .unwrap();

      // prisma_client
      //   .social()
      //   .create(
      //     prisma::user::UniqueWhereParam::IdEquals(new_user.id),
      //     vec![],
      //   )
      //   .exec()
      //   .await
      //   .unwrap();

      UserClaims {
        id: new_user.id,
        wallet_address: new_user.wallet_address,
        is_admin: false,
      }
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
