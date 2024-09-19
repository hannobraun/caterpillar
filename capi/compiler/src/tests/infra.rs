use std::collections::BTreeMap;

use capi_runtime::{Effect, Instructions, Runtime};

use crate::{compile, host::Host};

pub fn compile_and_run(source: &str) {
    let mut runtime = runtime();

    runtime.update_code(source).run_until_receiving(0);
}

pub fn runtime() -> TestRuntime {
    TestRuntime::default()
}

#[derive(Default)]
pub struct TestRuntime {
    runtime: Runtime,
    signals: BTreeMap<u32, u32>,
    instructions: Option<Instructions>,
}

impl TestRuntime {
    pub fn update_code(&mut self, source: &str) -> &mut Self {
        let (_, instructions, _) = compile::<TestHost>(source);
        self.instructions = Some(instructions);
        self
    }

    pub fn run_until_receiving(&mut self, signal: u32) -> &mut Self {
        let instructions = self
            .instructions
            .as_ref()
            .expect("Must call `update_code` before running.");

        while self.runtime.state().is_running()
            && self.signals.get(&signal) != Some(&1)
        {
            self.runtime.evaluate_next_instruction(instructions);

            match self.runtime.effects_mut().handle_first() {
                Some(Effect::Host) => {
                    let effect =
                        self.runtime.stack_mut().pop_operand().unwrap();
                    assert_eq!(effect.to_u32(), 0);

                    let channel =
                        self.runtime.stack_mut().pop_operand().unwrap();
                    let channel: u32 = u32::from_le_bytes(channel.0);

                    *self.signals.entry(channel).or_default() += 1;

                    self.runtime.ignore_next_instruction();
                }
                Some(effect) => {
                    panic!(
                        "Unexpected effect: {effect}\n\
                        Runtime: {:#?}\n\
                        Instructions: {instructions:#?}",
                        self.runtime,
                    );
                }
                None => {}
            }
        }

        assert_eq!(self.signals.get(&signal), Some(&1));

        self
    }
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
