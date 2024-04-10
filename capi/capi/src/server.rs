use std::{panic::catch_unwind, process::exit, thread};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use capi_runtime::DebugState;
use tokio::{net::TcpListener, runtime::Runtime};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub fn start(debug_state: DebugState) {
    thread::spawn(|| {
        let res = catch_unwind(|| {
            if let Err(err) = serve(debug_state) {
                eprintln!("Server error: {err}");
                exit(1);
            }
        });

        if res.is_err() {
            exit(2);
        }
    });
}

fn serve(debug_state: DebugState) -> anyhow::Result<()> {
    let runtime = Runtime::new()?;
    runtime.block_on(serve_async(debug_state))?;
    Ok(())
}

async fn serve_async(debug_state: DebugState) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(handler))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
        )
        .with_state(debug_state);
    let listener = TcpListener::bind("127.0.0.1:34481").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler(
    socket: WebSocketUpgrade,
    State(debug_state): State<DebugState>,
) -> impl IntoResponse {
    let debug_state = serde_json::to_string(&debug_state).unwrap();
    socket.on_upgrade(|socket| handle_socket(socket, debug_state))
}

async fn handle_socket(mut socket: WebSocket, functions: String) {
    socket.send(Message::Text(functions)).await.unwrap();
}
