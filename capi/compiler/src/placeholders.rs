use capi_process::InstructionAddress;

#[derive(Default)]
pub struct Placeholders {
    pub inner: Vec<CallToUserDefinedFunction>,
}

pub struct CallToUserDefinedFunction {
    pub name: String,
    pub address: InstructionAddress,
    pub is_tail_call: bool,
}
