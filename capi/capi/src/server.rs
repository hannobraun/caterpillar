use std::{panic::catch_unwind, process::exit, thread};

use axum::{extract::State, routing::get, Router};
use tokio::{net::TcpListener, runtime::Runtime};

use crate::capi::Functions;

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
    let app = Router::new().route("/", get(handler)).with_state(functions);
    let listener = TcpListener::bind("localhost:34481").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler(State(functions): State<Functions>) -> String {
    println!("{functions}");
    "Hello, world!".to_string()
}
