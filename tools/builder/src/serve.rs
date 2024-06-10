use std::process;

use axum::{extract::State, http::StatusCode, routing::get, Router};
use tokio::{net::TcpListener, sync::watch, task};
use tower_http::services::ServeDir;
use tracing::error;

pub async fn start(updates: watch::Receiver<()>) -> anyhow::Result<()> {
    let address = "localhost:34480";

    let router = Router::new()
        .route("/updates", get(serve_updates))
        .nest_service("/", ServeDir::new("capi/dist"))
        .with_state(updates);
    let listener = TcpListener::bind(address).await?;

    task::spawn(async {
        if let Err(err) = axum::serve(listener, router).await {
            error!("Error serving HTTP endpoints: {err}");
            process::exit(1);
        }
    });

    println!("Serving Caterpillar at http://{address}");

    Ok(())
}

async fn serve_updates(
    State(mut updates): State<watch::Receiver<()>>,
) -> StatusCode {
    updates.mark_unchanged();
    match updates.changed().await {
        Ok(()) => StatusCode::OK,
        Err(_) => {
            error!("Waiting for updates, but updates no longer available.");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
