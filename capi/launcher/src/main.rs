#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let code = capi_watch::build_game_once("snake").await?;

    let mut pixels = [0; capi_game_engine::tiles::NUM_PIXEL_BYTES];
    let mut game_engine = capi_game_engine::game_engine::GameEngine::new();

    game_engine.on_new_bytecode(code.bytecode);

    while !game_engine.process.state().has_finished() {
        game_engine.run_until_end_of_frame(&mut pixels)
    }

    Ok(())
}
