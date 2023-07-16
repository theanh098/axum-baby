use crate::database::prisma;
use crate::intercept::sercurity::Claims;
use crate::{utils, AppState};
use anyhow::Result;
use axum::{extract::State, Json};
use error::AuthError;
use ethers::types::Signature;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use siwe::Message;
use std::env;
use std::sync::Arc;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
  access_token: String,
  refresh_token: String,
  user: user_claims::Data,
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

#[axum_macros::debug_handler]
pub async fn login(
  State(state): State<AppState>,
  Json(payload): Json<AuthPayload>,
) -> Result<Json<Tokens>, AuthError> {
  let AppState {
    redis_client,
    prisma_client,
  } = state;
  let AuthPayload { signature, message } = payload;

  match message.parse::<Message>() {
    Ok(siwe_message) => {
      if let Ok(signature) = signature.as_str().parse::<Signature>() {
        match signature.verify(message, siwe_message.address) {
          Ok(_) => {
            let wallet_address = siwe::eip55(&siwe_message.address);
            let user_claims = handle_address(wallet_address, prisma_client).await;
            let tokens = generate_tokens(user_claims, redis_client).await.unwrap();
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

// expand data custom select

prisma::user::select!( user_claims {
  id
  wallet_address
  is_admin
});

async fn handle_address(
  wallet_address: String,
  prisma_client: Arc<prisma::PrismaClient>,
) -> user_claims::Data {
  let user = prisma_client
    .user()
    .find_unique(prisma::user::wallet_address::equals(
      wallet_address.to_owned(),
    ))
    .select(user_claims::select())
    .exec()
    .await
    .unwrap();

  match user {
    Some(user) => user,
    None => {
      let new_user = prisma_client
        .user()
        .create(wallet_address.to_owned(), vec![])
        .exec()
        .await
        .unwrap();

      prisma_client
        .social()
        .create(
          prisma::user::UniqueWhereParam::IdEquals(new_user.id),
          vec![],
        )
        .exec()
        .await
        .unwrap();

      user_claims::Data {
        id: new_user.id,
        wallet_address,
        is_admin: false,
      }
    }
  }
}

async fn generate_tokens(
  user_claims: user_claims::Data,
  redis_client: redis::Client,
) -> Result<Tokens> {
  let secret = env::var("JWT_SECRET")?;
  let refresh_secret = env::var("JWT_REFRESH_SECRET")?;

  let header = Header::new(Algorithm::HS256);

  let secret_key = EncodingKey::from_secret(secret.as_bytes());
  let refresh_key = EncodingKey::from_secret(refresh_secret.as_bytes());

  let access_token = encode(&header, &Claims::new_access(&user_claims), &secret_key)?;
  let refresh_token = encode(&header, &Claims::new_refresh(&user_claims), &refresh_key)?;

  let mut con = redis_client
    .get_connection()
    .expect("getting redis connection fail");

  redis::cmd("SET")
    .arg(utils::refresh_token_generate((&user_claims).id))
    .arg(&refresh_token)
    .query::<()>(&mut con)?;

  Ok(Tokens {
    access_token,
    refresh_token,
    user: user_claims,
  })
}
