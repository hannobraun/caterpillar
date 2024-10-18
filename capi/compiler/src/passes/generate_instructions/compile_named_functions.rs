use std::collections::{BTreeMap, VecDeque};

use capi_runtime::{Instruction, Instructions};

use crate::{
    code::{CallGraph, Changes, Function, NamedFunctions},
    compiler::CallInstructionsByCalleeHash,
    hash::Hash,
    source_map::SourceMap,
};

use super::{
    compile_cluster::compile_cluster,
    compile_function::{CallToFunction, FunctionToCompile},
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
    let mut context = NamedFunctionsContext {
        named_functions,
        instructions,
        source_map,
        call_instructions_by_callee_hash,
        queue_of_functions_to_compile: VecDeque::new(),
        recursive_function_calls_by_callee_hash: BTreeMap::new(),
        compiled_functions_by_hash: BTreeMap::new(),
    };

    for cluster in call_graph.clusters_from_leaves() {
        compile_cluster(cluster, changes, &mut context);
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
