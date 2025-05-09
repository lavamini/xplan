use axum::{
    extract::{Query, State},
    http::Uri,
    Json,
    routing::get,
    Router
};
use serde::Serialize;
use serde_json::json;
use sqlx::MySqlPool;

use super::{parse_pagination, Pagination};

// init router
pub fn init_router() -> Router<MySqlPool> {
    let app = Router::new()
        .route("/employees", get(employees));
    Router::new()
        .nest("/api", app)
}

// employee entity
#[derive(Serialize, sqlx::FromRow)]
struct Employee {
    emp_no: i32,
    birth_date: chrono::NaiveDate,
    first_name: String,
    last_name: String,
    gender: String,
    hire_date: chrono::NaiveDate
}

// employees
async fn employees(
    State(pool): State<MySqlPool>,
    uri: Uri
) -> Json<serde_json::Value> {
    let params: Result<Query<Pagination>, _> = Query::try_from_uri(&uri);
    if params.is_err() {
        return Json(json!({
            "code": 1,
            "msg": "invalid parameters"
        }))
    }

    // parse pagination
    let (_, page_size, offset) = parse_pagination(params.unwrap().0);

    let result = sqlx::query_as::<_, Employee>("SELECT t1.emp_no, birth_date, first_name, last_name, gender, hire_date FROM employee t1 INNER JOIN (SELECT emp_no FROM employee ORDER BY emp_no LIMIT ?,?) t2 ON t1.emp_no = t2.emp_no")
        .bind(offset)
        .bind(page_size)
        .fetch_all(&pool)
        .await;

    match result {
        Ok(data) => {
            return Json(json!({
                "code": 0,
                "data": data,
                "msg": "success"
            }))
        },
        Err(err) => {
            tracing::error!("select employees error: {}", err.to_string());
            return Json(json!({
                "code": 1,
                "msg": "select employees failed"
            }))
        }
    }
}
