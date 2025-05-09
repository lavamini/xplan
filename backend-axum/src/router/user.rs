use axum::{
    extract::{Query, State},
    http::Uri,
    Json,
    routing::{get, post},
    Router
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{MySqlPool, Row};

use super::{parse_pagination, Pagination};

// init router
pub fn init_router() -> Router<MySqlPool> {
    let app = Router::new()
        .route("/signin", post(signin))
        .route("/signup", post(signup))
        .route("/users", get(users));
    Router::new()
        .nest("/api", app)
}

// user form
#[derive(Deserialize)]
struct UserForm {
    #[serde(default)]
    username: String,
    #[serde(default)]
    password: String
}

// signin
async fn signin(
    State(pool): State<MySqlPool>,
    Json(user_form): Json<UserForm>
) -> Json<serde_json::Value> {
    let username = user_form.username.trim();
    let password = user_form.password.trim();

    if username == "" || password == "" {
        return Json(json!({
            "code": 1,
            "msg": "parameters missing"
        }))
    }

    let db_result = sqlx::query("SELECT password_hash FROM user WHERE username = ?")
        .bind(username)
        .fetch_one(&pool)
        .await;

    let password_hash = match db_result {
        Ok(db_row) => {
            db_row.try_get::<String, _>("password_hash").unwrap()
        },
        Err(sqlx::Error::RowNotFound) => {
            return Json(json!({
                "code": 1,
                "msg": "username or password not correct"
            }))
        },
        Err(err) => {
            tracing::error!("select user error: {}", err.to_string());
            return Json(json!({
                "code": 1,
                "msg": "signin failed"
            }))
        }
    };

    let valid = bcrypt::verify(password, &password_hash).unwrap();
    if valid {
        return Json(json!({
            "code": 0,
            "msg": "signin success"
        }))
    } else {
        return Json(json!({
            "code": 1,
            "msg": "username or password not correct"
        }))
    }
}

// signup
async fn signup(
    State(pool): State<MySqlPool>,
    Json(user_form): Json<UserForm>
) -> Json<serde_json::Value> {
    let username = user_form.username.trim();
    let password = user_form.password.trim();

    if username == "" || password == "" {
        return Json(json!({
            "code": 1,
            "msg": "parameters missing"
        }))
    }

    let db_result = sqlx::query("SELECT id FROM user WHERE username = ?")
        .bind(username)
        .fetch_one(&pool)
        .await;

    match db_result {
        Ok(_) => {
            return Json(json!({
                "code": 1,
                "msg": "username already exist"
            }))
        },
        Err(sqlx::Error::RowNotFound) => {
            // just skip
        },
        Err(err) => {
            tracing::error!("select user error: {}", err.to_string());
            return Json(json!({
                "code": 1,
                "msg": "signup failed"
            }))
        }
    };

    let password_hash = bcrypt::hash(password, 10).unwrap();

    let db_result = sqlx::query("INSERT INTO user(username, password_hash) VALUES(?, ?)")
        .bind(username)
        .bind(password_hash)
        .execute(&pool)
        .await;

    match db_result {
        Ok(_) => {
            return Json(json!({
                "code": 0,
                "msg": "signup success"
            }))
        },
        Err(err) => {
            tracing::error!("insert user error: {}", err.to_string());
            return Json(json!({
                "code": 1,
                "msg": "signup failed"
            }))
        }
    }
}

// user entity
#[derive(Serialize, sqlx::FromRow)]
struct User {
    id: u64,
    username: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

// users
async fn users(
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

    let sql_str;
    if offset < 10000 {
        sql_str = "SELECT id, username, created_at, updated_at FROM user LIMIT ?,?";
    } else {
        sql_str = "SELECT t1.id, username, created_at, updated_at FROM user t1, (SELECT id FROM user LIMIT ?,?) t2 WHERE t1.id = t2.id";
    }

    let db_result = sqlx::query_as::<_, User>(sql_str)
        .bind(offset)
        .bind(page_size)
        .fetch_all(&pool)
        .await;

    match db_result {
        Ok(users) => {
            return Json(json!({
                "code": 0,
                "msg": "success",
                "data": users
            }))
        },
        Err(err) => {
            tracing::error!("select users error: {}", err.to_string());
            return Json(json!({
                "code": 1,
                "msg": "select users failed"
            }))
        }
    }
}
