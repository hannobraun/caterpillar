use std::collections::{BTreeMap, BTreeSet, VecDeque};

use capi_process::{Effect, Instruction, InstructionAddress, Instructions};

use crate::{
    fragments::{
        Branch, Fragment, FragmentExpression, FragmentId, FragmentMap,
        FragmentPayload, Fragments, Function, Parameters,
    },
    host::Host,
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
        effect: Effect::Panic,
    });
    output.instructions.push(Instruction::Return);
    output.placeholders.push(CallToFunction {
        name: "main".to_string(),
        address: main,
        is_tail_call: true,
    });

    // Seed the queue from the root context.
    compile_context::<H>(
        fragments.root,
        &fragments.inner,
        &mut output,
        &mut queue,
    );

    while let Some(unit) = queue.pop_front() {
        match unit {
            CompileUnit::Block {
                start,
                environment,
                address,
            } => {
                let start = compile_context::<H>(
                    start,
                    &fragments.inner,
                    &mut output,
                    &mut queue,
                );

                output.instructions.replace(
                    address,
                    Instruction::MakeClosure {
                        address: start,
                        environment,
                    },
                );
            }
            CompileUnit::Function { id, name, branches } => {
                for branch in branches {
                    let arguments =
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
                    let bindings_address =
                        output.generate_binding(arguments, id);

                    let context_address = compile_context::<H>(
                        branch.start,
                        &fragments.inner,
                        &mut output,
                        &mut queue,
                    );

                    let address = bindings_address.unwrap_or(context_address);
                    functions
                        .by_name
                        .entry(name.clone())
                        .or_default()
                        .push((branch.parameters, address));
                }
            }
        }
    }

    for call in output.placeholders {
        let Some(function) = functions.by_name.get(&call.name) else {
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
        let function = function
            .iter()
            .map(|(arguments, address)| {
                let arguments = arguments
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
                (arguments, *address)
            })
            .collect();

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
        let addr = compile_fragment::<H>(fragment, output, queue);
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
    output: &mut Output,
    queue: &mut VecDeque<CompileUnit>,
) -> Option<InstructionAddress> {
    let addr = match &fragment.payload {
        FragmentPayload::Function {
            function: Function { name, branches },
            ..
        } => {
            queue.push_back(CompileUnit::Function {
                id: fragment.id(),
                name: name.clone(),
                branches: branches.clone(),
            });
            return None;
        }
        FragmentPayload::Expression { expression, .. } => {
            match expression {
                FragmentExpression::BindingDefinitions { names } => {
                    let bindings_address =
                        output.generate_binding(names, fragment.id());

                    let assert_address = output.generate_instruction(
                        Instruction::AssertBindingLeftNoOperands,
                        fragment.id(),
                    );

                    bindings_address.unwrap_or(assert_address)
                }
                FragmentExpression::Block { start, environment } => {
                    // We are currently compiling a function or block (otherwise
                    // we wouldn't be encountering any expression), and the
                    // instructions for that will be executed linearly.
                    //
                    // Which means we can't just start compiling this block
                    // right now. Its instructions would go into the middle of
                    // those other instructions and mess everything up.
                    //
                    // What _should_ happen instead, is that the block is turned
                    // into a closure, that can be passed around as a value and
                    // called whenever so desired.
                    //
                    // So for now, let's just generate this instruction as a
                    // placeholder, to be replaced with another instruction that
                    // creates that closure, once we have everything in place to
                    // make that happen.
                    let address = output.generate_instruction(
                        Instruction::TriggerEffect {
                            effect: Effect::Panic,
                        },
                        fragment.id(),
                    );

                    // And to make it happen later, we need to put what we
                    // already have into a queue. Once whatever's currently
                    // being compiled is out of the way, we can process that.
                    queue.push_front(CompileUnit::Block {
                        start: *start,
                        environment: environment.clone(),
                        address,
                    });

                    address
                }
                FragmentExpression::Comment { .. } => {
                    return None;
                }
                FragmentExpression::ResolvedBinding { name } => output
                    .generate_instruction(
                        Instruction::BindingEvaluate { name: name.clone() },
                        fragment.id(),
                    ),
                FragmentExpression::ResolvedBuiltinFunction { name } => {
                    // Here we check for special built-in functions that are
                    // implemented differently, without making sure anywhere,
                    // that their name doesn't conflict with any user-defined
                    // functions.
                    //
                    // I think it's fine for now. This seems like a temporary
                    // hack anyway, while the language is not powerful enough to
                    // support real conditionals.
                    let instruction = if name == "return_if_non_zero" {
                        Instruction::ReturnIfNonZero
                    } else {
                        Instruction::CallBuiltin { name: name.clone() }
                    };

                    output.generate_instruction(instruction, fragment.id())
                }
                FragmentExpression::ResolvedFunction {
                    name,
                    is_tail_call,
                    ..
                } => {
                    // We know that this expression refers to a user-defined
                    // function, but we might not have compiled that function
                    // yet.
                    //
                    // For now, just generate a placeholder that we can replace
                    // with the call later.
                    let address = output.generate_instruction(
                        Instruction::TriggerEffect {
                            effect: Effect::Panic,
                        },
                        fragment.id(),
                    );

                    // We can't leave it at that, however. We need to make sure
                    // this placeholder actually gets replaced later, and we're
                    // doing that by adding it to this list.
                    output.placeholders.push(CallToFunction {
                        name: name.clone(),
                        address,
                        is_tail_call: *is_tail_call,
                    });

                    address
                }
                FragmentExpression::ResolvedHostFunction { name } => {
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
                            address
                        }
                        None => output.generate_instruction(
                            Instruction::TriggerEffect {
                                effect: Effect::Panic,
                            },
                            fragment.id(),
                        ),
                    }
                }

                FragmentExpression::UnresolvedIdentifier { name: _ } => output
                    .generate_instruction(
                        Instruction::TriggerEffect {
                            effect: Effect::Panic,
                        },
                        fragment.id(),
                    ),
                FragmentExpression::Value(value) => output
                    .generate_instruction(
                        Instruction::Push { value: *value },
                        fragment.id(),
                    ),
            }
        }
        FragmentPayload::Terminator => {
            output.generate_instruction(Instruction::Return, fragment.id())
        }
    };

    Some(addr)
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
    pub address: InstructionAddress,
    pub is_tail_call: bool,
}

#[derive(Default)]
struct Functions {
    by_name: BTreeMap<String, Vec<(Parameters, InstructionAddress)>>,
}

enum CompileUnit {
    Block {
        start: FragmentId,
        environment: BTreeSet<String>,
        address: InstructionAddress,
    },
    Function {
        id: FragmentId,
        name: String,
        branches: Vec<Branch>,
    },
}
