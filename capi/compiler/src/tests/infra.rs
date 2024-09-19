use std::collections::BTreeMap;

use capi_runtime::{Effect, Runtime};

use crate::{compile, host::Host};

pub fn compile_and_run(source: &str) -> BTreeMap<u32, u32> {
    let (_, instructions, _) = compile::<TestHost>(source);

    let mut signals = BTreeMap::new();

    let mut runtime = runtime();

    while runtime.state().is_running() {
        runtime.evaluate_next_instruction(&instructions);

        match runtime.effects_mut().handle_first() {
            Some(Effect::Host) => {
                let effect = runtime.stack_mut().pop_operand().unwrap();
                assert_eq!(effect.to_u32(), 0);

                let channel = runtime.stack_mut().pop_operand().unwrap();
                let channel: u32 = u32::from_le_bytes(channel.0);

                *signals.entry(channel).or_default() += 1;

                runtime.ignore_next_instruction();
            }
            Some(effect) => {
                panic!(
                    "Unexpected effect: {effect}\n\
                    Runtime: {runtime:#?}\n\
                    Instructions: {instructions:#?}",
                );
            }
            None => {}
        }
    }

    signals
}

pub fn runtime() -> Runtime {
    Runtime::default()
}

#[derive(Debug)]
struct TestHost {}

impl Host for TestHost {
    fn effect_number_to_function_name(effect: u8) -> Option<&'static str> {
        match effect {
            0 => Some("send"),
            _ => None,
        }
    }

    fn function_name_to_effect_number(name: &str) -> Option<u8> {
        match name {
            "send" => Some(0),
            _ => None,
        }
    }
}
