use axum::{
    extract::State,
    http::StatusCode,
    routing::get, Router
};
use sqlx::MySqlPool;
use structopt::StructOpt;

mod config;
mod database;

use config::load_config;
use database::init_db_pool;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "p", long = "port", default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    let port = &args.port;

    // load config
    let config = load_config("config.toml");

    // init db pool
    let db = config.db;
    let db_conn_str = format!("mysql://{}:{}@{}:{}/{}",
        db.user, db.password, db.host, db.port, db.database);
    let pool = init_db_pool(db_conn_str.as_str()).await;

    // build our application with a route
    let app = Router::new().route("/", get(handler)).with_state(pool);

    // run it
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    println!("⇨ axum server listening on \x1b[32m{}\x1b[0m", port);
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    State(pool): State<MySqlPool>
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from mysql'")
    .fetch_one(&pool)
    .await
    .map_err(internal_error)
    //"Hello, axum server"
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
