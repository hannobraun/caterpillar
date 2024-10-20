use std::{future, io, path::PathBuf};

use axum::{
    extract::{ws::WebSocket, Path, State, WebSocketUpgrade},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use capi_build_game::CodeRx;
use capi_protocol::Versioned;
use tokio::{fs::File, io::AsyncReadExt, net::TcpListener};

pub async fn start(
    address: String,
    serve_dir: PathBuf,
    code: CodeRx,
) -> anyhow::Result<()> {
    let router = Router::new()
        .route("/is-alive", get(serve_is_alive))
        .route("/wait-while-alive", get(serve_wait_while_alive))
        .route("/code", get(serve_code))
        .route("/code/:timestamp", get(serve_code))
        .route("/", get(serve_index))
        .route("/*path", get(serve_static))
        .with_state(ServerState { serve_dir, code });

    let listener = TcpListener::bind(address).await?;
    println!("ready"); // signal the builder we're ready
    axum::serve(listener, router).await?;

    Ok(())
}

#[derive(Clone, Debug)]
pub struct ServerState {
    serve_dir: PathBuf,
    code: CodeRx,
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

async fn serve_code(
    State(mut state): State<ServerState>,
    timestamp: Option<Path<u64>>,
) -> impl IntoResponse {
    loop {
        if let Some(timestamp) = &timestamp {
            if timestamp.0 >= state.code.borrow().timestamp {
                if state.code.changed().await.is_err() {
                    // Sender has been dropped.
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
                continue;
            }
        }

        let code = &*state.code.borrow();
        let code = Versioned {
            timestamp: code.timestamp,
            inner: &code.inner,
        };

        return ron::to_string(&code)
            .unwrap()
            .as_bytes()
            .to_vec()
            .into_response();
    }
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
