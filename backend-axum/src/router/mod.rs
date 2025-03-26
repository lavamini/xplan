use axum::Router;
use sqlx::MySqlPool;
use serde::Deserialize;

mod index;
mod user;
mod employee;

pub fn init_router() -> Router<MySqlPool> {
    Router::new()
        .merge(index::init_router())
        .merge(user::init_router())
        .merge(employee::init_router())
}

#[derive(Deserialize)]
struct Pagination {
    #[serde(default)]
    page: i32,
    #[serde(default)]
    page_size: i32
}

const PAGE_SIZE_DEFAULT: i32 = 20;
const PAGE_SIZE_MIN: i32 = 10;
const PAGE_SIZE_MAX: i32 = 100;

fn parse_pagination(params: Pagination) -> (i32, i32, i32) {
    let mut page = params.page;
    if page <= 0 {
        page = 1;
    }

    let mut page_size = params.page_size;
    if page_size <= 0 {
        page_size = PAGE_SIZE_DEFAULT;
    }
    if page_size < PAGE_SIZE_MIN {
        page_size = PAGE_SIZE_MIN;
    }
    if page_size > PAGE_SIZE_MAX {
        page_size = PAGE_SIZE_MAX;
    }

    let offset = (page - 1) * page_size;
    return (page, page_size, offset);
}
