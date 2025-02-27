use sqlx::{Pool, mysql::{MySql, MySqlPoolOptions}};

// conn_str: mysql://user:password@host:port/database
pub async fn init_db_pool(conn_str: &str) -> Pool<MySql> {
    // 获取 cpu 核心数（不包括超线程）
	let cpus = num_cpus::get_physical() as u32;

    let pool = MySqlPoolOptions::new()
        .max_connections(cpus * 2 + 1)
        .connect(&conn_str)
        .await
        .expect("can't connect to database");

    pool
}