use std::collections::BTreeMap;

use capi_runtime::InstructionAddress;

use crate::{
    code::{CallGraph, Function, Functions, Hash, Types},
    host::Host,
    passes::{
        build_call_graph, detect_changes, determine_tail_positions,
        find_divergent_functions, generate_instructions, infer_types,
        mark_recursive_calls, parse, resolve_calls_to_user_defined_functions,
        resolve_most_identifiers, tokenize,
    },
    source_map::SourceMap,
    Instructions,
};

/// # Entry point to the compiler API
#[derive(Default)]
pub struct Compiler {
    old_functions: Option<Functions>,
    instructions: Instructions,
    call_instructions_by_callee: CallInstructionsByCallee,
    compiled_functions_by_hash:
        BTreeMap<Hash<Function>, capi_runtime::Function>,
    source_map: SourceMap,
}

impl Compiler {
    /// # Compile the provided source code
    pub fn compile(
        &mut self,
        source: &str,
        host: &impl Host,
    ) -> CompilerOutput {
        let tokens = tokenize(source);
        let mut named_functions = parse(tokens);
        determine_tail_positions(&mut named_functions);
        resolve_most_identifiers(&mut named_functions, host);
        let mut call_graph = build_call_graph(&named_functions);
        mark_recursive_calls(&mut named_functions, &call_graph);
        resolve_calls_to_user_defined_functions(
            &mut named_functions,
            &call_graph,
        );
        find_divergent_functions(&named_functions, &mut call_graph);
        let types = infer_types(&named_functions, &call_graph, host);
        let changes =
            detect_changes(self.old_functions.take(), &named_functions);

        self.old_functions = Some(named_functions.clone());

        generate_instructions(
            &named_functions,
            &call_graph,
            &changes,
            &mut self.instructions,
            &mut self.call_instructions_by_callee,
            &mut self.compiled_functions_by_hash,
            &mut self.source_map,
        );

        CompilerOutput {
            named_functions,
            call_graph,
            types,
            instructions: self.instructions.clone(),
            source_map: self.source_map.clone(),
        }
    }
}

#[derive(Default)]
pub struct CallInstructionsByCallee {
    pub inner: BTreeMap<Hash<Function>, Vec<InstructionAddress>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CompilerOutput {
    pub named_functions: Functions,
    pub call_graph: CallGraph,
    pub types: Types,
    pub instructions: Instructions,
    pub source_map: SourceMap,
}
