use std::iter;

use capi_process::{Bytecode, Function, Instruction, Location};

use crate::{
    repr::fragments::{
        Fragment, FragmentExpression, FragmentId, FragmentPayload, Fragments,
    },
    source_map::SourceMap,
};

pub fn fragments_to_bytecode(
    mut fragments: Fragments,
) -> (Bytecode, SourceMap) {
    let mut bytecode = Bytecode::default();
    let mut source_map = SourceMap::default();

    let mut compiler = Compiler {
        bytecode: &mut bytecode,
        source_map: &mut source_map,
    };

    for function in fragments.by_function {
        if let Some(start) = function.start {
            compiler.compile_function(
                function.name,
                function.args,
                fragments.inner.drain_from(start),
            );
        } else {
            compiler.compile_function(
                function.name,
                function.args,
                iter::empty(),
            );
        }
    }

    (bytecode, source_map)
}

struct Compiler<'r> {
    bytecode: &'r mut Bytecode,
    source_map: &'r mut SourceMap,
}

impl Compiler<'_> {
    fn compile_function(
        &mut self,
        name: String,
        args: Vec<String>,
        fragments: impl IntoIterator<Item = Fragment>,
    ) {
        let mut output = Function::new(name.clone(), args);

        for fragment in fragments {
            self.compile_fragment(fragment, &mut output);
        }

        self.bytecode.functions.insert(name, output);
    }

    fn compile_fragment(&mut self, fragment: Fragment, output: &mut Function) {
        let fragment_id = fragment.id();

        let expression = match fragment.payload {
            FragmentPayload::Expression(expression) => expression,
            FragmentPayload::Terminator => {
                // A terminator only has meaning in the source code
                // representation, but doesn't do anything at runtime.
                return;
            }
        };

        match expression {
            FragmentExpression::BindingDefinitions { names } => {
                self.generate(
                    Instruction::BindingsDefine { names },
                    fragment_id,
                    output,
                );
            }
            FragmentExpression::BindingEvaluation { name } => {
                self.generate(
                    Instruction::BindingEvaluate { name },
                    fragment_id,
                    output,
                );
            }
            FragmentExpression::BuiltinCall { name } => {
                let instruction = {
                    // Here we check for special built-in functions that are
                    // implemented differently, without making sure anywhere,
                    // that their name doesn't conflict with any user-defined
                    // functions.
                    //
                    // I think it's fine for now. This seems like a temporary
                    // hack anyway, while the language is not powerful enough
                    // to support real conditionals.
                    if name == "return_if_non_zero" {
                        Instruction::ReturnIfNonZero
                    } else if name == "return_if_zero" {
                        Instruction::ReturnIfZero
                    } else {
                        Instruction::CallBuiltin { name }
                    }
                };
                self.generate(instruction, fragment_id, output);
            }
            FragmentExpression::Comment { .. } => {}
            FragmentExpression::FunctionCall { name } => {
                self.generate(
                    Instruction::CallFunction { name },
                    fragment_id,
                    output,
                );
            }
            FragmentExpression::Value(value) => {
                self.generate(Instruction::Push { value }, fragment_id, output);
            }
        };
    }

    fn generate(
        &mut self,
        instruction: Instruction,
        fragment_id: FragmentId,
        output: &mut Function,
    ) {
        let index = output.instructions.push(instruction);

        let runtime_location = Location {
            function: output.name.clone(),
            index,
        };
        self.source_map
            .define_mapping(runtime_location.clone(), fragment_id);
    }
}
