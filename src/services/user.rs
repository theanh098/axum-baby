use crate::AppState;
use crate::{database::prisma, intercept::sercurity::Guard};
use axum::{extract::State, Json};
use error::AppError;

prisma::user::select!(me {
  id
  wallet_address
  avatar_url
  background_url
  email
  last_sync_ibt
  last_update
  nickname
  did: include {
    users
  }
});

#[axum_macros::debug_handler]
#[utoipa::path(
  get,
  path = "/users",
  tag = "users",
  responses(
      (status = 200, description = "return your information")
  ),
  security(
    ("BearerAuth" = []),
  )
)]
pub async fn who_am_i(
  Guard(claims): Guard,
  State(state): State<AppState>,
) -> Result<Json<me::Data>, AppError> {
  let prisma_client = state.prisma_client;

  let me = prisma_client
    .user()
    .find_unique(prisma::user::id::equals(claims.id))
    .select(me::select())
    .exec()
    .await
    .unwrap()
    .unwrap();

  Ok(Json(me))
}
