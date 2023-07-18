use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::{async_trait, extract::FromRef};
use deadpool_redis::{Config, Runtime};
use diesel_async::pooled_connection::{
  deadpool::{Object, Pool as DbPool},
  AsyncDieselConnectionManager,
};
use diesel_async::AsyncPgConnection;

#[derive(Clone)]
pub struct AppState {
  pg_pool: PgPool,
  redis_pool: RedisPool,
}

impl AppState {
  pub fn new() -> Self {
    let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");

    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pg_pool = DbPool::builder(config).build().unwrap();

    let cfg = Config::from_url("redis://127.0.0.1/");
    let redis_pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

    Self {
      pg_pool,
      redis_pool,
    }
  }
}

impl FromRef<AppState> for PgPool {
  fn from_ref(app_state: &AppState) -> PgPool {
    app_state.pg_pool.clone()
  }
}

impl FromRef<AppState> for RedisPool {
  fn from_ref(app_state: &AppState) -> RedisPool {
    app_state.redis_pool.clone()
  }
}

pub type PgConnection = Object<AsyncPgConnection>;
pub type RedisConnection = deadpool_redis::Connection;

pub struct Postgres(pub PgConnection);
pub struct Redis(pub RedisConnection);

pub type PgPool = DbPool<AsyncPgConnection>;
pub type RedisPool = deadpool_redis::Pool;

#[async_trait]
impl<S> FromRequestParts<S> for Postgres
where
  S: Send + Sync,
  PgPool: FromRef<S>,
{
  type Rejection = (StatusCode, String);

  async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let pg_pool = PgPool::from_ref(state);
    let conn = pg_pool.get().await.map_err(internal_error)?;

    Ok(Self(conn))
  }
}

#[async_trait]
impl<S> FromRequestParts<S> for Redis
where
  S: Send + Sync,
  RedisPool: FromRef<S>,
{
  type Rejection = (StatusCode, String);

  async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let redis_pool = RedisPool::from_ref(state);

    let conn = redis_pool.get().await.map_err(internal_error)?;

    Ok(Self(conn))
  }
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
  E: std::error::Error,
{
  (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
