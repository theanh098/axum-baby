use axum::extract::Path;

pub async fn get_path(Path(user_id): Path<u32>) -> String {
  format!("{user_id}")
}
