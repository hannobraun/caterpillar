use std::collections::{BTreeMap, VecDeque};

use capi_runtime::{Effect, Instruction, InstructionAddress, Instructions};

use crate::{
    code::{
        CallGraph, Changes, Function, FunctionInUpdate,
        FunctionIndexInRootContext, FunctionLocation, FunctionUpdate,
        NamedFunctions, Pattern,
    },
    hash::Hash,
    source_map::SourceMap,
};

use super::compile_named_functions::{
    compile_call_to_function, compile_named_functions, CallToFunction,
    FunctionToCompile,
};

pub fn generate_instructions(
    named_functions: &NamedFunctions,
    call_graph: &CallGraph,
    changes: &Changes,
    instructions: &mut Instructions,
    calls_by_function: &mut BTreeMap<Hash<Function>, Vec<InstructionAddress>>,
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

    let named_functions_to_compile = gather_named_functions_to_compile(changes);
    let queue_of_functions_to_compile = seed_queue_of_functions_to_compile(
        named_functions_to_compile,
        call_graph,
    );
    let mut functions = compile_named_functions(
        named_functions,
        changes,
        instructions,
        source_map,
        calls_by_function,
        queue_of_functions_to_compile,
    );
    compile_call_to_main(
        call_to_main,
        named_functions,
        instructions,
        &mut functions,
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
    mut named_functions_to_compile: BTreeMap<
        &FunctionIndexInRootContext,
        &Function,
    >,
    call_graph: &CallGraph,
) -> VecDeque<FunctionToCompile> {
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
        })
        .collect::<VecDeque<_>>()
}

fn compile_call_to_main(
    call_to_main: InstructionAddress,
    named_functions: &NamedFunctions,
    instructions: &mut Instructions,
    functions: &mut BTreeMap<
        Hash<Function>,
        Vec<(Vec<Pattern>, InstructionAddress)>,
    >,
) {
    if let Some(main) = named_functions.find_by_name("main") {
        compile_call_to_function(
            &Hash::new(&main),
            &CallToFunction {
                address: call_to_main,
                is_tail_call: true,
            },
            functions,
            instructions,
        );
    }
}
