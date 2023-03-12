use std::panic::{self, AssertUnwindSafe};

use crossterm::terminal;
use futures::FutureExt;

pub async fn run() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    let result = AssertUnwindSafe(run_inner()).catch_unwind().await;
    terminal::disable_raw_mode()?;

    match result {
        Ok(result) => result,
        Err(err) => panic::resume_unwind(err),
    }
}

async fn run_inner() -> anyhow::Result<()> {
    todo!()
}
