//! # End-to-end testing for `capi-compiler` and `capi-process`
//!
//! ## Implementation Note
//!
//! That this module lives in `capi-compiler` is a practical decision. The crate
//! depends on `capi-process` anyway, so we have everything here that we need.
//!
//! But it's a bit weird, because these tests explicitly cover both crates. And
//! in the future, when we can do hot code reloading, we'll need tests for that
//! too. It's not clear to me whether those should then go somewhere else, or if
//! we then need a central place for all of them.

use std::collections::BTreeMap;

use capi_process::{Effect, Process};

use crate::{compile, host::Host};

#[test]
fn anonymous_function_eval() {
    let source = r"
        main: { ||
            { || 0 send }
                eval
        }
    ";

    let mut signals = compile_and_run(source);

    assert_eq!(signals.remove(&0), Some(1));
    assert!(signals.is_empty());
}

fn compile_and_run(source: &str) -> BTreeMap<u32, u32> {
    let (_, instructions, _) = compile::<TestHost>(source);

    let mut signals = BTreeMap::new();

    let mut process = Process::default();
    process.reset([]);

    while process.can_step() {
        process.step(&instructions);

        while let Some(effect) = process.effects().first() {
            match effect {
                Effect::Host => {
                    let effect = process.stack_mut().pop_operand().unwrap();
                    assert_eq!(effect.to_u32(), 0);

                    let channel = process.stack_mut().pop_operand().unwrap();
                    let channel: u32 = u32::from_le_bytes(channel.0);

                    *signals.entry(channel).or_default() += 1;
                    process.effects_mut().handle_first();
                }
                effect => {
                    panic!(
                        "Unexpected effect: {effect}\n\
                        Process: {process:#?}\n\
                        Instructions: {instructions:#?}",
                    );
                }
            }
        }
    }

    signals
}

#[derive(Debug)]
struct TestHost {}

impl Host for TestHost {
    fn function_name_to_effect_number(name: &str) -> Option<u8> {
        match name {
            "send" => Some(0),
            _ => None,
        }
    }
}
