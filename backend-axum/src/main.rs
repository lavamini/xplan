use structopt::StructOpt;
use axum::{routing::get, Router};

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "p", long = "port", default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    let port = &args.port;

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    println!("⇨ axum server listening on \x1b[32m{}\x1b[0m", port);
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, axum server"
}
