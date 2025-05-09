use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

// conn_str: mysql://user:password@host:port/database
pub async fn init_db_pool(conn_str: &str, min_conns: u32, max_conns: u32) -> MySqlPool {
    tracing::debug!("connecting to database ...");

    let pool = MySqlPoolOptions::new()
        .min_connections(min_conns)
        .max_connections(max_conns)
        // 禁用 test_before_acquire
        .test_before_acquire(false)
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
