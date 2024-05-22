use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::get,
    Router,
};
use clap::Parser;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;

use std::time::Duration;

#[derive(Parser, Debug)]
pub struct Server {
    #[arg(long, default_value = "8080")]
    pub port: u16,
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
    #[arg(long)]
    pub workers: Option<usize>,
    #[arg(long)]
    pub threads: Option<usize>,
    #[arg(long)]
    pub keep_alive: Option<usize>,
    #[arg(long)]
    pub keep_alive_timeout: Option<usize>,
}

impl Server {
    pub async fn run(&self) -> anyhow::Result<()> {
        let db_connection_str = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

        // set up connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_connection_str)
            .await
            .expect("can't connect to database");

        // build our application with some routes
        let app = Router::new()
            .route(
                "/",
                get(using_connection_pool_extractor).post(using_connection_extractor),
            )
            .with_state(pool);

        let addr = format!("{}:{}", self.host, self.port);

        // run it with hyper
        let listener = TcpListener::bind(addr).await?;
        tracing::debug!("listening on {}", listener.local_addr()?);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

// we can extract the connection pool with `State`
async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

async fn using_connection_extractor(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
