use std::collections::{BTreeMap, VecDeque};

use capi_process::{Effect, Instruction, InstructionAddress, Instructions};

use crate::{
    fragments::{
        Expression, Fragment, FragmentId, FragmentKind, FragmentMap, Fragments,
        Function, Parameters,
    },
    host::Host,
    intrinsics::Intrinsic,
    source_map::SourceMap,
    syntax::Pattern,
};

pub fn generate_instructions<H: Host>(
    fragments: Fragments,
) -> (Instructions, SourceMap) {
    let mut queue = VecDeque::new();
    let mut output = Output::default();
    let mut functions = Functions::default();

    // Create placeholder for call to `main` function, and the last return that
    // ends the process, if executed.
    let main = output.instructions.push(Instruction::TriggerEffect {
        effect: Effect::MissingMainFunction,
    });
    output.instructions.push(Instruction::Return);
    if let Some(id) = fragments.inner.find_function_by_name("main") {
        output.placeholders.push(CallToFunction {
            name: "main".to_string(),
            id,
            address: main,
            is_tail_call: true,
        });
    }

    // Seed the queue from the root context.
    compile_context::<H>(
        fragments.root,
        &fragments.inner,
        &mut output,
        &mut queue,
    );

    while let Some(unit) = queue.pop_front() {
        let CompileUnit {
            id,
            function,
            address,
        } = unit;

        let mut branches = Vec::new();

        for branch in function.branches {
            let parameters =
                branch.parameters.inner.iter().filter_map(|pattern| {
                    match pattern {
                        Pattern::Identifier { name } => Some(name),
                        Pattern::Literal { .. } => {
                            // Literal patterns are only relevant when
                            // selecting the branch to execute. They no
                            // longer have meaning once the function
                            // actually starts executing.
                            None
                        }
                    }
                });
            let bindings_address = output.generate_binding(parameters, id);

            let context_address = compile_context::<H>(
                branch.start,
                &fragments.inner,
                &mut output,
                &mut queue,
            );

            let address = bindings_address.unwrap_or(context_address);
            functions
                .by_id
                .entry(id)
                .or_default()
                .push((branch.parameters.clone(), address));

            branches.push(capi_process::Branch {
                parameters: branch
                    .parameters
                    .inner
                    .into_iter()
                    .map(|pattern| match pattern {
                        Pattern::Identifier { name } => {
                            capi_process::Pattern::Identifier { name }
                        }
                        Pattern::Literal { value } => {
                            capi_process::Pattern::Literal { value }
                        }
                    })
                    .collect(),
                start: address,
            });
        }

        if let Some(address) = address {
            output.instructions.replace(
                address,
                Instruction::MakeClosure {
                    branches,
                    environment: function.environment,
                },
            );
        } else {
            assert!(
                function.environment.is_empty(),
                "We were not provided an address where to put a \"make \
                closure \" instruction, and yet the function has an \
                environment. This is a bug.",
            );
        }
    }

    for call in output.placeholders {
        let Some(function) = functions.by_id.get(&call.id) else {
            // This won't happen for any regular function, because we only
            // create placeholders for functions that we actually encounter. But
            // it can happen for the `main` function, since we create a
            // placeholder for that unconditionally.
            //
            // If that happens, let's just leave the placeholder panic. It's not
            // great, as it doesn't provide any context to the user. But while
            // we don't have any way to make panics more descriptive, it'll have
            // to do.
            assert_eq!(
                &call.name, "main",
                "Replacement found for function that doesn't exist, but only \
                the replacement for the `main` function is generated without \
                encountering that first.",
            );
            continue;
        };
        let function = capi_process::Function {
            branches: function
                .iter()
                .map(|(parameters, address)| {
                    let parameters = parameters
                        .inner
                        .iter()
                        .cloned()
                        .map(|pattern| match pattern {
                            Pattern::Identifier { name } => {
                                capi_process::Pattern::Identifier { name }
                            }
                            Pattern::Literal { value } => {
                                capi_process::Pattern::Literal { value }
                            }
                        })
                        .collect();

                    capi_process::Branch {
                        parameters,
                        start: *address,
                    }
                })
                .collect(),
            environment: BTreeMap::new(),
        };

        output.instructions.replace(
            call.address,
            Instruction::CallFunction {
                function,
                is_tail_call: call.is_tail_call,
            },
        );
    }

    (output.instructions, output.source_map)
}

fn compile_context<H: Host>(
    start: FragmentId,
    fragments: &FragmentMap,
    output: &mut Output,
    queue: &mut VecDeque<CompileUnit>,
) -> InstructionAddress {
    let mut first_instruction = None;

    for fragment in fragments.iter_from(start) {
        let addr = compile_fragment::<H>(fragment, fragments, output, queue);
        first_instruction = first_instruction.or(addr);
    }

    let Some(first_instruction) = first_instruction else {
        unreachable!(
            "Must have generated at least one instruction for the block: the \
            return instruction. If this has not happened, the fragments have \
            somehow been missing a terminator."
        );
    };

    first_instruction
}

