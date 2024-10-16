use std::collections::{BTreeMap, VecDeque};

use capi_runtime::{Instruction, InstructionAddress, Instructions};

use crate::{
    code::{Changes, Function, NamedFunctions, Pattern},
    hash::Hash,
    source_map::SourceMap,
};

use super::compile_function::{
    compile_call_to_function, compile_function, CallToFunction,
    FunctionToCompile,
};

pub struct Context<'r> {
    pub named_functions: &'r NamedFunctions,
    pub instructions: &'r mut Instructions,
    pub source_map: &'r mut SourceMap,
    pub calls_by_function:
        &'r mut BTreeMap<Hash<Function>, Vec<InstructionAddress>>,
    pub queue_of_functions_to_compile: VecDeque<FunctionToCompile>,
    pub placeholders: BTreeMap<Hash<Function>, Vec<CallToFunction>>,
    pub functions:
        BTreeMap<Hash<Function>, Vec<(Vec<Pattern>, InstructionAddress)>>,
}

pub fn compile_named_functions(
    named_functions: &NamedFunctions,
    changes: &Changes,
    instructions: &mut Instructions,
    source_map: &mut SourceMap,
    calls_by_function: &mut BTreeMap<Hash<Function>, Vec<InstructionAddress>>,
    queue_of_functions_to_compile: VecDeque<FunctionToCompile>,
) -> BTreeMap<Hash<Function>, Vec<(Vec<Pattern>, InstructionAddress)>> {
    let mut context = Context {
        named_functions,
        instructions,
        source_map,
        calls_by_function,
        queue_of_functions_to_compile,
        placeholders: BTreeMap::new(),
        functions: BTreeMap::new(),
    };

    while let Some(function_to_compile) =
        context.queue_of_functions_to_compile.pop_front()
    {
        compile_function(function_to_compile, &mut context);
    }

    for (hash, calls) in &context.placeholders {
        for call in calls {
            compile_call_to_function(
                hash,
                call,
                &mut context.functions,
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

            let function = context.functions.get(&new_hash).expect(
                "New function referenced in update should have been compiled; \
                is expected to exist.",
            );
            let function = capi_runtime::Function {
                branches: function
                    .iter()
                    .map(|(parameters, address)| {
                        let parameters = parameters
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

            context.instructions.replace(
                &calling_address,
                Instruction::CallFunction {
                    function,
                    is_tail_call: *is_tail_call,
                },
            );
        }
    }

    context.functions
}
