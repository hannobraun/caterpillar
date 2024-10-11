use std::collections::{BTreeMap, VecDeque};

use capi_runtime::{Effect, Instruction, InstructionAddress, Instructions};

use crate::{
    fragments::{
        Branch, BranchLocation, Changes, Fragment, FragmentLocation, Fragments,
        Function, FunctionLocation, Parameters, UpdatedFunction,
    },
    hash::Hash,
    intrinsics::Intrinsic,
    source_map::{Mapping, SourceMap},
    syntax::{Cluster, Pattern},
};

pub fn generate_instructions(
    fragments: &Fragments,
    changes: &Changes,
    instructions: &mut Instructions,
    source_map: &mut SourceMap,
) {
    let mut queue = VecDeque::new();
    let mut output = Output {
        instructions,
        source_map,
        placeholders: Vec::new(),
    };
    let mut functions = BTreeMap::default();

    // Create placeholder for call to `main` function.
    //
    // If there's no `main` function, this won't get replaced. Since this is a
    // result of invalid code, an instruction generating the `BuildError` effect
    // is an appropriate placeholder.
    let call_to_main = output.instructions.push(Instruction::TriggerEffect {
        effect: Effect::BuildError,
    });
    if let Some(function) = fragments.find_function_by_name("main") {
        output.placeholders.push(CallToFunction {
            hash: Hash::new(&function),
            address: call_to_main,
            is_tail_call: true,
        });
    }

    let added_and_updated_functions =
        changes.added.iter().chain(changes.updated.iter().map(
            |UpdatedFunction {
                 new: (index, function),
                 ..
             }| (index, function),
        ));
    for (&index, function) in added_and_updated_functions {
        let cluster = fragments
            .clusters
            .iter()
            .find(|cluster| cluster.functions.values().any(|i| i == &index))
            .expect("All named functions are part of a cluster.");

        queue.push_front(FunctionToCompile {
            function: function.clone(),
            location: FunctionLocation::NamedFunction { index },
            cluster: cluster.clone(),
            address_of_instruction_to_make_anon_function: None,
        });
    }

    while let Some(function_to_compile) = queue.pop_front() {
        compile_function(
            function_to_compile,
            fragments,
            &mut output,
            &mut queue,
            &mut functions,
        );
    }

    for call in output.placeholders {
        let Some(function) = functions.get(&call.hash) else {
            // This won't happen for any regular function, because we only
            // create placeholders for functions that we actually encounter. But
            // it can happen for the `main` function, since we create a
            // placeholder for that unconditionally.
            //
            // If that happens, let's just leave the placeholder panic. It's not
            // great, as it doesn't provide any context to the user. But while
            // we don't have any way to make panics more descriptive, it'll have
            // to do.
            continue;
        };
        let function = capi_runtime::Function {
            branches: function
                .iter()
                .map(|(parameters, address)| {
                    let parameters = parameters
                        .inner
                        .iter()
                        .cloned()
                        .map(|pattern| match pattern {
                            Pattern::Identifier { name } => {
                                capi_runtime::Pattern::Identifier { name }
                            }
                            Pattern::Literal { value } => {
                                capi_runtime::Pattern::Literal { value }
                            }
                        })
                        .collect();

                    capi_runtime::Branch {
                        parameters,
                        start: *address,
                    }
                })
                .collect(),
            environment: BTreeMap::new(),
        };

        output.instructions.replace(
            &call.address,
            Instruction::CallFunction {
                function,
                is_tail_call: call.is_tail_call,
            },
        );
    }
}

