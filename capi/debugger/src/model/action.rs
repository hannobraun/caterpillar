use capi_compiler::fragments::FragmentId;
use capi_process::InstructionAddress;

#[derive(Clone)]
pub enum Action {
    BreakpointClear {
        fragment: FragmentId,
        address: InstructionAddress,
    },
    BreakpointSet {
        fragment: FragmentId,
        address: InstructionAddress,
    },
    Continue,
    Reset,
    Step,
    Stop,
}
