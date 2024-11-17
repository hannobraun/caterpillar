use std::collections::BTreeMap;

use capi_runtime::InstructionAddress;

use crate::{
    code::{Function, Hash, OrderedFunctions, StableFunctions, Types},
    host::Host,
    passes::{
        detect_changes, find_divergent_functions, generate_instructions,
        infer_types, mark_tail_positions, order_functions_by_dependencies,
        parse, resolve_calls_to_user_defined_functions,
        resolve_most_identifiers, resolve_recursive_calls, tokenize,
    },
    source_map::SourceMap,
    Instructions,
};

/// # Entry point to the compiler API
#[derive(Default)]
pub struct Compiler {
    old_functions: Option<StableFunctions>,
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
        let mut functions = parse(tokens);
        mark_tail_positions(&mut functions);
        resolve_most_identifiers(&mut functions, host);
        let mut call_graph = order_functions_by_dependencies(&functions);
        resolve_recursive_calls(&mut functions, &call_graph);
        let functions =
            resolve_calls_to_user_defined_functions(functions, &call_graph);
        find_divergent_functions(&functions, &mut call_graph);
        let types = infer_types(&functions, &call_graph, host);
        let changes = detect_changes(self.old_functions.take(), &functions);

        self.old_functions = Some(functions.clone());

        generate_instructions(
            &functions,
            &call_graph,
            &changes,
            &mut self.instructions,
            &mut self.call_instructions_by_callee,
            &mut self.compiled_functions_by_hash,
            &mut self.source_map,
        );

        CompilerOutput {
            functions,
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
    pub functions: StableFunctions,
    pub call_graph: OrderedFunctions,
    pub types: Types,
    pub instructions: Instructions,
    pub source_map: SourceMap,
}
