mod watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rx = watch::watch()?;

    while let Some(event) = rx.recv().await {
        let _ = dbg!(event);
    }

    Ok(())
}
