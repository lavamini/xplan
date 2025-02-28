use axum::{
    extract::State,
    Json,
    routing::post,
    Router
};
use serde_json::json;
use sqlx::{MySqlPool, Row};

// init router
pub fn init_router() -> Router<MySqlPool> {
    let app = Router::new()
        .route("/signin", post(signin));
    Router::new()
        .nest("/api", app)
}

// signin
pub async fn signin(
    State(pool): State<MySqlPool>
) -> Json<serde_json::Value> {
    let result = sqlx::query("select 'hello world from mysql'")
        .fetch_one(&pool)
        .await;

    let result = match result {
        Ok(result) => {
            let txt: &str = result.get(0);
            json!({
                "code": 0,
                "msg": txt
            })
        },
        Err(_) => {
            json!({
                "code": 1,
                "msg": "select user error"
            })
        }
    };

    Json(result)
}