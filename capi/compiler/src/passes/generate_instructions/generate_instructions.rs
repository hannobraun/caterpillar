use std::collections::BTreeMap;

use capi_runtime::{Effect, Instruction, InstructionAddress};

use crate::{
    code::{
        Changes, Function, Functions, Hash, OrderedFunctions, StableFunctions,
        TailExpressions,
    },
    compiler::CallInstructionsByCallee,
    source_map::SourceMap,
    Instructions,
};

use super::{
    compile_function::{compile_call_to_function, CallToFunction},
    compile_functions::compile_functions,
};

#[allow(clippy::too_many_arguments)]
pub fn generate_instructions(
    functions: &StableFunctions,
    ordered_functions: &OrderedFunctions,
    tail_expressions: &TailExpressions,
    changes: &Changes,
    instructions: &mut Instructions,
    call_instructions_by_callee: &mut CallInstructionsByCallee,
    compiled_functions_by_hash: &mut BTreeMap<
        Hash<Function>,
        capi_runtime::Function,
    >,
    source_map: &mut SourceMap,
) {
    // The placeholder call into `main` is created unconditionally, regardless
    // of whether this is a fresh build and we actually need to do that, or if
    // we already have an active runtime and are just compiling changes.
    //
    // I don't think this has any adverse effects, except creating junk
    // instructions that increase the code size. And I don't want to fix that,
    // until we have infrastructure in place that would measure the code size
    // and actually show the impact of those changes.
    //
    // Otherwise, we'll just complicate the code with unclear benefit, and no
    // means to track whether simplifications are beneficial or not.
    let call_to_main = create_placeholder_for_call_to_main(instructions);

    compile_functions(
        functions,
        changes,
        ordered_functions,
        tail_expressions,
        instructions,
        source_map,
        call_instructions_by_callee,
        compiled_functions_by_hash,
    );
    compile_call_to_main(
        call_to_main,
        functions,
        instructions,
        compiled_functions_by_hash,
    );
}

fn create_placeholder_for_call_to_main(
    instructions: &mut Instructions,
) -> InstructionAddress {
    // If there's no `main` function, this instruction won't get replaced later.
    // That would be a result of invalid code (valid code would provide a `main`
    // function), so an instruction generating the `BuildError` effect is an
    // appropriate placeholder.
    instructions.push(Instruction::TriggerEffect {
        effect: Effect::BuildError,
    })
}

fn compile_call_to_main(
    call_to_main: InstructionAddress,
    functions: &Functions,
    instructions: &mut Instructions,
    compiled_functions_by_hash: &mut BTreeMap<
        Hash<Function>,
        capi_runtime::Function,
    >,
) {
    let Some(main) = functions.named.by_name("main") else {
        // If we can't find the call to `main`, that is a result of invalid
        // code. Leaving the placeholder instruction is appropriate in that
        // case.
        return;
    };

    compile_call_to_function(
        &Hash::new(&main.inner),
        CallToFunction {
            address: call_to_main,
            is_tail_call: true,
        },
        compiled_functions_by_hash,
        instructions,
    );
}
