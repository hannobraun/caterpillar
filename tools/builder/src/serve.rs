use std::{path::PathBuf, process};

use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::{fs::File, io::AsyncReadExt, net::TcpListener, sync::watch, task};
use tracing::error;

pub async fn start(updates: watch::Receiver<()>) -> anyhow::Result<()> {
    let address = "localhost:34480";

    let router = Router::new()
        .route("/updates", get(serve_updates))
        .route("/", get(serve_index))
        .route("/*path", get(serve_static))
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

async fn serve_index() -> impl IntoResponse {
    file_response(PathBuf::from("index.html")).await
}

async fn serve_static(Path(path): Path<PathBuf>) -> impl IntoResponse {
    file_response(path).await
}

async fn file_response(path: PathBuf) -> impl IntoResponse {
    let path = PathBuf::from("capi/dist").join(path);

    let content_type = match path.extension() {
        Some(os_str) => match os_str.to_str() {
            Some("html") => "text/html",
            Some("js") => "application/javascript",
            Some("wasm") => "application/wasm",
            _ => "application/octet-stream",
        },
        _ => "application/octet-stream",
    };

    let mut data = Vec::new();
    File::open(path)
        .await
        .unwrap()
        .read_to_end(&mut data)
        .await
        .unwrap();

    (StatusCode::OK, [(header::CONTENT_TYPE, content_type)], data)
}
