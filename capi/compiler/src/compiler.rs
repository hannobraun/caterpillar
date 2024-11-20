use std::collections::BTreeMap;

use capi_runtime::InstructionAddress;

use crate::{
    code::{
        tokens::{tokenize, Tokens},
        Function, Hash, OrderedFunctions, StableFunctions, Types,
    },
    host::Host,
    passes::{
        detect_changes, find_divergent_functions, generate_instructions,
        infer_types, mark_tail_positions, order_functions_by_dependencies,
        parse, resolve_most_identifiers, resolve_non_recursive_functions,
        resolve_recursive_calls, resolve_recursive_local_functions,
        sort_non_divergent_branches,
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
        let tokens = Tokens {
            inner: tokens.into(),
        };
        let mut functions = parse(tokens);
        mark_tail_positions(&mut functions);
        resolve_most_identifiers(&mut functions, host);
        let mut ordered_functions = order_functions_by_dependencies(&functions);
        resolve_recursive_calls(&mut functions, &ordered_functions);
        resolve_recursive_local_functions(&mut functions, &ordered_functions);
        let functions =
            resolve_non_recursive_functions(functions, &ordered_functions);
        find_divergent_functions(&functions, &mut ordered_functions);
        sort_non_divergent_branches(&functions, &mut ordered_functions);
        let types = infer_types(&functions, &ordered_functions, host);
        let changes = detect_changes(self.old_functions.take(), &functions);

        self.old_functions = Some(functions.clone());

        generate_instructions(
            &functions,
            &ordered_functions,
            &changes,
            &mut self.instructions,
            &mut self.call_instructions_by_callee,
            &mut self.compiled_functions_by_hash,
            &mut self.source_map,
        );

        CompilerOutput {
            functions,
            ordered_functions,
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
    pub ordered_functions: OrderedFunctions,
    pub types: Types,
    pub instructions: Instructions,
    pub source_map: SourceMap,
}
