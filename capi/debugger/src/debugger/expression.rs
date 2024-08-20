use capi_compiler::{
    fragments::{self, Fragment, FragmentPayload, Fragments},
    source_map::SourceMap,
};
use capi_process::{Effect, InstructionAddress, Process};

use super::Function;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Comment { text: String },
    Function { function: Function },
    Other(OtherExpression),
}

impl Expression {
    pub fn new(
        fragment: Fragment,
        fragments: &Fragments,
        source_map: &SourceMap,
        process: &Process,
    ) -> Option<Self> {
        let fragment_id = fragment.id();
        let FragmentPayload::Expression { expression, .. } = fragment.payload
        else {
            return None;
        };

        if let fragments::Expression::Function { function } = expression {
            let function =
                Function::new(function, fragments, source_map, process);

            return Some(Self::Function { function });
        }
        if let fragments::Expression::Comment { text } = expression {
            return Some(Self::Comment {
                text: format!("# {text}"),
            });
        }

        let instructions = source_map.fragment_to_instructions(&fragment_id);

        let has_durable_breakpoint = if let Some(instructions) = &instructions {
            instructions.iter().any(|instruction| {
                process.breakpoints().durable_at(instruction)
            })
        } else {
            false
        };

        let effect = process.effects().first().and_then(|effect| {
            let effect_fragment = source_map
                .instruction_to_fragment(&process.most_recent_step().unwrap())
                .expect("Expecting effects to originate from user code.");

            if effect_fragment == fragment_id {
                Some(*effect)
            } else {
                None
            }
        });

        let is_on_call_stack = if let Some(instructions) = instructions {
            instructions.iter().copied().any(|mut instruction| {
                instruction.increment();

                process
                    .evaluator()
                    .active_instructions()
                    .any(|next| next == instruction)
            })
        } else {
            false
        };

        Some(Self::Other(OtherExpression {
            expression,
            first_instruction: instructions
                .and_then(|instruction| instruction.first().copied()),
            has_durable_breakpoint,
            is_on_call_stack,
            effect,
        }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OtherExpression {
    pub expression: fragments::Expression,
    pub first_instruction: Option<InstructionAddress>,
    pub has_durable_breakpoint: bool,
    pub is_on_call_stack: bool,
    pub effect: Option<Effect>,
}
