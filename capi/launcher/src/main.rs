#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let code = capi_watch::build_game_once("snake").await?;
    dbg!(code);
    Ok(())
}
