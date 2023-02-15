use std::io;

use crate::{
    cells::{self, Generation},
    cp::{self, Arg, Type},
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
    pub interpreter: cp::Interpreter,
    pub generations: Vec<Generation>,
    pub buffer: ui::Buffer,
    pub stdout: io::Stdout,
}

pub fn run_once(event: Event, state: &mut State) -> anyhow::Result<()> {
    match event {
        Event::Key(Key::Backspace) => {
            let function = state
                .interpreter
                .functions
                .get_mut("cell_is_born", [Arg::Type(Type::U8)])
                .unwrap();
            let mut token = function
                .body
                .pop()
                .map(|cp::Token::Fn(token)| token)
                .unwrap_or_default();

            token.pop();

            if !token.is_empty() {
                function.body.push(cp::Token::Fn(token));
            }
        }
        Event::Key(Key::Char(ch)) => {
            let function = state
                .interpreter
                .functions
                .get_mut("cell_is_born", [Arg::Type(Type::U8)])
                .unwrap();
            let mut token = function
                .body
                .pop()
                .map(|cp::Token::Fn(token)| token)
                .unwrap_or_default();

            token.push(ch);

            function.body.push(cp::Token::Fn(token));
        }
        Event::Tick => {
            let current = state
                .generations
                .last()
                .cloned()
                .unwrap_or_else(cells::init);

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
