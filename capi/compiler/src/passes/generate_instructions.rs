use std::{
    collections::{BTreeMap, VecDeque},
    rc::Rc,
};

use capi_runtime::{Effect, Instruction, InstructionAddress, Instructions};

use crate::{
    fragments::{
        Branch, BranchLocation, Cluster, Fragment, FragmentId,
        FragmentLocation, FragmentMap, Fragments, Function, FunctionLocation,
        Parameters,
    },
    intrinsics::Intrinsic,
    source_map::SourceMap,
    syntax::Pattern,
};

pub fn generate_instructions(
    fragments: Fragments,
) -> (Instructions, SourceMap) {
    let mut queue = VecDeque::new();
    let mut output = Output::default();
    let mut functions = Functions::default();

    // Create placeholder for call to `main` function, and the last return that
    // ends the process, if executed.
    //
    // If there's no `main` function, this won't get replaced. Since this is a
    // result of wrong code, an instruction generating the `BuildError` effect
    // is an appropriate placeholder.
    let call_to_main = output.instructions.push(Instruction::TriggerEffect {
        effect: Effect::BuildError,
    });
    output.instructions.push(Instruction::Return);
    if let Some(function) = fragments.find_function_by_name("main") {
        output.placeholders.push(CallToFunction {
            function: function.id,
            address: call_to_main,
            is_tail_call: true,
        });
    }

    // Seed the queue from the root context.
    for ((&index, function), (id, _)) in fragments
        .functions
        .iter()
        .zip(fragments.iter_from(fragments.root))
    {
        queue.push_front(FunctionToCompile {
            fragment: id,
            function: function.clone(),
            location: Rc::new(FunctionLocation::NamedFunction { index }),
            address_of_instruction_to_make_anon_function: None,
        });
    }

    while let Some(function_to_compile) = queue.pop_front() {
        compile_function(
            function_to_compile,
            &fragments,
            &mut output,
            &mut queue,
            &mut functions,
        );
    }

    for call in output.placeholders {
        let Some(function) = functions.by_fragment.get(&call.function) else {
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

    (output.instructions, output.source_map)
}

fn compile_function(
    function_to_compile: FunctionToCompile,
    fragments: &Fragments,
    output: &mut Output,
    queue: &mut VecDeque<FunctionToCompile>,
    functions: &mut Functions,
) {
    let FunctionToCompile {
        fragment,
        function,
        location,
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
        let bindings_address = output.generate_binding(parameters, fragment);

        let [branch_address, last_address] = compile_branch(
            branch,
            Rc::new(BranchLocation {
                parent: location.clone(),
                index,
            }),
            &fragments.clusters,
            &fragments.map,
            output,
            queue,
        );

        let first_address = bindings_address.unwrap_or(branch_address);
        functions
            .by_fragment
            .entry(fragment)
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
        output.source_map.define_instruction_range(
            function.clone(),
            fragment,
            instruction_range,
        );
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
    location: Rc<BranchLocation>,
    clusters: &[Cluster],
    fragments: &FragmentMap,
    output: &mut Output,
    queue: &mut VecDeque<FunctionToCompile>,
) -> [InstructionAddress; 2] {
    let mut first_instruction = None;

    for ((_index, fragment), (id, _)) in
        branch.body.iter().zip(fragments.iter_from(branch.start))
    {
        let addr = compile_fragment(
            id,
            fragment,
            FragmentLocation {
                parent: location.clone(),
                index: *_index,
            },
            clusters,
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
        output.generate_instruction(Instruction::Return, None);

    let first_instruction = first_instruction.unwrap_or(last_instruction);

    [first_instruction, last_instruction]
}

fn compile_fragment(
    id: FragmentId,
    fragment: &Fragment,
    location: FragmentLocation,
    clusters: &[Cluster],
    fragments: &FragmentMap,
    output: &mut Output,
    queue: &mut VecDeque<FunctionToCompile>,
) -> Option<InstructionAddress> {
    match &fragment {
        Fragment::CallToFunction {
            name, is_tail_call, ..
        } => {
            // We know that this expression refers to a user-defined function,
            // but we might not have compiled that function yet.
            //
            // For now, just generate a placeholder that we can replace with the
            // call later.
            let address = output.generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::CompilerBug,
                },
                Some(id),
            );

            // We can't leave it at that, however. We need to make sure this
            // placeholder actually gets replaced later, and we're doing that by
            // adding it to this list.
            if let Some(function) = fragments.find_function_by_name(name) {
                output.placeholders.push(CallToFunction {
                    function: function.id,
                    address,
                    is_tail_call: *is_tail_call,
                });
            }

            Some(address)
        }
        Fragment::CallToFunctionRecursive {
            index,
            is_tail_call,
        } => {
            let (calling_function, _) = fragments
                .find_named_function_by_fragment_in_body(&id)
                .expect(
                    "Fragment ID is of a function call; must be located in a \
                    function.",
                );
            let cluster = clusters
                .iter()
                .find(|cluster| {
                    cluster
                        .functions
                        .values()
                        .any(|id| *id == calling_function.id)
                })
                .expect("Expecting every function to be in a cluster.");
            let called_function = cluster.functions[index];

            // We know that this expression refers to a user-defined function,
            // but we might not have compiled that function yet.
            //
            // For now, just generate a placeholder that we can replace with the
            // call later.
            let address = output.generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::CompilerBug,
                },
                Some(id),
            );

            // We can't leave it at that, however. We need to make sure this
            // placeholder actually gets replaced later, and we're doing that by
            // adding it to this list.
            output.placeholders.push(CallToFunction {
                function: called_function,
                address,
                is_tail_call: *is_tail_call,
            });

            Some(address)
        }
        Fragment::CallToHostFunction { effect_number } => {
            let address = output.generate_instruction(
                Instruction::Push {
                    value: (*effect_number).into(),
                },
                Some(id),
            );
            output.generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::Host,
                },
                Some(id),
            );
            Some(address)
        }
        Fragment::CallToIntrinsic {
            intrinsic,
            is_tail_call,
        } => {
            let instruction =
                intrinsic_to_instruction(intrinsic, *is_tail_call);

            Some(output.generate_instruction(instruction, Some(id)))
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
                    Some(output.generate_instruction(
                        Instruction::TriggerEffect {
                            effect: Effect::CompilerBug,
                        },
                        Some(id),
                    ))
                } else {
                    None
                };

            // And to make it happen later, we need to put what we already have
            // into a queue. Once whatever's currently being compiled is out of
            // the way, we can process that.
            queue.push_front(FunctionToCompile {
                fragment: id,
                function: function.clone(),
                location: Rc::new(FunctionLocation::AnonymousFunction {
                    location,
                }),
                address_of_instruction_to_make_anon_function,
            });

            address_of_instruction_to_make_anon_function
        }
        Fragment::ResolvedBinding { name } => {
            Some(output.generate_instruction(
                Instruction::BindingEvaluate { name: name.clone() },
                Some(id),
            ))
        }
        Fragment::UnresolvedIdentifier { name: _ } => {
            Some(output.generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::BuildError,
                },
                Some(id),
            ))
        }
        Fragment::Value(value) => Some(output.generate_instruction(
            Instruction::Push { value: *value },
            Some(id),
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
        fragment: Option<FragmentId>,
    ) -> InstructionAddress {
        let addr = self.instructions.push(instruction);
        if let Some(fragment) = fragment {
            self.source_map.define_mapping(addr, fragment);
        }
        addr
    }

    fn generate_binding<'r, N>(
        &mut self,
        names: N,
        fragment: FragmentId,
    ) -> Option<InstructionAddress>
    where
        N: IntoIterator<Item = &'r String>,
        N::IntoIter: DoubleEndedIterator,
    {
        let mut first_address = None;

        for name in names.into_iter().rev() {
            let address = self.generate_instruction(
                Instruction::Bind { name: name.clone() },
                Some(fragment),
            );
            first_address = first_address.or(Some(address));
        }

        first_address
    }
}

pub struct CallToFunction {
    pub function: FragmentId,
    pub address: InstructionAddress,
    pub is_tail_call: bool,
}

#[derive(Default)]
struct Functions {
    by_fragment: BTreeMap<FragmentId, Vec<(Parameters, InstructionAddress)>>,
}

struct FunctionToCompile {
    fragment: FragmentId,
    function: Function,
    location: Rc<FunctionLocation>,
    address_of_instruction_to_make_anon_function: Option<InstructionAddress>,
}
