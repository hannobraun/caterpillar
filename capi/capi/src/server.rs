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
use capi_runtime::{DebugEvent, Functions};
use tokio::{
    net::TcpListener,
    runtime::Runtime,
    sync::{mpsc, Mutex},
};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub fn start(functions: Functions) {
    thread::spawn(|| {
        let res = catch_unwind(|| {
            if let Err(err) = serve(functions) {
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
    let (events_tx, mut events_rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(event) = events_rx.recv().await {
            dbg!(event);
        }
    });

    let app = Router::new()
        .route("/", get(handler))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
        )
        .with_state((Arc::new(Mutex::new(functions)), events_tx));
    let listener = TcpListener::bind("127.0.0.1:34481").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler(
    socket: WebSocketUpgrade,
    State((functions, events)): State<(Arc<Mutex<Functions>>, EventsTx)>,
) -> impl IntoResponse {
    socket.on_upgrade(|socket| handle_socket(socket, functions, events))
}

async fn handle_socket(
    mut socket: WebSocket,
    functions: Arc<Mutex<Functions>>,
    events: EventsTx,
) {
    send(&functions, &mut socket).await;

    while let Some(message) = socket.recv().await {
        let message = message.unwrap();

        let event: DebugEvent = match message {
            Message::Text(event) => serde_json::from_str(&event).unwrap(),
            Message::Binary(event) => serde_json::from_slice(&event).unwrap(),
            _ => continue,
        };

        events.send(event.clone()).unwrap();

        functions.lock().await.apply_debug_event(event);
        send(&functions, &mut socket).await;
    }
}

async fn send(functions: &Arc<Mutex<Functions>>, socket: &mut WebSocket) {
    let message =
        serde_json::to_string(&functions.lock().await.deref()).unwrap();
    socket.send(Message::Text(message)).await.unwrap();
}

pub type EventsTx = mpsc::UnboundedSender<DebugEvent>;
