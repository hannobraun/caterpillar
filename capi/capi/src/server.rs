use std::{panic::catch_unwind, process::exit, thread};

use axum::{routing::get, Router};
use tokio::{net::TcpListener, runtime::Runtime};

use crate::capi::Functions;

pub fn start(functions: Functions) {
    functions.print();

    thread::spawn(|| {
        let res = catch_unwind(|| {
            if let Err(err) = serve() {
                eprintln!("Server error: {err}");
                exit(1);
            }
        });

        if res.is_err() {
            exit(2);
        }
    });
}

fn serve() -> anyhow::Result<()> {
    let runtime = Runtime::new()?;
    runtime.block_on(serve_async())?;
    Ok(())
}

async fn serve_async() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(handler));
    let listener = TcpListener::bind("localhost:34481").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler() -> &'static str {
    "Hello, world!"
}
