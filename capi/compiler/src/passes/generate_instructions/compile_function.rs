use std::collections::BTreeMap;

use capi_runtime::{Effect, Instruction, InstructionAddress, Instructions};

use crate::{
    code::{
        Branch, BranchLocation, Cluster, Fragment, FragmentLocation, Function,
        FunctionLocation, Pattern,
    },
    hash::Hash,
    intrinsics::IntrinsicFunction,
    source_map::Mapping,
};

use super::compile_named_functions::NamedFunctionsContext;

pub struct FunctionContext<'r> {
    pub cluster: &'r Cluster,
    pub recursive_function_calls_by_callee_hash:
        BTreeMap<Hash<Function>, Vec<CallToFunction>>,
}

pub fn compile_function(
    function_to_compile: FunctionToCompile,
    named_functions_context: &mut NamedFunctionsContext,
) -> capi_runtime::Function {
    let FunctionToCompile {
        function,
        location,
        cluster,
        address_of_instruction_to_make_anon_function,
    } = function_to_compile;

    let mut context = FunctionContext {
        cluster: &cluster,
        recursive_function_calls_by_callee_hash: BTreeMap::new(),
    };

    let mut runtime_function = capi_runtime::Function::default();
    let mut instruction_range = None;

    for (&index, branch) in function.branches.iter() {
        let (runtime_branch, [first_address, last_address]) = compile_branch(
            branch,
            BranchLocation {
                parent: Box::new(location.clone()),
                index,
            },
            &mut context,
            named_functions_context,
        );

        runtime_function.branches.push(runtime_branch);

        instruction_range = {
            let [first_in_function, _last_in_function] =
                instruction_range.unwrap_or([first_address, last_address]);

            Some([first_in_function, last_address])
        };
    }

    if let Some(instruction_range) = instruction_range {
        named_functions_context
            .source_map
            .map_function_to_instructions(location, instruction_range);
    }

    if let Some(address) = address_of_instruction_to_make_anon_function {
        named_functions_context.instructions.replace(
            &address,
            Instruction::MakeAnonymousFunction {
                branches: runtime_function.branches.clone(),
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

    runtime_function
}

fn compile_branch(
    branch: &Branch,
    location: BranchLocation,
    function_context: &mut FunctionContext,
    named_functions_context: &mut NamedFunctionsContext,
) -> (capi_runtime::Branch, [InstructionAddress; 2]) {
    let parameters = branch.parameters.iter().filter_map(|pattern| {
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
        compile_binding(parameters, named_functions_context.instructions);

    let [branch_address, last_address] = compile_branch_body(
        branch,
        location,
        function_context,
        named_functions_context,
    );

    let first_address = bindings_address.unwrap_or(branch_address);

    let branch = capi_runtime::Branch {
        parameters: branch
            .parameters
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
    };

    (branch, [first_address, last_address])
}

fn compile_branch_body(
    branch: &Branch,
    location: BranchLocation,
    function_context: &mut FunctionContext,
    named_functions_context: &mut NamedFunctionsContext,
) -> [InstructionAddress; 2] {
    let mut first_instruction = None;

    for (&index, fragment) in &branch.body {
        let addr = compile_fragment(
            fragment,
            FragmentLocation {
                parent: Box::new(location.clone()),
                index,
            },
            function_context,
            named_functions_context,
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
    let last_instruction = generate_instruction(
        Instruction::Return,
        named_functions_context.instructions,
        None,
    );

    let first_instruction = first_instruction.unwrap_or(last_instruction);

    [first_instruction, last_instruction]
}

#[allow(clippy::too_many_arguments)]
fn compile_fragment(
    fragment: &Fragment,
    location: FragmentLocation,
    function_context: &mut FunctionContext,
    named_functions_context: &mut NamedFunctionsContext,
) -> Option<InstructionAddress> {
    match &fragment {
        Fragment::CallToUserDefinedFunction {
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
                named_functions_context.instructions,
                Some(
                    &mut named_functions_context
                        .source_map
                        .map_fragment_to_instructions(location),
                ),
            );

            compile_call_to_function(
                hash,
                &CallToFunction {
                    address,
                    is_tail_call: *is_tail_call,
                },
                &mut named_functions_context.compiled_functions_by_hash,
                named_functions_context.instructions,
            );

            // We also need to do some bookkeeping, so we can update the call,
            // in case the called function is updated.
            named_functions_context
                .call_instructions_by_callee_hash
                .entry(*hash)
                .or_default()
                .push(address);

            Some(address)
        }
        Fragment::CallToUserDefinedFunctionRecursive {
            index,
            is_tail_call,
        } => {
            let function_index_in_root_context =
                function_context.cluster.functions[index];
            let called_function = named_functions_context
                .named_functions
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
                named_functions_context.instructions,
                Some(
                    &mut named_functions_context
                        .source_map
                        .map_fragment_to_instructions(location),
                ),
            );

            // We can't leave it at that, however. We need to make sure this
            // placeholder actually gets replaced later, and we're doing that by
            // adding it to this list.
            function_context
                .recursive_function_calls_by_callee_hash
                .entry(hash)
                .or_default()
                .push(CallToFunction {
                    address,
                    is_tail_call: *is_tail_call,
                });
            named_functions_context
                .recursive_function_calls_by_callee_hash
                .entry(hash)
                .or_default()
                .push(CallToFunction {
                    address,
                    is_tail_call: *is_tail_call,
                });

            // We also need to do some bookkeeping, so we can update the call,
            // in case the called function is updated.
            named_functions_context
                .call_instructions_by_callee_hash
                .entry(hash)
                .or_default()
                .push(address);

            Some(address)
        }
        Fragment::CallToHostFunction { effect_number } => {
            let mut mapping = named_functions_context
                .source_map
                .map_fragment_to_instructions(location);

            let address = generate_instruction(
                Instruction::Push {
                    value: (*effect_number).into(),
                },
                named_functions_context.instructions,
                Some(&mut mapping),
            );
            generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::Host,
                },
                named_functions_context.instructions,
                Some(&mut mapping),
            );
            Some(address)
        }
        Fragment::CallToIntrinsicFunction {
            intrinsic,
            is_tail_call,
        } => {
            let instruction =
                intrinsic_to_instruction(intrinsic, *is_tail_call);

            Some(generate_instruction(
                instruction,
                named_functions_context.instructions,
                Some(
                    &mut named_functions_context
                        .source_map
                        .map_fragment_to_instructions(location),
                ),
            ))
        }
        Fragment::Comment { .. } => None,
        Fragment::Function { function } => {
            assert!(
                function.name.is_none(),
                "An anonymous function should not have a name."
            );

            // We have encountered an anonymous function. We need to emit an
            // instruction that allocates it, and takes care of its environment.
            //
            // But we haven't compiled the anonymous function yet, and we can't
            // do that right now. If we did, we would be emitting its
            // instructions in the middle of whatever function (anonymous or
            // named) that we're currently compiling.
            //
            // The result of that would be, that every anonymous function would
            // be executed right where it's defined, which would defeat the
            // purpose of having them in the first place.
            //
            // But we still somehow need to emit that instruction to allocate
            // the anonymous function and take care of its environment. We'll do
            // that later, after we've actually compiled the anonymous function.
            //
            // For now, we'll just emit a placeholder that can be replaced with
            // the real instruction then.
            let address_of_instruction_to_make_anon_function =
                Some(generate_instruction(
                    Instruction::TriggerEffect {
                        effect: Effect::CompilerBug,
                    },
                    named_functions_context.instructions,
                    Some(
                        &mut named_functions_context
                            .source_map
                            .map_fragment_to_instructions(location.clone()),
                    ),
                ));

            // We've done what we could. Let's arrange for the anonymous
            // function to be compiled, and the placeholder instruction to be
            // replaced, at a later time.
            named_functions_context
                .queue_of_functions_to_compile
                .push_front(FunctionToCompile {
                    function: function.clone(),
                    location: FunctionLocation::AnonymousFunction { location },
                    cluster: function_context.cluster.clone(),
                    address_of_instruction_to_make_anon_function,
                });

            address_of_instruction_to_make_anon_function
        }
        Fragment::ResolvedBinding { name } => Some(generate_instruction(
            Instruction::BindingEvaluate { name: name.clone() },
            named_functions_context.instructions,
            Some(
                &mut named_functions_context
                    .source_map
                    .map_fragment_to_instructions(location),
            ),
        )),
        Fragment::UnresolvedIdentifier { .. } => Some(generate_instruction(
            Instruction::TriggerEffect {
                effect: Effect::BuildError,
            },
            named_functions_context.instructions,
            Some(
                &mut named_functions_context
                    .source_map
                    .map_fragment_to_instructions(location),
            ),
        )),
        Fragment::Value(value) => Some(generate_instruction(
            Instruction::Push { value: *value },
            named_functions_context.instructions,
            Some(
                &mut named_functions_context
                    .source_map
                    .map_fragment_to_instructions(location),
            ),
        )),
    }
}

pub fn compile_call_to_function(
    hash: &Hash<Function>,
    call: &CallToFunction,
    functions: &mut BTreeMap<Hash<Function>, capi_runtime::Function>,
    instructions: &mut Instructions,
) {
    let Some(function) = functions.get(hash) else {
        // This won't happen for any regular function, because we only create
        // placeholders for functions that we actually encounter. But it can
        // happen for the `main` function, since we create a placeholder for
        // that unconditionally.
        //
        // If that happens, let's just leave the placeholder panic. It's not
        // great, as it doesn't provide any context to the user. But while we
        // don't have any way to make panics more descriptive, it'll have to do.
        return;
    };

    instructions.replace(
        &call.address,
        Instruction::CallFunction {
            function: function.clone(),
            is_tail_call: call.is_tail_call,
        },
    );
}

fn intrinsic_to_instruction(
    intrinsic: &IntrinsicFunction,
    is_tail_call: bool,
) -> Instruction {
    match intrinsic {
        IntrinsicFunction::AddS8 => Instruction::AddS8,
        IntrinsicFunction::AddS32 => Instruction::AddS32,
        IntrinsicFunction::AddU8 => Instruction::AddU8,
        IntrinsicFunction::AddU8Wrap => Instruction::AddU8Wrap,
        IntrinsicFunction::And => Instruction::LogicalAnd,
        IntrinsicFunction::Brk => Instruction::TriggerEffect {
            effect: Effect::Breakpoint,
        },
        IntrinsicFunction::Copy => Instruction::Copy,
        IntrinsicFunction::DivS32 => Instruction::DivS32,
        IntrinsicFunction::DivU8 => Instruction::DivU8,
        IntrinsicFunction::Drop => Instruction::Drop,
        IntrinsicFunction::Eq => Instruction::Eq,
        IntrinsicFunction::Eval => Instruction::Eval { is_tail_call },
        IntrinsicFunction::GreaterS8 => Instruction::GreaterS8,
        IntrinsicFunction::GreaterS32 => Instruction::GreaterS32,
        IntrinsicFunction::GreaterU8 => Instruction::GreaterU8,
        IntrinsicFunction::MulS32 => Instruction::MulS32,
        IntrinsicFunction::MulU8Wrap => Instruction::MulU8Wrap,
        IntrinsicFunction::NegS32 => Instruction::NegS32,
        IntrinsicFunction::Nop => Instruction::Nop,
        IntrinsicFunction::Not => Instruction::LogicalNot,
        IntrinsicFunction::RemainderS32 => Instruction::RemainderS32,
        IntrinsicFunction::S32ToS8 => Instruction::ConvertS32ToS8,
        IntrinsicFunction::SubS32 => Instruction::SubS32,
        IntrinsicFunction::SubU8 => Instruction::SubU8,
        IntrinsicFunction::SubU8Wrap => Instruction::SubU8Wrap,
    }
}

fn compile_binding<'r, N>(
    names: N,
    instructions: &mut Instructions,
) -> Option<InstructionAddress>
where
    N: IntoIterator<Item = &'r String>,
    N::IntoIter: DoubleEndedIterator,
{
    let mut first_address = None;

    for name in names.into_iter().rev() {
        let address = generate_instruction(
            Instruction::Bind { name: name.clone() },
            instructions,
            None,
        );
        first_address = first_address.or(Some(address));
    }

    first_address
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

pub struct CallToFunction {
    pub address: InstructionAddress,
    pub is_tail_call: bool,
}

pub struct FunctionToCompile {
    pub function: Function,
    pub location: FunctionLocation,
    pub cluster: Cluster,
    pub address_of_instruction_to_make_anon_function:
        Option<InstructionAddress>,
}
