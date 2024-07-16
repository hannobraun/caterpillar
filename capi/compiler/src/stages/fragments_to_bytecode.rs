use capi_process::{Bytecode, Function, Instruction, InstructionAddr, Value};

use crate::{
    repr::fragments::{
        Fragment, FragmentExpression, FragmentId, FragmentMap, FragmentPayload,
        Fragments,
    },
    source_map::SourceMap,
};

pub fn fragments_to_bytecode(fragments: Fragments) -> (Bytecode, SourceMap) {
    let mut bytecode = Bytecode::default();
    let mut source_map = SourceMap::default();

    let mut compiler = Compiler {
        bytecode: &mut bytecode,
        source_map: &mut source_map,
    };

    for function in fragments.by_function {
        compiler.compile_function(
            function.name,
            function.args,
            function.start,
            &fragments.inner,
        );
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
        arguments: Vec<String>,
        start: FragmentId,
        fragments: &FragmentMap,
    ) {
        let first_instruction = self.compile_block(start, fragments);

        self.bytecode.functions.insert(
            name.clone(),
            Function {
                name,
                arguments,
                first_instruction,
            },
        );
    }

    fn compile_block(
        &mut self,
        start: FragmentId,
        fragments: &FragmentMap,
    ) -> InstructionAddr {
        let mut first_instruction = None;

        for fragment in fragments.iter_from(start) {
            let addr = self.compile_fragment(fragment, fragments);
            first_instruction = first_instruction.or(addr);
        }

        let Some(first_instruction) = first_instruction else {
            unreachable!(
                "Must have generated at least one instruction for the block: \
                the return instruction. If this has not happened, the \
                fragments have somehow been missing a terminator."
            );
        };

        first_instruction
    }

    fn compile_fragment(
        &mut self,
        fragment: &Fragment,
        fragments: &FragmentMap,
    ) -> Option<InstructionAddr> {
        let instruction = match &fragment.payload {
            FragmentPayload::Expression { expression, .. } => {
                match expression {
                    FragmentExpression::BindingDefinitions { names } => {
                        Instruction::BindingsDefine {
                            names: names.clone(),
                        }
                    }
                    FragmentExpression::BindingEvaluation { name } => {
                        Instruction::BindingEvaluate { name: name.clone() }
                    }
                    FragmentExpression::Block { start } => {
                        let start = self.compile_block(*start, fragments);

                        Instruction::Push {
                            value: Value(start.index.to_le_bytes()),
                        }
                    }
                    FragmentExpression::BuiltinCall { name } => {
                        // Here we check for special built-in functions that are
                        // implemented differently, without making sure
                        // anywhere, that their name doesn't conflict with any
                        // user-defined functions.
                        //
                        // I think it's fine for now. This seems like a
                        // temporary hack anyway, while the language is not
                        // powerful enough to support real conditionals.
                        if name == "return_if_non_zero" {
                            Instruction::ReturnIfNonZero
                        } else if name == "return_if_zero" {
                            Instruction::ReturnIfZero
                        } else {
                            Instruction::CallBuiltin { name: name.clone() }
                        }
                    }
                    FragmentExpression::Comment { .. } => {
                        return None;
                    }
                    FragmentExpression::FunctionCall { name } => {
                        Instruction::CallFunction { name: name.clone() }
                    }
                    FragmentExpression::Value(value) => {
                        Instruction::Push { value: *value }
                    }
                }
            }
            FragmentPayload::Terminator => Instruction::Return,
        };

        let addr = self.generate(instruction, fragment.id());
        Some(addr)
    }

    fn generate(
        &mut self,
        instruction: Instruction,
        fragment_id: FragmentId,
    ) -> InstructionAddr {
        let addr = self.bytecode.instructions.push(instruction);
        self.source_map.define_mapping(addr, fragment_id);
        addr
    }
}
