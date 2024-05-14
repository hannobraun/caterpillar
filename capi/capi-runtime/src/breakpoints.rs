use std::collections::BTreeMap;

use crate::InstructionAddress;

pub type Breakpoints = BTreeMap<InstructionAddress, bool>;