fn compile_function(
    function_to_compile: FunctionToCompile,
    fragments: &Fragments,
    output: &mut Output,
    queue: &mut VecDeque<FunctionToCompile>,
    functions: &mut BTreeMap<
        Hash<Function>,
        Vec<(Parameters, InstructionAddress)>,
    >,
) {
    let FunctionToCompile {
        function,
        location,
        cluster,
        address_of_instruction_to_make_anon_function,
    } = function_to_compile;

    let mut branches = Vec::new();
    let mut instruction_range = None;

    for (&index, branch) in function.branches.iter() {
        let parameters = branch.parameters.inner.iter().filter_map(|pattern| {
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
        let bindings_address = output.generate_binding(parameters);

        let [branch_address, last_address] = compile_branch(
            branch,
            BranchLocation {
                parent: Box::new(location.clone()),
                index,
            },
            &cluster,
            fragments,
            output,
            queue,
        );

        let first_address = bindings_address.unwrap_or(branch_address);
        functions
            .entry(Hash::new(&function))
            .or_default()
            .push((branch.parameters.clone(), first_address));

        branches.push(capi_runtime::Branch {
            parameters: branch
                .parameters
                .inner
                .iter()
                .cloned()
                .map(|pattern| match pattern {
                    Pattern::Identifier { name } => {
                        capi_runtime::Pattern::Identifier { name }
                    }
                    Pattern::Literal { value } => {
                        capi_runtime::Pattern::Literal { value }
                    }
                })
                .collect(),
            start: first_address,
        });

        instruction_range = {
            let [first_in_function, _last_in_function] =
                instruction_range.unwrap_or([first_address, last_address]);

            Some([first_in_function, last_address])
        };
    }

    if let Some(instruction_range) = instruction_range {
        output
            .source_map
            .map_function_to_instructions(location, instruction_range);
    }

    if let Some(address) = address_of_instruction_to_make_anon_function {
        output.instructions.replace(
            &address,
            Instruction::MakeAnonymousFunction {
                branches,
                environment: function.environment,
            },
        );
    } else {
        assert!(
            function.environment.is_empty(),
            "We were not provided an address where to put a \"make anonymous \
            function\" instruction, and yet the function has an environment. \
            This is a bug.",
        );
    }
}

fn compile_branch(
    branch: &Branch,
    location: BranchLocation,
    cluster: &Cluster,
    fragments: &Fragments,
    output: &mut Output,
    queue: &mut VecDeque<FunctionToCompile>,
) -> [InstructionAddress; 2] {
    let mut first_instruction = None;

    for (&index, fragment) in &branch.body {
        let addr = compile_fragment(
            fragment,
            FragmentLocation {
                parent: Box::new(location.clone()),
                index,
            },
            cluster,
            fragments,
            output,
            queue,
        );
        first_instruction = first_instruction.or(addr);
    }

    // Unconditionally generating a return instruction, like we do here, is
    // redundant. If the previous fragment was a tail call, it didn't create a
    // new stack frame.
    //
    // In this case, the return instruction at the end of the called function
    // returns to the current function's caller, and we never get to the return
    // we generated here. It's just a junk instruction that has no effect,
    // except to make the code bigger.
    //
    // I don't think it's worth fixing right now, for the following reasons:
    //
    // - Tail call elimination still partially happens at runtime. The
    //   plan is to move it to compile-time completely. Adding other
    //   optimizations (like omitting this return instruction) will make
    //   this transition more complicated, for little gain in the
    //   meantime.
    // - There's no infrastructure in place to measure the impact of
    //   compiler optimizations. I'd rather have that, instead of making
    //   this change blindly. It will probably make the code more
    //   complicated, so it needs to be justified.
    let last_instruction =
        generate_instruction(Instruction::Return, output.instructions, None);

    let first_instruction = first_instruction.unwrap_or(last_instruction);

    [first_instruction, last_instruction]
}

fn compile_fragment(
    fragment: &Fragment,
    location: FragmentLocation,
    cluster: &Cluster,
    fragments: &Fragments,
    output: &mut Output,
    queue: &mut VecDeque<FunctionToCompile>,
) -> Option<InstructionAddress> {
    match &fragment {
        Fragment::CallToFunction {
            hash, is_tail_call, ..
        } => {
            // We know that this expression refers to a user-defined function,
            // but we might not have compiled that function yet.
            //
            // For now, just generate a placeholder that we can replace with the
            // call later.
            let address = generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::CompilerBug,
                },
                output.instructions,
                Some(
                    &mut output
                        .source_map
                        .map_fragment_to_instructions(location),
                ),
            );

            // We can't leave it at that, however. We need to make sure this
            // placeholder actually gets replaced later, and we're doing that by
            // adding it to this list.
            output.placeholders.push(CallToFunction {
                hash: *hash,
                address,
                is_tail_call: *is_tail_call,
            });

            Some(address)
        }
        Fragment::CallToFunctionRecursive {
            index,
            is_tail_call,
        } => {
            let function_index_in_root_context = cluster.functions[index];
            let called_function = fragments
                .functions
                .get(&function_index_in_root_context)
                .expect("Function referred to from cluster must exist.");
            let hash = Hash::new(called_function);

            // We know that this expression refers to a user-defined function,
            // but we might not have compiled that function yet.
            //
            // For now, just generate a placeholder that we can replace with the
            // call later.
            let address = generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::CompilerBug,
                },
                output.instructions,
                Some(
                    &mut output
                        .source_map
                        .map_fragment_to_instructions(location),
                ),
            );

            // We can't leave it at that, however. We need to make sure this
            // placeholder actually gets replaced later, and we're doing that by
            // adding it to this list.
            output.placeholders.push(CallToFunction {
                hash,
                address,
                is_tail_call: *is_tail_call,
            });

            Some(address)
        }
        Fragment::CallToHostFunction { effect_number } => {
            let mut mapping =
                output.source_map.map_fragment_to_instructions(location);

            let address = generate_instruction(
                Instruction::Push {
                    value: (*effect_number).into(),
                },
                output.instructions,
                Some(&mut mapping),
            );
            generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::Host,
                },
                output.instructions,
                Some(&mut mapping),
            );
            Some(address)
        }
        Fragment::CallToIntrinsic {
            intrinsic,
            is_tail_call,
        } => {
            let instruction =
                intrinsic_to_instruction(intrinsic, *is_tail_call);

            Some(generate_instruction(
                instruction,
                output.instructions,
                Some(
                    &mut output
                        .source_map
                        .map_fragment_to_instructions(location),
                ),
            ))
        }
        Fragment::Comment { .. } => None,
        Fragment::Function { function } => {
            let address_of_instruction_to_make_anon_function =
                if function.name.is_none() {
                    // If this is an anonymous function, we need to emit an
                    // instruction that allocates it, and takes care of its
                    // environment.
                    //
                    // But we haven't compiled the anonymous function yet, so we
                    // don't have the required information to do that. For now,
                    // let's create a placeholder for that instruction.
                    //
                    // Once the function gets compiled, we'll replace the
                    // placeholder with the real instruction.
                    Some(generate_instruction(
                        Instruction::TriggerEffect {
                            effect: Effect::CompilerBug,
                        },
                        output.instructions,
                        Some(
                            &mut output
                                .source_map
                                .map_fragment_to_instructions(location.clone()),
                        ),
                    ))
                } else {
                    None
                };

            // And to make it happen later, we need to put what we already have
            // into a queue. Once whatever's currently being compiled is out of
            // the way, we can process that.
            queue.push_front(FunctionToCompile {
                function: function.clone(),
                location: FunctionLocation::AnonymousFunction { location },
                cluster: cluster.clone(),
                address_of_instruction_to_make_anon_function,
            });

            address_of_instruction_to_make_anon_function
        }
        Fragment::ResolvedBinding { name } => Some(generate_instruction(
            Instruction::BindingEvaluate { name: name.clone() },
            output.instructions,
            Some(&mut output.source_map.map_fragment_to_instructions(location)),
        )),
        Fragment::UnresolvedIdentifier { name: _ } => {
            Some(generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::BuildError,
                },
                output.instructions,
                Some(
                    &mut output
                        .source_map
                        .map_fragment_to_instructions(location),
                ),
            ))
        }
        Fragment::Value(value) => Some(generate_instruction(
            Instruction::Push { value: *value },
            output.instructions,
            Some(&mut output.source_map.map_fragment_to_instructions(location)),
        )),
    }
}