fn compile_fragment<H: Host>(
    fragment: &Fragment,
    fragments: &FragmentMap,
    output: &mut Output,
    queue: &mut VecDeque<CompileUnit>,
) -> Option<InstructionAddress> {
    match &fragment.kind {
        FragmentKind::Expression { expression, .. } => {
            match expression {
                Expression::CallToFunction { name, is_tail_call } => {
                    // We know that this expression refers to a user-defined
                    // function, but we might not have compiled that function
                    // yet.
                    //
                    // For now, just generate a placeholder that we can replace
                    // with the call later.
                    let address = output.generate_instruction(
                        Instruction::TriggerEffect {
                            effect: Effect::CompilerBug,
                        },
                        fragment.id(),
                    );

                    // We can't leave it at that, however. We need to make sure
                    // this placeholder actually gets replaced later, and we're
                    // doing that by adding it to this list.
                    if let Some(id) = fragments.find_function_by_name(name) {
                        output.placeholders.push(CallToFunction {
                            name: name.clone(),
                            id,
                            address,
                            is_tail_call: *is_tail_call,
                        });
                    }

                    Some(address)
                }
                Expression::CallToHostFunction { name } => {
                    match H::function_name_to_effect_number(name) {
                        Some(effect) => {
                            let address = output.generate_instruction(
                                Instruction::Push {
                                    value: effect.into(),
                                },
                                fragment.id(),
                            );
                            output.generate_instruction(
                                Instruction::TriggerEffect {
                                    effect: Effect::Host,
                                },
                                fragment.id(),
                            );
                            Some(address)
                        }
                        None => Some(output.generate_instruction(
                            Instruction::TriggerEffect {
                                effect: Effect::UnknownHostFunction,
                            },
                            fragment.id(),
                        )),
                    }
                }
                Expression::CallToIntrinsic {
                    intrinsic,
                    is_tail_call,
                } => match intrinsic {
                    Intrinsic::Eval => Some(output.generate_instruction(
                        Instruction::Eval {
                            is_tail_call: *is_tail_call,
                        },
                        fragment.id(),
                    )),
                },
                Expression::Comment { .. } => None,
                Expression::Function { function } => {
                    let address = if function.name.is_none() {
                        // If this is an anonymous function, we need to emit an
                        // instruction that allocates it, and takes care of its
                        // environment.
                        //
                        // But we haven't compiled the anonymous function yet,
                        // so we don't have the required information to do that.
                        // For now, let's create a placeholder for that
                        // instruction.
                        //
                        // Once the function gets compiled, we'll replace the
                        // placeholder with the real instruction.
                        Some(output.generate_instruction(
                            Instruction::TriggerEffect {
                                effect: Effect::CompilerBug,
                            },
                            fragment.id(),
                        ))
                    } else {
                        None
                    };

                    // And to make it happen later, we need to put what we
                    // already have into a queue. Once whatever's currently
                    // being compiled is out of the way, we can process that.
                    queue.push_front(CompileUnit {
                        id: fragment.id(),
                        function: function.clone(),
                        address,
                    });

                    address
                }
                Expression::ResolvedBinding { name } => {
                    Some(output.generate_instruction(
                        Instruction::BindingEvaluate { name: name.clone() },
                        fragment.id(),
                    ))
                }
                Expression::ResolvedBuiltinFunction { name } => {
                    Some(output.generate_instruction(
                        Instruction::CallBuiltin { name: name.clone() },
                        fragment.id(),
                    ))
                }
                Expression::UnresolvedIdentifier { name: _ } => {
                    Some(output.generate_instruction(
                        Instruction::TriggerEffect {
                            effect: Effect::UnresolvedIdentifier,
                        },
                        fragment.id(),
                    ))
                }
                Expression::Value(value) => Some(output.generate_instruction(
                    Instruction::Push { value: *value },
                    fragment.id(),
                )),
            }
        }
        FragmentKind::Terminator => Some(
            output.generate_instruction(Instruction::Return, fragment.id()),
        ),
    }
}

#[derive(Default)]
struct Output {
    instructions: Instructions,
    placeholders: Vec<CallToFunction>,
    source_map: SourceMap,
}

impl Output {
    fn generate_instruction(
        &mut self,
        instruction: Instruction,
        fragment_id: FragmentId,
    ) -> InstructionAddress {
        let addr = self.instructions.push(instruction);
        self.source_map.define_mapping(addr, fragment_id);
        addr
    }

    fn generate_binding<'r, N>(
        &mut self,
        names: N,
        fragment_id: FragmentId,
    ) -> Option<InstructionAddress>
    where
        N: IntoIterator<Item = &'r String>,
        N::IntoIter: DoubleEndedIterator,
    {
        let mut first_address = None;

        for name in names.into_iter().rev() {
            let address = self.generate_instruction(
                Instruction::Bind { name: name.clone() },
                fragment_id,
            );
            first_address = first_address.or(Some(address));
        }

        first_address
    }
}

pub struct CallToFunction {
    pub name: String,
    pub id: FragmentId,
    pub address: InstructionAddress,
    pub is_tail_call: bool,
}

#[derive(Default)]
struct Functions {
    by_id: BTreeMap<FragmentId, Vec<(Parameters, InstructionAddress)>>,
}

struct CompileUnit {
    id: FragmentId,
    function: Function,
    address: Option<InstructionAddress>,
}
