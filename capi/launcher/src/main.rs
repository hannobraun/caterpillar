use std::time::Instant;

use capi_build_game::build_game_once;
use capi_game_engine::{display::NUM_PIXEL_BYTES, game_engine::GameEngine};
use rand::random;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let code = build_game_once("snake").await?;

    let mut pixels = [0; NUM_PIXEL_BYTES];
    let mut game_engine = GameEngine::new();

    game_engine.on_new_instructions(code.instructions);

    let start = Instant::now();
    let mut now = Instant::now();

    let mut total_frame_times_ms = 0;
    let mut min_frame_time = None;
    let mut max_frame_time = None;
    let mut num_frame_times = 0;

    while !game_engine.process.has_finished() {
        while game_engine.push_random(random()) {}

        if !game_engine
            .run_until_end_of_frame(start.elapsed().as_secs_f64(), &mut pixels)
        {
            // Game engine decided that it's not time to run another frame yet.
            continue;
        }

        if let Some(effect) = game_engine.process.inspect_effect() {
            eprintln!("Unhandled effect: {effect:#?}");
            eprintln!("Current stack:\n{:#?}", game_engine.process.stack());
            break;
        }

        let frame_time = now.elapsed().as_millis();

        total_frame_times_ms += frame_time;
        num_frame_times += 1;

        if let Some(min) = min_frame_time {
            if frame_time < min {
                min_frame_time = Some(frame_time);
            }
        } else {
            min_frame_time = Some(frame_time);
        }
        if let Some(max) = max_frame_time {
            if frame_time > max {
                max_frame_time = Some(frame_time);
            }
        } else {
            max_frame_time = Some(frame_time);
        }

        if total_frame_times_ms >= 1000 {
            let avg = total_frame_times_ms / num_frame_times;
            let max = max_frame_time.unwrap();
            let min = min_frame_time.unwrap();

            eprintln!("avg: {avg} ms; max: {max} ms; min: {min} ms",);

            total_frame_times_ms = 0;
            max_frame_time = None;
            min_frame_time = None;
            num_frame_times = 0;
        }

        // Do this after the whole frame time bookkeeping, so it's not
        // influencing the performance measurement.
        now = Instant::now();
    }

    Ok(())
}
