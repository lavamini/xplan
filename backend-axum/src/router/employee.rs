use axum::{
    extract::State,
    Json,
    routing::get,
    Router
};
use serde::Serialize;
use serde_json::json;
use sqlx::{MySql, MySqlPool};

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
    State(pool): State<MySqlPool>
) -> Json<serde_json::Value> {
    let result = sqlx::query_as::<MySql, Employee>("SELECT emp_no, birth_date, first_name, last_name, gender, hire_date FROM employee LIMIT 20000,20")
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
