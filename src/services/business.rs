use axum::extract::State;

use crate::{intercept::did::Did, AppState};

#[axum_macros::debug_handler]
pub async fn get_businesses(did: Option<Did>, State(state): State<AppState>) -> String {
  dbg!(did);

  "saf".into()
}
