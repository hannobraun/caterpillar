use std::future::IntoFuture;

use axum::{routing::get, Router};
use tokio::{net::TcpListener, task};

use crate::watch::DebouncedChanges;

pub async fn serve(changes: DebouncedChanges) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/changes", get(serve_changes))
        .with_state(changes);
    let listener = TcpListener::bind("localhost:34480").await?;

    task::spawn(axum::serve(listener, router).into_future());

    Ok(())
}

async fn serve_changes() -> &'static str {
    "Hello, world!"
}
