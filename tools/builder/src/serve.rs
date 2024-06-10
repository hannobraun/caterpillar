use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub async fn serve() -> anyhow::Result<()> {
    let router = Router::new().route("/changes", get(changes));
    let listener = TcpListener::bind("localhost:34480").await?;

    axum::serve(listener, router).await?;

    Ok(())
}

async fn changes() -> &'static str {
    "Hello, world!"
}
