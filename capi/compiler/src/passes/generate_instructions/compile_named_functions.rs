use std::collections::{BTreeMap, VecDeque};

use capi_runtime::{Instruction, InstructionAddress, Instructions};

use crate::{
    code::{Changes, Function, NamedFunctions},
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
    pub calls_by_function:
        &'r mut BTreeMap<Hash<Function>, Vec<InstructionAddress>>,
    pub queue_of_functions_to_compile: VecDeque<FunctionToCompile>,
    pub recursive_function_calls_by_callee_hash:
        BTreeMap<Hash<Function>, Vec<CallToFunction>>,
    pub compiled_functions_by_hash:
        BTreeMap<Hash<Function>, capi_runtime::Function>,
}

pub fn compile_named_functions(
    named_functions: &NamedFunctions,
    changes: &Changes,
    instructions: &mut Instructions,
    source_map: &mut SourceMap,
    calls_by_function: &mut BTreeMap<Hash<Function>, Vec<InstructionAddress>>,
    queue_of_functions_to_compile: VecDeque<FunctionToCompile>,
) -> BTreeMap<Hash<Function>, capi_runtime::Function> {
    let mut context = NamedFunctionsContext {
        named_functions,
        instructions,
        source_map,
        calls_by_function,
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
            .calls_by_function
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
