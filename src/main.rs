#![recursion_limit = "256"]
mod database;
mod intercept;
mod open_api;
mod schedulers;
mod services;
mod utils;
use axum::{
  routing::{get, post},
  Router,
};
use database::prisma::PrismaClient;
use dotenv::dotenv;
// use futures::prelude::*;
use open_api::ApiDoc;
use schedulers::cmc::CmcCrawling;
use std::sync::Arc;
use tokio_cron_scheduler::JobScheduler;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
  prisma_client: Arc<PrismaClient>,
  redis_conn: redis::aio::ConnectionManager,
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

  let redis_conn = redis::aio::ConnectionManager::new(
    redis::Client::open("redis://127.0.0.1/").expect("opening redis client fail"),
  )
  .await
  .unwrap();

  let app_state = AppState {
    prisma_client,
    redis_conn,
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

  let sched = JobScheduler::new().await.unwrap();
  sched.crawl_cmc().await.unwrap();
  sched.start().await.unwrap();

  // run it with hyper on localhost:8080
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

// Minnie
