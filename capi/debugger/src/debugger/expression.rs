use capi_compiler::{
    repr::fragments::{
        Fragment, FragmentExpression, FragmentPayload, Fragments,
    },
    source_map::SourceMap,
};
use capi_process::{Effect, InstructionAddr, Process};
use capi_protocol::host::{GameEngineEffect, GameEngineHost};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
    Block {
        expressions: Vec<Self>,
    },
    Comment {
        text: String,
    },
    Other {
        expression: FragmentExpression,
        instruction: Option<InstructionAddr>,
        has_durable_breakpoint: bool,
        is_on_call_stack: bool,
        effect: Option<Effect<GameEngineEffect>>,
    },
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

        match expression {
            FragmentExpression::Block { start, .. } => {
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
            FragmentExpression::Comment { text } => {
                return Some(Self::Comment {
                    text: format!("# {text}"),
                });
            }
            _ => {}
        }

        let instruction = source_map.fragment_to_instruction(&fragment_id);

        let has_durable_breakpoint = if let Some(instruction) = &instruction {
            process.breakpoints().durable_at(instruction)
        } else {
            false
        };

        let effect =
            process.state().first_unhandled_effect().and_then(|effect| {
                let effect_fragment = source_map.instruction_to_fragment(
                    &process.state().most_recent_step().unwrap(),
                );

                if effect_fragment == fragment_id {
                    Some(effect.clone())
                } else {
                    None
                }
            });

        let is_on_call_stack = if let Some(instruction) = &instruction {
            process
                .stack()
                .is_next_instruction_in_any_frame(instruction)
        } else {
            false
        };

        Some(Self::Other {
            expression,
            instruction,
            has_durable_breakpoint,
            is_on_call_stack,
            effect,
        })
    }
}
