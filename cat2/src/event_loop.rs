use std::io;

use crate::{
    cells::{self, Generation},
    cp::{self, Functions},
    ui,
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
    pub functions: Functions,
    pub generations: Vec<Generation>,
    pub buffer: ui::Buffer,
    pub stdout: io::Stdout,
}

pub fn run_once(event: Event, state: &mut State) -> anyhow::Result<()> {
    match event {
        Event::Key(Key::Backspace) => {
            state.functions.get_mut("cell_is_born").pop();
        }
        Event::Key(Key::Char(ch)) => {
            state.functions.get_mut("cell_is_born").push(ch);
        }
        Event::Tick => {
            let current = state
                .generations
                .last()
                .cloned()
                .unwrap_or_else(cells::init);

            let mut interpreter = cp::Interpreter::new();

            // We only add new generations, but never delete them. This is fine
            // for now, I think. Let's just hope nobody runs this for long
            // enough to fill up their main memory.
            let next = cells::next_generation(
                current,
                &mut interpreter,
                &state.functions,
            );
            state.generations.push(next);
        }
    }

    ui::draw(
        &state.generations,
        &state.functions,
        &mut state.buffer,
        &mut state.stdout,
    )?;

    Ok(())
}
