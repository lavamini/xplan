use axum::Router;
use sqlx::MySqlPool;

mod index;
mod user;
mod employee;

pub fn init_router() -> Router<MySqlPool> {
    Router::new()
        .merge(index::init_router())
        .merge(user::init_router())
        .merge(employee::init_router())
}
