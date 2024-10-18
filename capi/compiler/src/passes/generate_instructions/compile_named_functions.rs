use std::collections::{BTreeMap, VecDeque};

use capi_runtime::{Instruction, Instructions};

use crate::{
    code::{
        CallGraph, Changes, Function, FunctionInUpdate,
        FunctionIndexInRootContext, FunctionLocation, FunctionUpdate,
        NamedFunctions,
    },
    compiler::CallInstructionsByCalleeHash,
    hash::Hash,
    source_map::SourceMap,
};

use super::compile_function::{
    compile_call_to_function, compile_function, CallToFunction,
    FunctionToCompile,
};

pub struct NamedFunctionsContext<'r> {
    pub named_functions: &'r NamedFunctions,
    pub instructions: &'r mut Instructions,
    pub source_map: &'r mut SourceMap,
    pub call_instructions_by_callee_hash: &'r mut CallInstructionsByCalleeHash,
    pub queue_of_functions_to_compile: VecDeque<FunctionToCompile>,

    /// # Track calls to recursive functions by hash of called function
    ///
    /// When a recursive call is encountered, not all branches of the callee
    /// (which might be the calling function itself, or another function in the
    /// same cluster) might be compiled yet. But they're needed to compile the
    /// call.
    ///
    /// So instead of compiling the call right then and there, a placeholder
    /// instruction is emitted instead. An entry is also added to this map, so
    /// the placeholder instruction can be replaced with the real call, once all
    /// functions have been compiled.
    pub recursive_function_calls_by_callee_hash:
        BTreeMap<Hash<Function>, Vec<CallToFunction>>,

    pub compiled_functions_by_hash:
        BTreeMap<Hash<Function>, capi_runtime::Function>,
}

pub fn compile_named_functions(
    named_functions: &NamedFunctions,
    changes: &Changes,
    call_graph: &CallGraph,
    instructions: &mut Instructions,
    source_map: &mut SourceMap,
    call_instructions_by_callee_hash: &mut CallInstructionsByCalleeHash,
) -> BTreeMap<Hash<Function>, capi_runtime::Function> {
    let mut queue_of_functions_to_compile = VecDeque::new();

    let named_functions_to_compile = gather_named_functions_to_compile(changes);
    seed_queue_of_functions_to_compile(
        &mut queue_of_functions_to_compile,
        named_functions_to_compile,
        call_graph,
    );

    let mut context = NamedFunctionsContext {
        named_functions,
        instructions,
        source_map,
        call_instructions_by_callee_hash,
        queue_of_functions_to_compile,
        recursive_function_calls_by_callee_hash: BTreeMap::new(),
        compiled_functions_by_hash: BTreeMap::new(),
    };

    while let Some(function_to_compile) =
        context.queue_of_functions_to_compile.pop_front()
    {
        let hash = Hash::new(&function_to_compile.function);
        let runtime_function =
            compile_function(function_to_compile, &mut context);
        context
            .compiled_functions_by_hash
            .insert(hash, runtime_function);
    }

    for (hash, calls) in &context.recursive_function_calls_by_callee_hash {
        for call in calls {
            compile_call_to_function(
                hash,
                call,
                &mut context.compiled_functions_by_hash,
                context.instructions,
            );
        }
    }

    for update in &changes.updated {
        let old_hash = Hash::new(&update.old.function);
        let new_hash = Hash::new(&update.new.function);

        for calling_address in context
            .call_instructions_by_callee_hash
            .inner
            .remove(&old_hash)
            .unwrap_or_default()
        {
            let calling_instruction = context
                .instructions
                .get(&calling_address)
                .expect("Instruction referenced from source map must exist.");
            let Instruction::CallFunction { is_tail_call, .. } =
                calling_instruction
            else {
                panic!(
                    "Calling instruction referenced from source map is not a \
                    function call."
                );
            };

            let function = context
                .compiled_functions_by_hash
                .get(&new_hash)
                .expect(
                "New function referenced in update should have been compiled; \
                is expected to exist.",
            );

            context.instructions.replace(
                &calling_address,
                Instruction::CallFunction {
                    function: function.clone(),
                    is_tail_call: *is_tail_call,
                },
            );
        }
    }

    context.compiled_functions_by_hash
}

fn gather_named_functions_to_compile(
    changes: &Changes,
) -> BTreeMap<&FunctionIndexInRootContext, &Function> {
    changes
        .added
        .iter()
        .chain(changes.updated.iter().map(
            |FunctionUpdate {
                 new: FunctionInUpdate { index, function },
                 ..
             }| (index, function),
        ))
        .collect::<BTreeMap<_, _>>()
}

fn seed_queue_of_functions_to_compile(
    queue_of_functions_to_compile: &mut VecDeque<FunctionToCompile>,
    mut named_functions_to_compile: BTreeMap<
        &FunctionIndexInRootContext,
        &Function,
    >,
    call_graph: &CallGraph,
) {
    queue_of_functions_to_compile.extend(
        call_graph
            .functions_from_leaves()
            .filter_map(|(&index, cluster)| {
                let function = named_functions_to_compile.remove(&index)?;
                Some(FunctionToCompile {
                    function: function.clone(),
                    location: FunctionLocation::NamedFunction { index },
                    cluster: cluster.clone(),
                    address_of_instruction_to_make_anon_function: None,
                })
            }),
    )
}
