use std::{io, path::PathBuf, process};

use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tokio::{
    fs::File,
    io::AsyncReadExt,
    net::TcpListener,
    task::{self, JoinHandle},
};
use tracing::error;

use crate::build::UpdatesRx;

pub async fn start(mut updates: UpdatesRx) -> anyhow::Result<()> {
    let address = "localhost:34480";

    start_server(address, updates.clone()).await?;

    while let Ok(()) = updates.changed().await {
        println!();
        println!("Caterpillar is ready:");
        println!();
        println!("\t🚀 http://{address}/");
        println!();
    }

    Ok(())
}

async fn start_server(
    address: &'static str,
    updates: UpdatesRx,
) -> anyhow::Result<JoinHandle<()>> {
    let router = Router::new()
        .route("/updates", get(serve_updates))
        .route("/", get(serve_index))
        .route("/*path", get(serve_static))
        .with_state(updates);
    let listener = TcpListener::bind(address).await?;

    let handle = task::spawn(async {
        if let Err(err) = axum::serve(listener, router).await {
            error!("Error serving HTTP endpoints: {err}");
            process::exit(1);
        }
    });

    Ok(handle)
}

async fn serve_updates(State(mut updates): State<UpdatesRx>) -> StatusCode {
    updates.mark_unchanged();
    match updates.changed().await {
        Ok(()) => StatusCode::OK,
        Err(_) => {
            error!("Waiting for updates, but updates no longer available.");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn serve_index(State(updates): State<UpdatesRx>) -> impl IntoResponse {
    file_response(PathBuf::from("index.html"), updates).await
}

async fn serve_static(
    Path(path): Path<PathBuf>,
    State(updates): State<UpdatesRx>,
) -> impl IntoResponse {
    file_response(path, updates).await
}

async fn file_response(path: PathBuf, updates: UpdatesRx) -> Response {
    let Some(serve_dir) = updates.borrow().clone() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let path = serve_dir.join(path);

    let content_type = match path.extension() {
        Some(os_str) => match os_str.to_str() {
            Some("html") => "text/html",
            Some("js") => "application/javascript",
            Some("wasm") => "application/wasm",
            _ => "application/octet-stream",
        },
        _ => "application/octet-stream",
    };

    let mut file = match File::open(path).await {
        Ok(file) => file,
        Err(err) => return error_to_response(err),
    };

    let mut data = Vec::new();
    if let Err(err) = file.read_to_end(&mut data).await {
        return error_to_response(err);
    }

    (StatusCode::OK, [(header::CONTENT_TYPE, content_type)], data)
        .into_response()
}

fn error_to_response(err: io::Error) -> Response {
    let status = match err.kind() {
        io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    status.into_response()
}
