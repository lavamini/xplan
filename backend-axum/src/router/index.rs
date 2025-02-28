use axum::{routing::get, Router};
use sqlx::MySqlPool;

// init router
pub fn init_router() -> Router<MySqlPool> {
    Router::new()
        .route("/", get(index))
}

// index
pub async fn index() -> &'static str {
    "Hello, axum server"
}
