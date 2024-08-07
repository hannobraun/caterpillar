use capi_compiler::{
    fragments::{Fragment, FragmentExpression, FragmentPayload, Fragments},
    source_map::SourceMap,
};
use capi_game_engine::host::{GameEngineEffect, GameEngineHost};
use capi_process::{Effect, InstructionAddress, Process};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Block { expressions: Vec<Self> },
    Comment { text: String },
    Other(OtherExpression),
}

impl Expression {
    pub fn new(
        fragment: Fragment,
        fragments: &Fragments,
        source_map: &SourceMap,
        process: &Process<GameEngineHost>,
    ) -> Option<Self> {
        let fragment_id = fragment.id();
        let FragmentPayload::Expression { expression, .. } = fragment.payload
        else {
            return None;
        };

        if let FragmentExpression::Block { start, .. } = expression {
            let expressions = fragments
                .inner
                .iter_from(start)
                .cloned()
                .filter_map(|fragment| {
                    Self::new(fragment, fragments, source_map, process)
                })
                .collect();

            return Some(Self::Block { expressions });
        }
        if let FragmentExpression::Comment { text } = expression {
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

        let effect =
            process.state().first_unhandled_effect().and_then(|effect| {
                let effect_fragment = source_map
                    .instruction_to_fragment(
                        &process.state().most_recent_step().unwrap(),
                    )
                    .expect("Expecting effects to originate from user code.");

                if effect_fragment == fragment_id {
                    Some(effect.clone())
                } else {
                    None
                }
            });

        let is_on_call_stack = if let Some(instructions) = instructions {
            instructions.iter().copied().any(|mut instruction| {
                instruction.increment();

                process
                    .stack()
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
    pub expression: FragmentExpression,
    pub first_instruction: Option<InstructionAddress>,
    pub has_durable_breakpoint: bool,
    pub is_on_call_stack: bool,
    pub effect: Option<Effect<GameEngineEffect>>,
}
