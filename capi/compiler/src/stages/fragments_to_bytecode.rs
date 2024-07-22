use std::collections::{BTreeSet, VecDeque};

use capi_process::{Bytecode, Function, Instruction, InstructionAddr};

use crate::{
    repr::fragments::{
        self, Fragment, FragmentExpression, FragmentId, FragmentMap,
        FragmentPayload, Fragments,
    },
    source_map::SourceMap,
};

pub fn fragments_to_bytecode(fragments: Fragments) -> (Bytecode, SourceMap) {
    let mut bytecode = Bytecode::default();
    let mut source_map = SourceMap::default();

    let mut compiler = Compiler {
        queue: VecDeque::new(),
        bytecode: &mut bytecode,
        source_map: &mut source_map,
        fragments: &fragments.inner,
    };

    compiler
        .queue
        .extend(fragments.by_function.into_iter().map(CompileUnit::Function));
    compiler.compile();

    (bytecode, source_map)
}

struct Compiler<'r> {
    queue: VecDeque<CompileUnit>,
    bytecode: &'r mut Bytecode,
    source_map: &'r mut SourceMap,
    fragments: &'r FragmentMap,
}

impl Compiler<'_> {
    fn compile(&mut self) {
        while let Some(unit) = self.queue.pop_front() {
            match unit {
                CompileUnit::Block {
                    start,
                    environment,
                    addr,
                } => {
                    let start = self.compile_block(start);

                    self.bytecode.instructions.replace(
                        addr,
                        Instruction::MakeClosure {
                            addr: start,
                            environment,
                        },
                    );
                }
                CompileUnit::Function(function) => {
                    self.compile_function(
                        function.name,
                        function.args,
                        function.start,
                    );
                }
            }
        }
    }

    fn compile_function(
        &mut self,
        name: String,
        arguments: Vec<String>,
        start: FragmentId,
    ) {
        let first_instruction = self.compile_block(start);

        self.bytecode.functions.insert(
            name,
            Function {
                arguments,
                first_instruction,
            },
        );
    }

    fn compile_block(&mut self, start: FragmentId) -> InstructionAddr {
        let mut first_instruction = None;

        for fragment in self.fragments.iter_from(start) {
            let addr = self.compile_fragment(fragment);
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
    ) -> Option<InstructionAddr> {
        let addr = match &fragment.payload {
            FragmentPayload::Expression { expression, .. } => {
                match expression {
                    FragmentExpression::BindingDefinitions { names } => self
                        .generate(
                            Instruction::BindingsDefine {
                                names: names.clone(),
                            },
                            fragment.id(),
                        ),
                    FragmentExpression::Block { start, environment } => {
                        let addr =
                            self.generate(Instruction::Panic, fragment.id());

                        self.queue.push_front(CompileUnit::Block {
                            start: *start,
                            environment: environment.clone(),
                            addr,
                        });

                        addr
                    }
                    FragmentExpression::Comment { .. } => {
                        return None;
                    }
                    FragmentExpression::ResolvedBinding { name } => self
                        .generate(
                            Instruction::BindingEvaluate { name: name.clone() },
                            fragment.id(),
                        ),
                    FragmentExpression::ResolvedBuiltinFunction { name } => {
                        // Here we check for special built-in functions that are
                        // implemented differently, without making sure
                        // anywhere, that their name doesn't conflict with any
                        // user-defined functions.
                        //
                        // I think it's fine for now. This seems like a
                        // temporary hack anyway, while the language is not
                        // powerful enough to support real conditionals.
                        let instruction = if name == "return_if_non_zero" {
                            Instruction::ReturnIfNonZero
                        } else if name == "return_if_zero" {
                            Instruction::ReturnIfZero
                        } else {
                            Instruction::CallBuiltin { name: name.clone() }
                        };

                        self.generate(instruction, fragment.id())
                    }
                    FragmentExpression::ResolvedHostFunction { name } => self
                        .generate(
                            Instruction::CallBuiltin { name: name.clone() },
                            fragment.id(),
                        ),
                    FragmentExpression::ResolvedUserFunction { name } => self
                        .generate(
                            Instruction::CallFunction { name: name.clone() },
                            fragment.id(),
                        ),
                    FragmentExpression::UnresolvedWord { name: _ } => {
                        self.generate(Instruction::Panic, fragment.id())
                    }
                    FragmentExpression::Value(value) => self.generate(
                        Instruction::Push { value: *value },
                        fragment.id(),
                    ),
                }
            }
            FragmentPayload::Terminator => {
                self.generate(Instruction::Return, fragment.id())
            }
        };

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

enum CompileUnit {
    Block {
        start: FragmentId,
        environment: BTreeSet<String>,
        addr: InstructionAddr,
    },
    Function(fragments::Function),
}
