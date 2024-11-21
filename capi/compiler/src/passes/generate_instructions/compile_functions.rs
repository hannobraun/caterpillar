use std::collections::BTreeMap;

use capi_runtime::Instruction;

use crate::{
    code::{
        Bindings, Changes, Function, FunctionCalls, Hash, OrderedFunctions,
        Recursion, StableFunctions, TailExpressions,
    },
    compiler::CallInstructionsByCallee,
    source_map::SourceMap,
    Instructions,
};

use super::compile_cluster::compile_cluster;

pub struct FunctionsContext<'r> {
    pub functions: &'r StableFunctions,
    pub bindings: &'r Bindings,
    pub function_calls: &'r FunctionCalls,
    pub tail_expressions: &'r TailExpressions,
    pub recursion: &'r Recursion,
    pub instructions: &'r mut Instructions,
    pub source_map: &'r mut SourceMap,
    pub call_instructions_by_callee: &'r mut CallInstructionsByCallee,
    pub compiled_functions_by_hash:
        &'r mut BTreeMap<Hash<Function>, capi_runtime::Function>,
}

#[allow(clippy::too_many_arguments)]
pub fn compile_functions(
    functions: &StableFunctions,
    changes: &Changes,
    ordered_functions: &OrderedFunctions,
    bindings: &Bindings,
    function_calls: &FunctionCalls,
    tail_expressions: &TailExpressions,
    recursion: &Recursion,
    instructions: &mut Instructions,
    source_map: &mut SourceMap,
    call_instructions_by_callee: &mut CallInstructionsByCallee,
    compiled_functions_by_hash: &mut BTreeMap<
        Hash<Function>,
        capi_runtime::Function,
    >,
) {
    let mut context = FunctionsContext {
        functions,
        bindings,
        function_calls,
        tail_expressions,
        recursion,
        instructions,
        source_map,
        call_instructions_by_callee,
        compiled_functions_by_hash,
    };

    for cluster in ordered_functions.clusters_from_leaves() {
        compile_cluster(cluster, changes, &mut context);
    }

    for update in &changes.updated {
        let old_hash = Hash::new(&update.old.function);
        let new_hash = Hash::new(&update.new.function);

        for calling_address in context
            .call_instructions_by_callee
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

            let function =
                context.compiled_functions_by_hash.get(&new_hash).expect(
                    "New function referenced in update should have been \
                    compiled; is expected to exist.",
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
}
