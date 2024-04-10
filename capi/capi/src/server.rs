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
use capi_runtime::{Function, Functions};
use tokio::{net::TcpListener, runtime::Runtime};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::debug::DebugState;

pub fn start(debug_state: DebugState) {
    thread::spawn(|| {
        let res = catch_unwind(|| {
            if let Err(err) = serve(debug_state.functions) {
                eprintln!("Server error: {err}");
                exit(1);
            }
        });

        if res.is_err() {
            exit(2);
        }
    });
}

fn serve(functions: Functions) -> anyhow::Result<()> {
    let runtime = Runtime::new()?;
    runtime.block_on(serve_async(functions))?;
    Ok(())
}

async fn serve_async(functions: Functions) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(handler))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
        )
        .with_state(functions.inner);
    let listener = TcpListener::bind("127.0.0.1:34481").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler(
    socket: WebSocketUpgrade,
    State(functions): State<Vec<Function>>,
) -> impl IntoResponse {
    let functions = serde_json::to_string(&functions).unwrap();

    socket.on_upgrade(|socket| handle_socket(socket, functions))
}

async fn handle_socket(mut socket: WebSocket, functions: String) {
    socket.send(Message::Text(functions)).await.unwrap();
}
