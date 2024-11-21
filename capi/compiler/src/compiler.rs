use std::collections::BTreeMap;

use capi_runtime::InstructionAddress;

use crate::{
    code::{
        syntax::parse, Bindings, Function, FunctionCalls, Hash,
        OrderedFunctions, Recursion, StableFunctions, TailExpressions, Tokens,
    },
    host::Host,
    passes::{
        detect_changes, generate_instructions, order_functions_by_dependencies,
        resolve_non_recursive_functions, resolve_recursive_calls,
        resolve_recursive_local_functions,
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
    pub fn compile(&mut self, input: &str, host: &impl Host) -> CompilerOutput {
        let tokens = Tokens::from_input(input);
        let mut functions = parse(tokens);
        let bindings = Bindings::resolve(&functions);
        let function_calls = FunctionCalls::resolve(&functions, host);
        let tail_expressions = TailExpressions::find(&functions);
        let ordered_functions =
            order_functions_by_dependencies(&functions, &function_calls);
        let recursion = Recursion::find(&functions, &ordered_functions);
        resolve_recursive_calls(
            &mut functions,
            &function_calls,
            &ordered_functions,
        );
        resolve_recursive_local_functions(&mut functions, &ordered_functions);
        let functions = resolve_non_recursive_functions(
            functions,
            &function_calls,
            &ordered_functions,
        );
        let changes = detect_changes(self.old_functions.take(), &functions);

        self.old_functions = Some(functions.clone());

        generate_instructions(
            &functions,
            &ordered_functions,
            &bindings,
            &function_calls,
            &tail_expressions,
            &recursion,
            &changes,
            &mut self.instructions,
            &mut self.call_instructions_by_callee,
            &mut self.compiled_functions_by_hash,
            &mut self.source_map,
        );

        CompilerOutput {
            functions,
            function_calls,
            ordered_functions,
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
    pub function_calls: FunctionCalls,
    pub ordered_functions: OrderedFunctions,
    pub instructions: Instructions,
    pub source_map: SourceMap,
}
