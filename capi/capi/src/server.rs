use std::{ops::Deref, panic::catch_unwind, process::exit, sync::Arc, thread};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use capi_runtime::{DebugEvent, DebugState, Functions};
use tokio::{net::TcpListener, runtime::Runtime, sync::Mutex};
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
        .with_state(Arc::new(Mutex::new(debug_state.functions)));
    let listener = TcpListener::bind("127.0.0.1:34481").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler(
    socket: WebSocketUpgrade,
    State(debug_state): State<Arc<Mutex<Functions>>>,
) -> impl IntoResponse {
    socket.on_upgrade(|socket| handle_socket(socket, debug_state))
}

async fn handle_socket(
    mut socket: WebSocket,
    functions: Arc<Mutex<Functions>>,
) {
    send_debug_state(&functions, &mut socket).await;

    while let Some(message) = socket.recv().await {
        let message = message.unwrap();

        let event: DebugEvent = match message {
            Message::Text(event) => serde_json::from_str(&event).unwrap(),
            Message::Binary(event) => serde_json::from_slice(&event).unwrap(),
            _ => continue,
        };

        functions.lock().await.apply_debug_event(event);
        send_debug_state(&functions, &mut socket).await;
    }
}

async fn send_debug_state(
    functions: &Arc<Mutex<Functions>>,
    socket: &mut WebSocket,
) {
    let message =
        serde_json::to_string(&functions.lock().await.deref()).unwrap();
    socket.send(Message::Text(message)).await.unwrap();
}
