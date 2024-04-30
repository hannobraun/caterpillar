use std::{
    fmt,
    panic::{catch_unwind, AssertUnwindSafe},
    process::exit,
    thread,
};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use capi_runtime::DebugEvent;
use futures::{SinkExt, StreamExt};
use tokio::{net::TcpListener, runtime::Runtime, sync::mpsc};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::updates::UpdatesRx;

pub fn start(updates: UpdatesRx, events: EventsTx) {
    thread::spawn(|| {
        // Unwind safety doesn't matter, because we access no data from within
        // the panicking context after the panic. We just exit the process.
        //
        // The only thing we need it for in the first place, is the `event`
        // channel.
        let res = catch_unwind(AssertUnwindSafe(|| {
            if let Err(err) = serve(updates, events) {
                eprintln!("Server error: {err}");
                exit(1);
            }
        }));

        if res.is_err() {
            exit(2);
        }
    });
}

fn serve(updates: UpdatesRx, events: EventsTx) -> anyhow::Result<()> {
    let runtime = Runtime::new()?;
    runtime.block_on(serve_async(updates, events))?;
    Ok(())
}

async fn serve_async(
    updates: UpdatesRx,
    events: EventsTx,
) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(handler))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
        )
        .with_state((updates, events));
    let listener = TcpListener::bind("127.0.0.1:34481").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler(
    socket: WebSocketUpgrade,
    State((updates, events)): State<(UpdatesRx, EventsTx)>,
) -> impl IntoResponse {
    socket.on_upgrade(|socket| handle_socket(socket, updates, events))
}

async fn handle_socket(
    socket: WebSocket,
    updates: UpdatesRx,
    events: EventsTx,
) {
    let (socket_tx, socket_rx) = socket.split();

    tokio::spawn(handle_updates(updates, socket_tx));
    handle_events(events, socket_rx).await;
}

async fn handle_updates<S>(mut updates: UpdatesRx, mut socket: S)
where
    S: SinkExt<Message> + Unpin,
    S::Error: fmt::Debug,
{
    // The initial value is considered to be "seen". Mark the receiver as
    // changed, so we send an initial update to the client immediately.
    updates.mark_changed();

    loop {
        let program = match updates.changed().await {
            Ok(()) => updates.borrow_and_update().clone(),
            Err(err) => panic!("{err}"),
        };

        let message = ron::to_string(&program).unwrap();
        socket.send(Message::Text(message)).await.unwrap();
    }
}

async fn handle_events<S>(events: EventsTx, mut socket_rx: S)
where
    S: StreamExt<Item = Result<Message, axum::Error>> + Unpin,
{
    while let Some(message) = socket_rx.next().await {
        let message = message.unwrap();

        let event: DebugEvent = match message {
            Message::Text(text) => ron::from_str(&text).unwrap(),
            Message::Binary(binary) => {
                panic!("Unexpectedly received binary message: {binary:?}");
            }
            _ => continue,
        };

        events.send(event).unwrap();
    }
}

pub type EventsRx = mpsc::UnboundedReceiver<DebugEvent>;
pub type EventsTx = mpsc::UnboundedSender<DebugEvent>;
