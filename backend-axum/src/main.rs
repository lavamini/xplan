use structopt::StructOpt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod database;
mod router;

use config::load_config;
use database::init_db_pool;
use router::init_router;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "p", long = "port", default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Cli::from_args();
    let port = &args.port;

    // load config
    let config = load_config("config.toml");

    // init db pool
    let db = config.db;
    let db_conn_str = format!("mysql://{}:{}@{}:{}/{}",
        db.user, db.password, db.host, db.port, db.database);
    let pool = init_db_pool(db_conn_str.as_str()).await;

    // init router
    let app = init_router().with_state(pool);

    // run it
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    tracing::info!("⇨ axum server listening on \x1b[32m{}\x1b[0m", port);
    axum::serve(listener, app).await.unwrap();
}
