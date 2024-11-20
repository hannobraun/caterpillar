use std::collections::BTreeMap;

use capi_runtime::{Effect, Instruction, InstructionAddress};

use crate::{
    code::{
        Branch, BranchLocation, Cluster, Expression, ExpressionLocation,
        Function, FunctionLocation, Hash, IndexMap, Pattern,
    },
    intrinsics::IntrinsicFunction,
    source_map::Mapping,
    Instructions,
};

use super::{
    compile_cluster::ClusterContext, compile_functions::FunctionsContext,
};

pub fn compile_function(
    function: Function,
    location: FunctionLocation,
    address_of_instruction_to_make_anon_function: Option<InstructionAddress>,
    cluster: &Cluster,
    cluster_context: &mut ClusterContext,
    functions_context: &mut FunctionsContext,
) -> capi_runtime::Function {
    let mut runtime_function = capi_runtime::Function::default();
    let mut instruction_range = None;

    for (index, branch) in function.branches.into_iter() {
        let (runtime_branch, [first_address, last_address]) = compile_branch(
            branch,
            BranchLocation {
                parent: Box::new(location.clone()),
                index,
            },
            cluster,
            cluster_context,
            functions_context,
        );

        runtime_function.branches.push(runtime_branch);

        instruction_range = {
            let [first_in_function, _last_in_function] =
                instruction_range.unwrap_or([first_address, last_address]);

            Some([first_in_function, last_address])
        };
    }

    if let Some(instruction_range) = instruction_range {
        functions_context
            .source_map
            .map_function_to_instructions(location, instruction_range);
    }

    if let Some(address) = address_of_instruction_to_make_anon_function {
        functions_context.instructions.replace(
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
    branch: Branch,
    location: BranchLocation,
    cluster: &Cluster,
    cluster_context: &mut ClusterContext,
    functions_context: &mut FunctionsContext,
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
        compile_binding(parameters, functions_context.instructions);

    let [branch_address, last_address] = compile_branch_body(
        branch.body,
        location,
        cluster,
        cluster_context,
        functions_context,
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

fn compile_branch_body(
    body: IndexMap<Expression>,
    location: BranchLocation,
    cluster: &Cluster,
    cluster_context: &mut ClusterContext,
    functions_context: &mut FunctionsContext,
) -> [InstructionAddress; 2] {
    let mut first_instruction = None;

    for (index, expression) in body {
        let addr = compile_expression(
            expression,
            ExpressionLocation {
                parent: Box::new(location.clone()),
                index,
            },
            cluster,
            cluster_context,
            functions_context,
        );
        first_instruction = first_instruction.or(addr);
    }

    // Unconditionally generating a return instruction, like we do here, is
    // redundant. If the previous expression was a tail call, it didn't create a
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
        functions_context.instructions,
        None,
    );

    let first_instruction = first_instruction.unwrap_or(last_instruction);

    [first_instruction, last_instruction]
}

fn compile_expression(
    expression: Expression,
    location: ExpressionLocation,
    cluster: &Cluster,
    cluster_context: &mut ClusterContext,
    functions_context: &mut FunctionsContext,
) -> Option<InstructionAddress> {
    let mut mapping = functions_context
        .source_map
        .map_expression_to_instructions(location.clone());

    match expression {
        Expression::Binding { name, .. } => {
            let address = generate_instruction(
                Instruction::BindingEvaluate { name: name.clone() },
                functions_context.instructions,
                Some(&mut mapping),
            );
            Some(address)
        }
        Expression::CallToUserDefinedFunction { hash, is_tail_call } => {
            let Some(function) =
                functions_context.compiled_functions_by_hash.get(&hash)
            else {
                let function =
                    functions_context.functions.named.iter().find(|function| {
                        Hash::new(&function.fragment.inner) == hash
                    });

                panic!(
                    "Compiling call to user-defined function `{hash}`. \
                    Expecting functions to be compiled before any \
                    non-recursive calls to them, but can't find the compiled \
                    version of this one.\n\
                    \n\
                    Function:\n\
                    {function:#?}",
                )
            };

            let address = generate_instruction(
                Instruction::CallFunction {
                    function: function.clone(),
                    is_tail_call,
                },
                functions_context.instructions,
                Some(&mut mapping),
            );

            // For now, we're done with this call. But the function we're
            // calling might get updated in the future. When that happens, the
            // compiler wants to know about all calls to the function, to update
            // them.
            //
            // Let's make sure that information is going to be available.
            functions_context
                .call_instructions_by_callee
                .inner
                .entry(hash)
                .or_default()
                .push(address);

            Some(address)
        }
        Expression::CallToUserDefinedFunctionRecursive {
            index,
            is_tail_call,
        } => {
            let hash = {
                let function_location = &cluster.functions[&index];

                let called_function = functions_context
                    .functions
                    .by_location(function_location)
                    .expect("Function referred to from cluster must exist.");

                Hash::new(called_function.fragment)
            };

            // For recursive calls, we can't generally assume that the called
            // function has been compiled yet. It's a recursive call, after all!
            //
            // Let's emit a placeholder instruction and arrange for that to be
            // replaced later, once all of the functions in the cluster have
            // been compiled.
            let address = generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::CompilerBug,
                },
                functions_context.instructions,
                Some(&mut mapping),
            );
            cluster_context
                .recursive_calls_by_callee
                .entry(hash)
                .or_default()
                .push(CallToFunction {
                    address,
                    is_tail_call,
                });

            // For now, we're done with this call. But the function we're
            // calling might get updated in the future. When that happens, the
            // compiler wants to know about all calls to the function, to update
            // them.
            //
            // Let's make sure that information is going to be available.
            functions_context
                .call_instructions_by_callee
                .inner
                .entry(hash)
                .or_default()
                .push(address);

            Some(address)
        }
        Expression::CallToHostFunction { number } => {
            let address = generate_instruction(
                Instruction::Push {
                    value: number.into(),
                },
                functions_context.instructions,
                Some(&mut mapping),
            );
            generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::Host,
                },
                functions_context.instructions,
                Some(&mut mapping),
            );
            Some(address)
        }
        Expression::CallToIntrinsicFunction {
            intrinsic,
            is_tail_call: _,
        } => {
            let is_tail_call = functions_context
                .tail_expressions
                .is_tail_expression(&location);

            let address = compile_intrinsic(
                intrinsic,
                is_tail_call,
                functions_context.instructions,
                &mut mapping,
            );
            Some(address)
        }
        Expression::Comment { .. } => None,
        Expression::LiteralNumber { value } => {
            let address = generate_instruction(
                Instruction::Push { value },
                functions_context.instructions,
                Some(&mut mapping),
            );
            Some(address)
        }
        Expression::LocalFunction { hash } => {
            let function = functions_context.functions.by_hash(&hash).expect(
                "Anonymous function that has been previously resolved must be \
                available.",
            );

            let address = compile_local_function(
                &function,
                location,
                cluster_context,
                functions_context.instructions,
                &mut mapping,
            );
            Some(address)
        }
        Expression::LocalFunctionRecursive { index } => {
            let function = {
                let location = cluster.functions.get(&index).expect(
                    "Resolved local recursive function must exist in cluster.",
                );
                functions_context
                    .functions
                    .by_location(location)
                    .expect("Function referenced from cluster must exist.")
            };

            let address = compile_local_function(
                &function,
                location,
                cluster_context,
                functions_context.instructions,
                &mut mapping,
            );
            Some(address)
        }
        Expression::UnresolvedIdentifier { .. } => {
            let address = generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::BuildError,
                },
                functions_context.instructions,
                Some(&mut mapping),
            );
            Some(address)
        }
        Expression::UnresolvedLocalFunction => {
            let address = generate_instruction(
                Instruction::TriggerEffect {
                    effect: Effect::BuildError,
                },
                functions_context.instructions,
                Some(&mut mapping),
            );
            Some(address)
        }
    }
}

