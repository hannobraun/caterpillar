use capi_runtime::{Effect, Heap, Runtime};

use crate::{host::Host, Compiler, Instructions};

pub fn runtime() -> TestRuntime {
    TestRuntime::default()
}

#[derive(Default)]
pub struct TestRuntime {
    compiler: Compiler,
    runtime: Runtime,
    instructions: Option<Instructions>,
    heap: Heap,
}

impl TestRuntime {
    pub fn update_code(&mut self, source: &str) -> &mut Self {
        let output = self.compiler.compile::<TestHost>(source);
        self.instructions = Some(output.instructions);
        self
    }

    pub fn run_until_effect(&mut self) -> Option<Effect> {
        let instructions = self
            .instructions
            .as_ref()
            .expect("Must call `update_code` before running.");

        while self.runtime.state().is_running() {
            self.runtime.evaluate_next_instruction(
                instructions.to_runtime_instructions(),
                &mut self.heap,
            );

            if let Some(effect) = self.runtime.effect_mut().handle() {
                return Some(effect);
            }
        }

        None
    }

    pub fn run_until_receiving(&mut self, expected_channel: u32) -> &mut Self {
        let Some(effect) = self.run_until_effect() else {
            panic!(
                "Waited to receive on channel `{expected_channel}`, but did \
                not receive anything."
            );
        };

        match effect {
            Effect::Host => {
                let effect = self.runtime.stack_mut().pop_operand().unwrap();
                assert_eq!(effect.to_u32(), 0);

                let channel = self.runtime.stack_mut().pop_operand().unwrap();
                let channel: u32 = u32::from_le_bytes(channel.0);

                if channel == expected_channel {
                    self.runtime.ignore_next_instruction();
                } else {
                    panic!(
                        "Received unexpected signal on channel `{channel}`. \
                        Expected to receive signal on channel \
                        `{expected_channel}`."
                    );
                }
            }
            effect => {
                let instructions = self.instructions.as_ref();

                panic!(
                    "Unexpected effect: {effect}\n\
                    Runtime: {:#?}\n\
                    Instructions: {instructions:#?}",
                    self.runtime,
                );
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
