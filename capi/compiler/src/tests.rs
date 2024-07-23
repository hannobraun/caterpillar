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

use capi_process::{Effect, Host, Process, Stack};

use crate::{compile, repr::syntax::Script};

#[test]
fn closure_in_function() {
    let mut script = Script::default();
    script.function("main", [], |s| {
        s.v(0)
            .bind(["channel"])
            .block(|s| {
                s.r("channel").r("send");
            })
            .r("eval");
    });

    let (_, bytecode, _) = compile::<TestHost>(script);

    let mut signals = BTreeMap::new();

    let mut process = Process::<TestHost>::default();
    process.reset(&bytecode, []);

    while process.state().can_step() {
        process.step(&bytecode);

        while let Some(effect) = process.state().first_unhandled_effect() {
            match effect {
                Effect::Host(TestEffect { channel }) => {
                    *signals.entry(*channel).or_default() += 1;
                    process.handle_first_effect();
                }
                effect => {
                    panic!(
                        "Unexpected effect: {effect}\n\
                        Process: {process:#?}\n\
                        Bytecode: {bytecode:#?}"
                    );
                }
            }
        }
    }

    assert_eq!(signals.remove(&0), Some(1));
    assert!(signals.is_empty());
}

#[derive(Debug)]
struct TestHost {}

impl Host for TestHost {
    type Effect = TestEffect;

    fn function(
        name: &str,
    ) -> Option<capi_process::HostFunction<Self::Effect>> {
        match name {
            "send" => Some(send),
            _ => None,
        }
    }
}

fn send(stack: &mut Stack) -> Result<(), Effect<TestEffect>> {
    let channel = stack.pop_operand()?;
    let channel: u32 = u32::from_le_bytes(channel.0);

    Err(Effect::Host(TestEffect { channel }))
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
struct TestEffect {
    channel: u32,
}
