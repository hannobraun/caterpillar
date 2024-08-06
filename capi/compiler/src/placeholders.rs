use capi_process::InstructionAddress;

pub struct CallToUserDefinedFunction {
    pub name: String,
    pub address: InstructionAddress,
    pub is_tail_call: bool,
}
