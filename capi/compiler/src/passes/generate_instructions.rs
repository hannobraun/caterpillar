use std::collections::{BTreeMap, BTreeSet, VecDeque};

use capi_process::{Host, Instruction, InstructionAddress, Instructions};

use crate::{
    fragments::{
        Cluster, Fragment, FragmentExpression, FragmentId, FragmentMap,
        FragmentPayload, Fragments, Function,
    },
    source_map::SourceMap,
    syntax::Pattern,
};

pub fn generate_instructions<H: Host>(
    fragments: Fragments,
) -> (Instructions, SourceMap) {
    let mut queue = VecDeque::new();
    let mut output = Output::default();
    let mut clusters = Clusters::default();

    // Create placeholder for call to `main` function, and the last return that
    // ends the process, if executed.
    let main = output.instructions.push(Instruction::Panic);
    output.instructions.push(Instruction::Return);
    output.placeholders.push(CallToCluster {
        name: "main".to_string(),
        arguments: H::arguments_to_main(),
        address: main,
        is_tail_call: true,
    });

    // Seed the queue from the root context.
    compile_context(fragments.root, &fragments.inner, &mut output, &mut queue);

    while let Some(unit) = queue.pop_front() {
        match unit {
            CompileUnit::Block {
                start,
                environment,
                address,
            } => {
                let start = compile_context(
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
            CompileUnit::Cluster { name, members } => {
                for function in members {
                    let address = compile_context(
                        function.start,
                        &fragments.inner,
                        &mut output,
                        &mut queue,
                    );

                    clusters
                        .addresses_by_name
                        .entry(name.clone())
                        .or_default()
                        .push(address);
                }
            }
        }
    }

    for call in output.placeholders {
        let Some(addresses) = clusters.addresses_by_name.get(&call.name) else {
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

        assert_eq!(
            addresses.len(),
            1,
            "Pattern matching in function definitions is not supported yet.",
        );
        let address = addresses
            .first()
            .expect("Just checked that there is one address");

        output.instructions.replace(
            call.address,
            Instruction::CallCluster {
                arguments: call.arguments,
                address: *address,
                is_tail_call: call.is_tail_call,
            },
        );
    }

    (output.instructions, output.source_map)
}

fn compile_context(
    start: FragmentId,
    fragments: &FragmentMap,
    output: &mut Output,
    queue: &mut VecDeque<CompileUnit>,
) -> InstructionAddress {
    let mut first_instruction = None;

    for fragment in fragments.iter_from(start) {
        let addr = compile_fragment(fragment, output, queue);
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

fn compile_fragment(
    fragment: &Fragment,
    output: &mut Output,
    queue: &mut VecDeque<CompileUnit>,
) -> Option<InstructionAddress> {
    let addr = match &fragment.payload {
        FragmentPayload::Cluster {
            cluster: Cluster { name, members },
            ..
        } => {
            queue.push_back(CompileUnit::Cluster {
                name: name.clone(),
                members: members.clone(),
            });
            return None;
        }
        FragmentPayload::Expression { expression, .. } => {
            match expression {
                FragmentExpression::BindingDefinitions { names } => output
                    .generate_instruction(
                        Instruction::BindingsDefine {
                            names: names.clone(),
                        },
                        fragment.id(),
                    ),
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
                        Instruction::Panic,
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
                    } else if name == "return_if_zero" {
                        Instruction::ReturnIfZero
                    } else {
                        Instruction::CallBuiltin { name: name.clone() }
                    };

                    output.generate_instruction(instruction, fragment.id())
                }
                FragmentExpression::ResolvedCluster {
                    name,
                    all_arguments,
                    is_tail_call,
                } => {
                    // We are still in the process of implementing pattern
                    // matching in function definitions. For now, we simply
                    // don't support it here.
                    assert_eq!(
                        all_arguments.len(),
                        1,
                        "Pattern matching in function definitions is not \
                        supported yet."
                    );
                    let arguments = all_arguments
                        .first()
                        .expect(
                            "Each cluster must have at least one member, which \
                            means we can expect to find one set of arguments \
                            here.",
                        )
                        .inner
                        .iter()
                        .map(|argument| match argument {
                            Pattern::Identifier { name } => name.clone(),
                            Pattern::Literal { .. } => {
                                panic!(
                                    "Pattern matching in function definitions \
                                    is not supported yet."
                                );
                            }
                        })
                        .collect();

                    // We know that this expression refers to a user-defined
                    // function, but we might not have compiled that function
                    // yet.
                    //
                    // For now, just generate a placeholder that we can replace
                    // with the call later.
                    let address = output.generate_instruction(
                        Instruction::Panic,
                        fragment.id(),
                    );

                    // We can't leave it at that, however. We need to make sure
                    // this placeholder actually gets replaced later, and we're
                    // doing that by adding it to this list.
                    output.placeholders.push(CallToCluster {
                        name: name.clone(),
                        arguments,
                        address,
                        is_tail_call: *is_tail_call,
                    });

                    address
                }
                FragmentExpression::ResolvedHostFunction { name } => output
                    .generate_instruction(
                        Instruction::CallBuiltin { name: name.clone() },
                        fragment.id(),
                    ),
                FragmentExpression::UnresolvedIdentifier { name: _ } => output
                    .generate_instruction(Instruction::Panic, fragment.id()),
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
    placeholders: Vec<CallToCluster>,
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
}

pub struct CallToCluster {
    pub name: String,
    pub arguments: Vec<String>,
    pub address: InstructionAddress,
    pub is_tail_call: bool,
}

#[derive(Default)]
struct Clusters {
    addresses_by_name: BTreeMap<String, Vec<InstructionAddress>>,
}

enum CompileUnit {
    Block {
        start: FragmentId,
        environment: BTreeSet<String>,
        address: InstructionAddress,
    },
    Cluster {
        name: String,
        members: Vec<Function>,
    },
}
