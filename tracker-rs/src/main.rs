/*!
 * For those that are newer to rust, it may not be obvious but passing around
 * a mutex and locking / unlocking it every request makes for potentially REALLY slow code as the
 * mutex acts like a gate keeper for every request and makes it go from parallel to just concurrent
 * which can be quite bad for perf.
 *
 * Did this for simplicity
 */
mod db;
mod error;
mod templates;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use db::{get_timings, push_timing, clear_timings};
use error::AppError;
use templates::{Index, TimingType};
use tower_http::services::ServeDir;

async fn index() -> Result<Index, AppError> {
    return Ok(Index { timings: get_timings().await });
}

async fn movement() -> Result<Index, AppError> {
    push_timing(TimingType::Movement).await?;
    return Ok(Index { timings: get_timings().await });
}

async fn consume() -> Result<Index, AppError> {
    push_timing(TimingType::Consume).await?;
    return Ok(Index { timings: get_timings().await });
}

async fn clear() -> Result<Index, AppError> {
    clear_timings().await;
    return Ok(Index { timings: get_timings().await });
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/movement", post(movement))
        .route("/consume", post(consume))
        .route("/clear", post(clear))
        .nest_service("/dist", ServeDir::new("assets"));

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
