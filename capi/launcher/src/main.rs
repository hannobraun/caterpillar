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

    let mut times_gross = Measurements::default();

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

        times_gross.measure(frame_time_gross);

        if times_gross.total_ms >= 1000 {
            let avg_gross = times_gross.total_ms / times_gross.num;
            let max_gross = times_gross.max_ms.unwrap();
            let min = times_gross.min_ms.unwrap();

            eprintln!(
                "avg: {avg_gross} ms; max: {max_gross} ms; min: {min} ms",
            );

            times_gross = Measurements::default();
        }
    }

    Ok(())
}

#[derive(Default)]
pub struct Measurements {
    total_ms: u128,
    min_ms: Option<u128>,
    max_ms: Option<u128>,
    num: u128,
}

impl Measurements {
    fn measure(&mut self, time_ms: u128) {
        self.total_ms += time_ms;
        self.num += 1;

        if let Some(min) = self.min_ms {
            if time_ms < min {
                self.min_ms = Some(time_ms);
            }
        } else {
            self.min_ms = Some(time_ms);
        }
        if let Some(max) = self.max_ms {
            if time_ms > max {
                self.max_ms = Some(time_ms);
            }
        } else {
            self.max_ms = Some(time_ms);
        }
    }
}
