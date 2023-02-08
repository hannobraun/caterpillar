use std::io;

use crate::{
    cells::{self, Generation},
    cp::Functions,
    ui,
};

pub enum Event {
    Tick,
}

pub struct State {
    pub functions: Functions,
    pub generations: Vec<Generation>,
    pub buffer: ui::Buffer,
    pub stdout: io::Stdout,
}

pub fn run_once(event: Event, state: &mut State) -> anyhow::Result<()> {
    let Event::Tick = event;

    let current = state
        .generations
        .last()
        .cloned()
        .unwrap_or_else(cells::init);

    // We only add new generations, but never delete them. This is fine for now,
    // I think. Let's just hope nobody runs this for long enough to fill up
    // their main memory.
    let next = cells::next_generation(current, &state.functions);
    state.generations.push(next);

    ui::draw(
        &state.generations,
        &state.functions,
        &mut state.buffer,
        &mut state.stdout,
    )?;

    Ok(())
}
