use capi_runtime::Instructions;

use crate::{
    code::{CallGraph, NamedFunctions},
    host::Host,
    passes::{
        create_call_graph, detect_changes, determine_tail_positions,
        generate_fragments, generate_instructions, mark_recursive_calls, parse,
        resolve_calls_to_user_defined_functions, resolve_most_identifiers,
        tokenize,
    },
    source_map::SourceMap,
};

/// # Entry point to the compiler API
#[derive(Default)]
pub struct Compiler {
    old_functions: Option<NamedFunctions>,
    instructions: Instructions,
    source_map: SourceMap,
}

impl Compiler {
    /// # Compile the provided source code
    pub fn compile<H: Host>(&mut self, source: &str) -> CompilerOutput {
        let tokens = tokenize(source);
        let mut functions = parse(tokens);
        determine_tail_positions(&mut functions);
        resolve_most_identifiers::<H>(&mut functions);
        let (mut functions, call_graph) = create_call_graph(functions);
        mark_recursive_calls(&mut functions, &call_graph);
        resolve_calls_to_user_defined_functions(&mut functions, &call_graph);

        let named_functions = generate_fragments(functions, &call_graph);
        let changes =
            detect_changes(self.old_functions.take(), &named_functions);

        self.old_functions = Some(named_functions.clone());

        generate_instructions(
            &named_functions,
            &call_graph,
            &changes,
            &mut self.instructions,
            &mut self.source_map,
        );

        CompilerOutput {
            named_functions,
            call_graph,
            instructions: self.instructions.clone(),
            source_map: self.source_map.clone(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CompilerOutput {
    pub named_functions: NamedFunctions,
    pub call_graph: CallGraph,
    pub instructions: Instructions,
    pub source_map: SourceMap,
}
