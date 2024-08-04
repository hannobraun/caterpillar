use capi_game_engine::{game_engine::GameEngine, tiles::NUM_PIXEL_BYTES};
use capi_watch::build_game_once;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let code = build_game_once("snake").await?;

    let mut pixels = [0; NUM_PIXEL_BYTES];
    let mut game_engine = GameEngine::new();

    game_engine.on_new_bytecode(code.bytecode);

    while !game_engine.process.state().has_finished() {
        game_engine.run_until_end_of_frame(&mut pixels)
    }

    Ok(())
}
