// #![recursion_limit = "256"]
mod intercept;
mod open_api;
mod schedulers;
mod services;
mod utils;
use axum::{
  routing::{get, post},
  Router,
};
use axum_baby::AppState;
use dotenv::dotenv;
use open_api::ApiDoc;
// use schedulers::cmc::CmcCrawling;
// use tokio_cron_scheduler::JobScheduler;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
  dotenv().ok();

  let app = Router::new()
    .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .route("/auth/nonce", get(services::auth::get_nonce))
    .route("/auth/login", post(services::auth::login))
    .route("/users", get(services::user::who_am_i))
    .route(
      "/rand-businesses",
      get(services::business::get_rand_businesses),
    )
    .layer(
      CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any),
    )
    .with_state(AppState::new().await.unwrap());

  // let sched = JobScheduler::new().await.unwrap();
  // sched.crawl_cmc().await.unwrap();
  // sched.start().await.unwrap();

  // run it with hyper on localhost:8080
  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

// Minnie
