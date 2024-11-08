use std::collections::{BTreeMap, VecDeque};

use crate::code::{Changes, Cluster, Function, FunctionLocation, Hash};

use super::{
    compile_function::{
        compile_call_to_function, compile_function, CallToFunction,
        FunctionToCompile,
    },
    compile_functions::FunctionsContext,
};

pub struct ClusterContext {
    /// # The queue of functions to compile in the cluster
    ///
    /// This is initially seeded by the named functions in the cluster that are
    /// new or have been updated. But any anonymous functions encountered while
    /// compiling those, will be added later.
    pub queue_of_functions_to_compile: VecDeque<FunctionToCompile>,

    /// # Recursive calls within the cluster that need to be replaced
    ///
    /// When a recursive call is encountered, not all branches of the callee
    /// (which might be the calling function itself, or another function in the
    /// same cluster) might be compiled yet. But they're needed to compile the
    /// call.
    ///
    /// So instead of compiling the call right then and there, a placeholder
    /// instruction is emitted. An entry is also added to this map, so the
    /// placeholder instruction can be replaced with the real call, once all
    /// functions have been compiled.
    pub recursive_calls_by_callee:
        BTreeMap<Hash<Function>, Vec<CallToFunction>>,
}

pub fn compile_cluster(
    cluster: &Cluster,
    changes: &Changes,
    functions_context: &mut FunctionsContext,
) {
    let mut context = ClusterContext {
        queue_of_functions_to_compile: VecDeque::new(),
        recursive_calls_by_callee: BTreeMap::new(),
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
            function_to_compile.function,
            function_to_compile.location,
            function_to_compile.address_of_instruction_to_make_anon_function,
            cluster,
            &mut context,
            functions_context,
        );

        functions_context
            .compiled_functions_by_hash
            .insert(hash, runtime_function);
    }

    for (hash, calls) in context.recursive_calls_by_callee {
        for call in calls {
            compile_call_to_function(
                &hash,
                call,
                functions_context.compiled_functions_by_hash,
                functions_context.instructions,
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
                address_of_instruction_to_make_anon_function: None,
            })
        });
    queue_of_functions_to_compile.extend(functions_in_cluster_to_compile);
}
