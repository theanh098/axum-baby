use axum::{
  routing::{get, post},
  Router,
};
mod routes;

#[tokio::main]
async fn main() {
  // build our application with a single route
  let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/query", get(routes::query::get_query_string_as_a_struct))
    .route("/body", post(routes::body::get_json_body))
    .route("/path/:userid", get(routes::path::get_path));

  // run it with hyper on localhost:3000
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

// Minnie
