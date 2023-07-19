use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::{async_trait, extract::FromRef};
use deadpool_redis::{Config, Runtime};
use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct AppState {
  pg_conn: DatabaseConnection,
  redis_pool: RedisPool,
}

impl AppState {
  pub async fn new() -> anyhow::Result<Self> {
    let db_url = std::env::var("DATABASE_URL").expect("db_url must be set");
    let pg_conn: DatabaseConnection = Database::connect(db_url).await?;
    let cfg = Config::from_url("redis://127.0.0.1/");
    let redis_pool = cfg.create_pool(Some(Runtime::Tokio1))?;

    Ok(Self {
      pg_conn,
      redis_pool,
    })
  }
}

impl FromRef<AppState> for DatabaseConnection {
  fn from_ref(app_state: &AppState) -> DatabaseConnection {
    app_state.pg_conn.clone()
  }
}

impl FromRef<AppState> for RedisPool {
  fn from_ref(app_state: &AppState) -> RedisPool {
    app_state.redis_pool.clone()
  }
}

pub type RedisConnection = deadpool_redis::Connection;

pub struct Redis(pub RedisConnection);
pub struct Postgres(pub DatabaseConnection);

pub type RedisPool = deadpool_redis::Pool;

#[async_trait]
impl<S> FromRequestParts<S> for Postgres
where
  S: Send + Sync,
  DatabaseConnection: FromRef<S>,
{
  type Rejection = (StatusCode, String);

  async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let conn = DatabaseConnection::from_ref(state);
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
