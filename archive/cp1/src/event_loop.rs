use std::io;

use crate::{
    cells::{self, Generation},
    cp, ui,
};

pub enum Event {
    Key(Key),
    Tick,
}

pub enum Key {
    Backspace,
    Char(char),
}

pub struct State {
    pub interpreter: cp::Interpreter,
    pub generations: Vec<Generation>,
    pub buffer: ui::Buffer,
    pub stdout: io::Stdout,
}

pub fn run_once(event: Event, state: &mut State) -> anyhow::Result<()> {
    match event {
        Event::Key(_) => {
            // Keys are not processed, currently.
        }
        Event::Tick => {
            let current = state
                .generations
                .last()
                .cloned()
                .unwrap_or_else(|| cells::init(&mut state.interpreter));

            // We only add new generations, but never delete them. This is fine
            // for now, I think. Let's just hope nobody runs this for long
            // enough to fill up their main memory.
            let next = cells::next_generation(current, &mut state.interpreter);
            state.generations.push(next);
        }
    }

    ui::draw(&state.generations, &mut state.buffer, &mut state.stdout)?;

    Ok(())
}
