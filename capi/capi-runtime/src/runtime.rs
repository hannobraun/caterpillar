use std::collections::VecDeque;

use crate::InstructionAddress;

pub type Function = VecDeque<InstructionAddress>;
