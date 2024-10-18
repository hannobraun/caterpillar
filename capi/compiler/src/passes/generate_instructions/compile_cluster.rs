use std::collections::VecDeque;

use crate::{
    code::{Changes, Cluster, FunctionLocation},
    hash::Hash,
};

use super::{
    compile_function::{
        compile_call_to_function, compile_function, FunctionToCompile,
    },
    compile_named_functions::NamedFunctionsContext,
};

pub struct ClusterContext {
    pub queue_of_functions_to_compile: VecDeque<FunctionToCompile>,
}

pub fn compile_cluster(
    cluster: &Cluster,
    changes: &Changes,
    named_functions_context: &mut NamedFunctionsContext,
) {
    let mut context = ClusterContext {
        queue_of_functions_to_compile: VecDeque::new(),
    };

    seed_queue_of_functions_to_compile(
        &mut context.queue_of_functions_to_compile,
        cluster,
        changes,
    );

    while let Some(function_to_compile) =
        context.queue_of_functions_to_compile.pop_front()
    {
        let hash = Hash::new(&function_to_compile.function);
        let runtime_function = compile_function(
            function_to_compile,
            &mut context,
            named_functions_context,
        );
        named_functions_context
            .compiled_functions_by_hash
            .insert(hash, runtime_function);
    }

    for (hash, calls) in
        &named_functions_context.recursive_function_calls_by_callee_hash
    {
        for call in calls {
            compile_call_to_function(
                hash,
                call,
                &mut named_functions_context.compiled_functions_by_hash,
                named_functions_context.instructions,
            );
        }
    }
}

fn seed_queue_of_functions_to_compile(
    queue_of_functions_to_compile: &mut VecDeque<FunctionToCompile>,
    cluster: &Cluster,
    changes: &Changes,
) {
    let functions_in_cluster_to_compile =
        cluster.functions.values().filter_map(|&index| {
            let function = changes.new_or_updated_function(&index)?;
            Some(FunctionToCompile {
                function: function.clone(),
                location: FunctionLocation::NamedFunction { index },
                cluster: cluster.clone(),
                address_of_instruction_to_make_anon_function: None,
            })
        });
    queue_of_functions_to_compile.extend(functions_in_cluster_to_compile);
}
