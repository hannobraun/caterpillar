use std::{future, io, path::PathBuf, process};

use axum::{
    extract::{ws::WebSocket, Path, State, WebSocketUpgrade},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use clap::Parser;
use tokio::{
    fs::File,
    io::AsyncReadExt,
    net::TcpListener,
    task::{self, JoinHandle},
};
use tracing::error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = Args::parse();
    println!("Address: {}", args.address);
    println!("Directory to serve: {}", args.serve_dir.display());

    let server = start_server(args.address, args.serve_dir).await?;
    server.await?;

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
) -> anyhow::Result<JoinHandle<()>> {
    let router = Router::new()
        .route("/is-alive", get(serve_is_alive))
        .route("/updates", get(serve_updates))
        .route("/", get(serve_index))
        .route("/*path", get(serve_static))
        .with_state(serve_dir);
    let listener = TcpListener::bind(address).await?;

    let handle = task::spawn(async {
        if let Err(err) = axum::serve(listener, router).await {
            error!("Error serving HTTP endpoints: {err}");
            process::exit(1);
        }
    });

    Ok(handle)
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
