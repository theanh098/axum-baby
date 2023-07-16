mod database;
mod intercept;
mod open_api;
mod services;
mod utils;
use axum::{
  routing::{get, post},
  Router,
};
use database::prisma::PrismaClient;
use dotenv::dotenv;
use open_api::ApiDoc;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
  prisma_client: Arc<PrismaClient>,
  redis_client: redis::Client,
}

#[tokio::main]
async fn main() {
  dotenv().ok();

  let prisma_client = Arc::new(
    PrismaClient::_builder()
      .build()
      .await
      .expect("creating prisma was wrong"),
  );

  let redis_client = redis::Client::open("redis://127.0.0.1/").expect("opening redis client fail");

  let app_state = AppState {
    prisma_client,
    redis_client,
  };

  let app = Router::new()
    .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .route("/auth/nonce", get(services::auth::get_nonce))
    .route("/auth/login", post(services::auth::login))
    .route("/users", get(services::user::who_am_i))
    .route("/businesses", get(services::business::get_businesses))
    .layer(
      CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any),
    )
    .with_state(app_state);

  // run it with hyper on localhost:8080
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

// Minnie