pub fn compile_call_to_function(
    hash: &Hash<Function>,
    call: CallToFunction,
    functions: &mut BTreeMap<Hash<Function>, capi_runtime::Function>,
    instructions: &mut Instructions,
) {
    let function = functions.get(hash).expect(
        "Attempting to compile call to function. Expecting that function to \
        have been compiled already.",
    );

    instructions.replace(
        &call.address,
        Instruction::CallFunction {
            function: function.clone(),
            is_tail_call: call.is_tail_call,
        },
    );
}

fn compile_intrinsic(
    intrinsic: IntrinsicFunction,
    is_tail_call: bool,
    instructions: &mut Instructions,
    mapping: &mut Mapping,
) -> InstructionAddress {
    let instruction = match intrinsic {
        IntrinsicFunction::AddS8 => Instruction::AddS8,
        IntrinsicFunction::AddS32 => Instruction::AddS32,
        IntrinsicFunction::AddU8 => Instruction::AddU8,
        IntrinsicFunction::AddU8Wrap => Instruction::AddU8Wrap,
        IntrinsicFunction::And => Instruction::LogicalAnd,
        IntrinsicFunction::Brk => Instruction::TriggerEffect {
            effect: Effect::Breakpoint,
        },
        IntrinsicFunction::Copy => Instruction::Copy { offset: 0 },
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
    };

    generate_instruction(instruction, instructions, Some(mapping))
}

fn compile_local_function(
    function: &Function,
    location: ExpressionLocation,
    cluster_context: &mut ClusterContext,
    instructions: &mut Instructions,
    mapping: &mut Mapping,
) -> InstructionAddress {
    // We have encountered an anonymous function. We need to emit an instruction
    // that allocates it, and takes care of its environment.
    //
    // But we haven't compiled the anonymous function yet, and we can't do that
    // right now. If we did, we would be emitting its instructions in the middle
    // of whatever function (anonymous or named) that we're currently compiling.
    //
    // The result of that would be, that every anonymous function would be
    // executed right where it's defined, which would defeat the purpose of
    // having them in the first place.
    //
    // But we still somehow need to emit that instruction to allocate the
    // anonymous function and take care of its environment. We'll do that later,
    // after we've actually compiled the anonymous function.
    //
    // For now, we'll just emit a placeholder that can be replaced with the real
    // instruction then.
    let address = generate_instruction(
        Instruction::TriggerEffect {
            effect: Effect::CompilerBug,
        },
        instructions,
        Some(mapping),
    );

    // We've done what we could. Let's arrange for the anonymous function to be
    // compiled, and the placeholder instruction to be replaced, at a later
    // time.
    cluster_context.queue_of_functions_to_compile.push_front(
        FunctionToCompile {
            function: function.clone(),
            location: FunctionLocation::AnonymousFunction { location },
            address_of_instruction_to_make_anon_function: Some(address),
        },
    );

    address
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
    pub address_of_instruction_to_make_anon_function:
        Option<InstructionAddress>,
}
