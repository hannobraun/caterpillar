use std::time::Instant;

use capi_game_engine::{game_engine::GameEngine, tiles::NUM_PIXEL_BYTES};
use capi_watch::build_game_once;
use rand::random;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let code = build_game_once("snake").await?;

    let mut pixels = [0; NUM_PIXEL_BYTES];
    let mut game_engine = GameEngine::new();

    game_engine.on_new_bytecode(code.bytecode);

    let mut now = Instant::now();

    let mut total_frame_times_ms = 0;
    let mut num_frame_times = 0;

    while !game_engine.process.state().has_finished() {
        while game_engine.push_random(random()) {}
        game_engine.run_until_end_of_frame(&mut pixels);

        if let Some(effect) =
            game_engine.process.state().first_unhandled_effect()
        {
            println!("Unhandled effect: {effect:#?}");
            break;
        }

        let frame_time = now.elapsed();

        total_frame_times_ms += frame_time.as_millis();
        num_frame_times += 1;

        if total_frame_times_ms >= 1000 {
            let average = total_frame_times_ms / num_frame_times;

            println!("Average: {average} ms",);

            total_frame_times_ms = 0;
            num_frame_times = 0;
        }

        // Do this after the whole frame time bookkeeping, so it's not
        // influencing the performance measurement.
        now = Instant::now();
    }

    Ok(())
}
