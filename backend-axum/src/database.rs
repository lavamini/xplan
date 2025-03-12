use std::time::Duration;
use sqlx::{Pool, mysql::{MySql, MySqlPoolOptions}};

// conn_str: mysql://user:password@host:port/database
pub async fn init_db_pool(conn_str: &str) -> Pool<MySql> {
    tracing::debug!("connecting to database ...");
    // 获取 cpu 核心数（不包括超线程）
	let cpus = num_cpus::get_physical() as u32;

    let pool = MySqlPoolOptions::new()
        .min_connections(cpus * 2 + 1)
        .max_connections(cpus * 2 + 1)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&conn_str)
        .await;

    let pool = match pool {
        Ok(pool) => pool,
        Err(err) => {
            tracing::error!("can't connect to database: {}", err.to_string());
            std::process::exit(1);
        }
    };
    pool
}
