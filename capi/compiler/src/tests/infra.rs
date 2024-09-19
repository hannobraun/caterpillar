use std::collections::BTreeMap;

use capi_runtime::{Effect, Instructions, Runtime};

use crate::{compile, host::Host};

pub fn compile_and_run(source: &str) -> BTreeMap<u32, u32> {
    let (_, instructions, _) = compile::<TestHost>(source);

    let mut signals = BTreeMap::new();

    runtime().run_until_finished(&instructions, &mut signals);

    signals
}

pub fn runtime() -> TestRuntime {
    TestRuntime::default()
}

#[derive(Default)]
pub struct TestRuntime {
    inner: Runtime,
}

impl TestRuntime {
    pub fn run_until_finished(
        &mut self,
        instructions: &Instructions,
        signals: &mut BTreeMap<u32, u32>,
    ) -> &mut Self {
        while self.inner.state().is_running() {
            self.inner.evaluate_next_instruction(instructions);

            match self.inner.effects_mut().handle_first() {
                Some(Effect::Host) => {
                    let effect = self.inner.stack_mut().pop_operand().unwrap();
                    assert_eq!(effect.to_u32(), 0);

                    let channel = self.inner.stack_mut().pop_operand().unwrap();
                    let channel: u32 = u32::from_le_bytes(channel.0);

                    *signals.entry(channel).or_default() += 1;

                    self.inner.ignore_next_instruction();
                }
                Some(effect) => {
                    panic!(
                        "Unexpected effect: {effect}\n\
                        Runtime: {:#?}\n\
                        Instructions: {instructions:#?}",
                        self.inner,
                    );
                }
                None => {}
            }
        }

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
