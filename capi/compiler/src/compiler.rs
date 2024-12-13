use std::collections::BTreeMap;

use capi_runtime::InstructionAddress;

use crate::{
    code::{
        syntax::{FunctionLocation, SyntaxTree},
        Bindings, ExplicitTypes, FunctionCalls, Functions, OrderedFunctions,
        Recursion, TailExpressions, Tokens, Types,
    },
    host::Host,
    passes::{
        detect_changes, generate_instructions, order_functions_by_dependencies,
    },
    source_map::SourceMap,
    Instructions,
};

/// # Entry point to the compiler API
#[derive(Default)]
pub struct Compiler {
    old_code: Option<SyntaxTree>,
    instructions: Instructions,
    call_instructions_by_callee: CallInstructionsByCallee,
    compiled_functions_by_location:
        BTreeMap<FunctionLocation, capi_runtime::Function>,
    source_map: SourceMap,
}

impl Compiler {
    /// # Compile the provided source code
    pub fn compile(&mut self, input: &str, host: &impl Host) -> CompilerOutput {
        let tokens = Tokens::tokenize(input);
        let syntax_tree = SyntaxTree::parse(tokens);
        let bindings = Bindings::resolve(&syntax_tree);
        let function_calls = FunctionCalls::resolve(&syntax_tree, host);
        let tail_expressions = TailExpressions::find(&syntax_tree);
        let explicit_types = ExplicitTypes::resolve(&syntax_tree);
        let types = Types::infer(
            &syntax_tree,
            &bindings,
            &function_calls,
            explicit_types,
        );
        let ordered_functions =
            order_functions_by_dependencies(&syntax_tree, &function_calls);
        let functions = Functions {
            inner: syntax_tree
                .all_functions()
                .map(|function| (function.location, function.fragment.clone()))
                .collect(),
        };
        let recursion =
            Recursion::find(&function_calls, &functions, &ordered_functions);
        let changes = detect_changes(self.old_code.take(), &syntax_tree);

        self.old_code = Some(syntax_tree.clone());

        generate_instructions(
            &syntax_tree,
            &functions,
            &ordered_functions,
            &bindings,
            &function_calls,
            &tail_expressions,
            &types,
            &recursion,
            &changes,
            &mut self.instructions,
            &mut self.call_instructions_by_callee,
            &mut self.compiled_functions_by_location,
            &mut self.source_map,
        );

        CompilerOutput {
            syntax_tree,
            functions,
            function_calls,
            ordered_functions,
            types,
            instructions: self.instructions.clone(),
            source_map: self.source_map.clone(),
        }
    }
}

#[derive(Default)]
pub struct CallInstructionsByCallee {
    pub inner: BTreeMap<FunctionLocation, Vec<InstructionAddress>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CompilerOutput {
    pub syntax_tree: SyntaxTree,
    pub functions: Functions,
    pub function_calls: FunctionCalls,
    pub ordered_functions: OrderedFunctions,
    pub types: Types,
    pub instructions: Instructions,
    pub source_map: SourceMap,
}
