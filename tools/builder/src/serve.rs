use std::process;

use axum::{extract::State, http::StatusCode, routing::get, Router};
use tokio::{net::TcpListener, task};
use tower_http::services::ServeDir;
use tracing::error;

use crate::watch::DebouncedChanges;

pub async fn start(changes: DebouncedChanges) -> anyhow::Result<()> {
    let address = "localhost:34480";

    let router = Router::new()
        .route("/changes", get(serve_changes))
        .nest_service("/", ServeDir::new("capi/dist"))
        .with_state(changes);
    let listener = TcpListener::bind(address).await?;

    task::spawn(async {
        if let Err(err) = axum::serve(listener, router).await {
            error!("Error serving HTTP endpoints: {err}");
            process::exit(1);
        }
    });

    Ok(())
}

async fn serve_changes(
    State(mut changes): State<DebouncedChanges>,
) -> StatusCode {
    changes.wait_for_change().await;
    StatusCode::OK
}
