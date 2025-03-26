use axum::{
    extract::State,
    Json,
    routing::get,
    Router
};
use serde::Serialize;
use serde_json::json;
use sqlx::{MySqlPool, Row};

// init router
pub fn init_router() -> Router<MySqlPool> {
    let app = Router::new()
        .route("/employees", get(employees));
    Router::new()
        .nest("/api", app)
}

// employee entity
#[derive(Serialize)]
pub struct EmployeeEntity {
    emp_no: i32,
    birth_date: String,
    first_name: String,
    last_name: String,
    gender: String,
    hire_date: String
}

// employees
pub async fn employees(
    State(pool): State<MySqlPool>
) -> Json<serde_json::Value> {
    let result = sqlx::query("SELECT emp_no, birth_date, first_name, last_name, gender, hire_date FROM employee limit 20")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(result) => {
            let mut data: Vec<EmployeeEntity> = vec![];

            for row in result {
                let birth_date: chrono::NaiveDate = row.get(1);
                let birth_date = birth_date.format("%Y-%m-%d").to_string();
                let hire_date: chrono::NaiveDate = row.get(5);
                let hire_date = hire_date.format("%Y-%m-%d").to_string();

                let entity = EmployeeEntity {
                    emp_no: row.get(0),
                    birth_date,
                    first_name: row.get(2),
                    last_name: row.get(3),
                    gender: row.get(4),
                    hire_date
                };

                data.push(entity);
            }

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
