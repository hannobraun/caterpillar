use std::{future, io, path::PathBuf, str};

use axum::{
    extract::{ws::WebSocket, Path, State, WebSocketUpgrade},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use capi_compiler::compiler::compile;
use capi_process::Bytecode;
use capi_protocol::update::SourceCode;
use clap::Parser;
use tokio::{fs::File, io::AsyncReadExt, net::TcpListener};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = Args::parse();

    let script = snake::main();
    let script = str::from_utf8(&script).unwrap();
    let script = ron::from_str(script).unwrap();

    let (bytecode, source_map) = compile(&script);
    let source_code = SourceCode {
        functions: script.functions,
        source_map,
    };

    start_server(args.address, args.serve_dir, source_code, bytecode).await?;

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
    source_code: SourceCode,
    bytecode: Bytecode,
) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/is-alive", get(serve_is_alive))
        .route("/wait-while-alive", get(serve_wait_while_alive))
        .route("/source-code", get(serve_source_code))
        .route("/bytecode", get(serve_bytecode))
        .route("/", get(serve_index))
        .route("/*path", get(serve_static))
        .with_state(ServerState {
            serve_dir,
            source_code,
            bytecode,
        });

    let listener = TcpListener::bind(address).await?;
    println!("builder: ready"); // signal the builder we're ready
    axum::serve(listener, router).await?;

    Ok(())
}

#[derive(Clone)]
pub struct ServerState {
    serve_dir: PathBuf,
    source_code: SourceCode,
    bytecode: Bytecode,
}

async fn serve_is_alive() -> StatusCode {
    StatusCode::OK
}

async fn serve_wait_while_alive(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(do_nothing_while_server_is_alive)
}

async fn do_nothing_while_server_is_alive(_: WebSocket) {
    future::pending::<()>().await;
}

async fn serve_source_code(
    State(state): State<ServerState>,
) -> impl IntoResponse {
    ron::to_string(&state.source_code)
        .unwrap()
        .as_bytes()
        .to_vec()
}

async fn serve_bytecode(State(state): State<ServerState>) -> impl IntoResponse {
    ron::to_string(&state.bytecode).unwrap().as_bytes().to_vec()
}

async fn serve_index(State(state): State<ServerState>) -> impl IntoResponse {
    make_file_response(state.serve_dir.join("index.html")).await
}

async fn serve_static(
    Path(path): Path<PathBuf>,
    State(state): State<ServerState>,
) -> impl IntoResponse {
    make_file_response(state.serve_dir.join(path)).await
}

async fn make_file_response(path: PathBuf) -> Response {
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
