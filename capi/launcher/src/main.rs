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

    let start_of_game = Instant::now();
    let mut start_of_frame = Instant::now();

    let mut times_gross = Measurements {
        total_ms: 0,
        min_ms: None,
        max_ms: None,
        num: 0,
    };

    while !game_engine.process.has_finished() {
        while game_engine.push_random(random()) {}

        if !game_engine.run_until_end_of_frame(
            start_of_game.elapsed().as_secs_f64(),
            &mut pixels,
        ) {
            // Game engine decided that it's not time to run another frame yet.
            continue;
        }

        if let Some(effect) = game_engine.process.inspect_effect() {
            eprintln!("Unhandled effect: {effect:#?}");
            eprintln!("Current stack:\n{:#?}", game_engine.process.stack());
            break;
        }

        let frame_time_gross = start_of_frame.elapsed().as_millis();
        start_of_frame = Instant::now();

        times_gross.total_ms += frame_time_gross;
        times_gross.num += 1;

        if let Some(min) = times_gross.min_ms {
            if frame_time_gross < min {
                times_gross.min_ms = Some(frame_time_gross);
            }
        } else {
            times_gross.min_ms = Some(frame_time_gross);
        }
        if let Some(max) = times_gross.max_ms {
            if frame_time_gross > max {
                times_gross.max_ms = Some(frame_time_gross);
            }
        } else {
            times_gross.max_ms = Some(frame_time_gross);
        }

        if times_gross.total_ms >= 1000 {
            let avg = times_gross.total_ms / times_gross.num;
            let max = times_gross.max_ms.unwrap();
            let min = times_gross.min_ms.unwrap();

            eprintln!("avg: {avg} ms; max: {max} ms; min: {min} ms",);

            times_gross = Measurements {
                total_ms: 0,
                min_ms: None,
                max_ms: None,
                num: 0,
            }
        }
    }

    Ok(())
}

pub struct Measurements {
    total_ms: u128,
    min_ms: Option<u128>,
    max_ms: Option<u128>,
    num: u128,
}
