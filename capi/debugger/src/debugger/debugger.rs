use capi_game_engine::memory::Memory;
use capi_process::{InstructionAddress, Process, Value};
use capi_protocol::updates::Code;

use super::{ActiveFunctions, Breakpoints};

#[derive(Clone, Debug, Default)]
pub struct Debugger {
    pub code: Option<Code>,
    pub breakpoints: Breakpoints,
    pub active_functions: ActiveFunctions,
    pub operands: Vec<Value>,
    pub memory: Option<Memory>,
}

impl Debugger {
    pub fn update(
        &mut self,
        code: Option<Code>,
        memory: Option<Memory>,
        process: Option<&Process>,
    ) {
        self.active_functions = ActiveFunctions::new(code.as_ref(), process);
        self.code = code;
        self.operands = process
            .map(|process| {
                process
                    .stack()
                    .operands_in_current_stack_frame()
                    .copied()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        self.memory = memory;
    }

    pub fn set_durable_breakpoint(
        &mut self,
        address: InstructionAddress,
    ) -> anyhow::Result<()> {
        let code = self.code.as_ref().expect(
            "Trying to set breakpoint from the UI, which means the code must \
            already be available to the UI. And therefore, must be available \
            in general.",
        );
        let instruction = code
            .instructions
            .get(&address)
            .expect(
                "Trying to set breakpoint for instruction from UI. Expecting \
                that instruction to exist, because otherwise, where would the \
                UI have gotten the address?",
            )
            .clone();

        self.breakpoints.set_durable(address, instruction);

        Ok(())
    }
}
