#[tokio::main]
async fn main() -> anyhow::Result<()> {
    warp::serve(warp::fs::dir("."))
        .run(([127, 0, 0, 1], 8080))
        .await;

    Ok(())
}
