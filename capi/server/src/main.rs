use std::{future, io, path::PathBuf};

use axum::{
    extract::{ws::WebSocket, Path, State, WebSocketUpgrade},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use clap::Parser;
use tokio::{fs::File, io::AsyncReadExt, net::TcpListener};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = Args::parse();
    start_server(args.address, args.serve_dir).await?;

    info!("`capi-server` shutting down.");
    Ok(())
}

/// Caterpillar server
#[derive(clap::Parser)]
pub struct Args {
    /// Address to serve at
    #[arg(short, long)]
    pub address: String,

    /// Directory to serve from
    #[arg(short, long)]
    pub serve_dir: PathBuf,
}

async fn start_server(
    address: String,
    serve_dir: PathBuf,
) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/is-alive", get(serve_is_alive))
        .route("/updates", get(serve_updates))
        .route("/", get(serve_index))
        .route("/*path", get(serve_static))
        .with_state(serve_dir);
    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, router).await?;

    Ok(())
}

async fn serve_is_alive() -> StatusCode {
    StatusCode::OK
}

async fn serve_updates(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(serve_updates_handle_socket)
}

async fn serve_updates_handle_socket(_: WebSocket) {
    future::pending::<()>().await;
}

async fn serve_index(State(serve_dir): State<PathBuf>) -> impl IntoResponse {
    file_response(PathBuf::from("index.html"), serve_dir).await
}

async fn serve_static(
    Path(path): Path<PathBuf>,
    State(serve_dir): State<PathBuf>,
) -> impl IntoResponse {
    file_response(path, serve_dir).await
}

async fn file_response(path: PathBuf, serve_dir: PathBuf) -> Response {
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
