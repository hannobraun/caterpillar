use axum::{extract::State, http::StatusCode, routing::get, Router};
use tokio::{net::TcpListener, task};

use crate::watch::DebouncedChanges;

pub async fn start(changes: DebouncedChanges) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/changes", get(serve_changes))
        .with_state(changes);
    let listener = TcpListener::bind("localhost:34480").await?;

    task::spawn(async { axum::serve(listener, router).await });

    Ok(())
}

async fn serve_changes(
    State(mut changes): State<DebouncedChanges>,
) -> StatusCode {
    changes.wait_for_change().await;
    StatusCode::OK
}