fn intrinsic_to_instruction(
    intrinsic: &Intrinsic,
    is_tail_call: bool,
) -> Instruction {
    match intrinsic {
        Intrinsic::AddS8 => Instruction::AddS8,
        Intrinsic::AddS32 => Instruction::AddS32,
        Intrinsic::AddU8 => Instruction::AddU8,
        Intrinsic::AddU8Wrap => Instruction::AddU8Wrap,
        Intrinsic::And => Instruction::LogicalAnd,
        Intrinsic::Brk => Instruction::TriggerEffect {
            effect: Effect::Breakpoint,
        },
        Intrinsic::Copy => Instruction::Copy,
        Intrinsic::DivS32 => Instruction::DivS32,
        Intrinsic::DivU8 => Instruction::DivU8,
        Intrinsic::Drop => Instruction::Drop,
        Intrinsic::Eq => Instruction::Eq,
        Intrinsic::Eval => Instruction::Eval { is_tail_call },
        Intrinsic::GreaterS8 => Instruction::GreaterS8,
        Intrinsic::GreaterS32 => Instruction::GreaterS32,
        Intrinsic::GreaterU8 => Instruction::GreaterU8,
        Intrinsic::MulS32 => Instruction::MulS32,
        Intrinsic::MulU8Wrap => Instruction::MulU8Wrap,
        Intrinsic::NegS32 => Instruction::NegS32,
        Intrinsic::Nop => Instruction::Nop,
        Intrinsic::Not => Instruction::LogicalNot,
        Intrinsic::RemainderS32 => Instruction::RemainderS32,
        Intrinsic::S32ToS8 => Instruction::ConvertS32ToS8,
        Intrinsic::SubS32 => Instruction::SubS32,
        Intrinsic::SubU8 => Instruction::SubU8,
        Intrinsic::SubU8Wrap => Instruction::SubU8Wrap,
    }
}

fn generate_instruction(
    instruction: Instruction,
    instructions: &mut Instructions,
    mapping: Option<&mut Mapping<'_>>,
) -> InstructionAddress {
    let addr = instructions.push(instruction);
    if let Some(mapping) = mapping {
        mapping.append_instruction(addr);
    }
    addr
}

struct Output<'r> {
    instructions: &'r mut Instructions,
    source_map: &'r mut SourceMap,
    placeholders: Vec<CallToFunction>,
}

impl Output<'_> {
    fn generate_binding<'r, N>(
        &mut self,
        names: N,
    ) -> Option<InstructionAddress>
    where
        N: IntoIterator<Item = &'r String>,
        N::IntoIter: DoubleEndedIterator,
    {
        let mut first_address = None;

        for name in names.into_iter().rev() {
            let address = generate_instruction(
                Instruction::Bind { name: name.clone() },
                self.instructions,
                None,
            );
            first_address = first_address.or(Some(address));
        }

        first_address
    }
}

pub struct CallToFunction {
    pub hash: Hash<Function>,
    pub address: InstructionAddress,
    pub is_tail_call: bool,
}

struct FunctionToCompile {
    function: Function,
    location: FunctionLocation,
    cluster: Cluster,
    address_of_instruction_to_make_anon_function: Option<InstructionAddress>,
}
