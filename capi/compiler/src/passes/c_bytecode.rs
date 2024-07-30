use std::collections::{BTreeMap, BTreeSet, VecDeque};

use capi_process::{
    Bytecode, Function, Instruction, InstructionAddr, Instructions,
};

use crate::{
    repr::fragments::{
        self, Fragment, FragmentExpression, FragmentId, FragmentMap,
        FragmentPayload, Fragments,
    },
    source_map::SourceMap,
};

pub fn generate_bytecode(fragments: Fragments) -> (Bytecode, SourceMap) {
    let mut source_map = SourceMap::default();

    let mut compiler = Compiler {
        queue: VecDeque::new(),
        instructions: Instructions::default(),
        functions_by_address: BTreeMap::new(),
        functions_by_name: BTreeMap::new(),
        source_map: &mut source_map,
        fragments: &fragments.inner,
    };

    // This is a placeholder for the instruction that's going to call the entry
    // function.
    let init = compiler.instructions.push(Instruction::Panic);
    compiler.instructions.push(Instruction::Return);

    compiler
        .queue
        .extend(fragments.by_function.into_iter().map(CompileUnit::Function));
    compiler.compile();

    if let Some(_main) = compiler.functions_by_name.get("main") {
        // If we have an entry function, replace that panic instruction we added
        // as a placeholder.
        //
        // Right now, this will just result in an non-descriptive panic, if no
        // entry function was provided. Eventually, the panic instruction might
        // grow a "reason" parameter which will provide more clarity in such a
        // case.
        //
        // In addition, this is something that should be detected during pre-
        // compilation, and result in a nice error message in the debugger.
        compiler.instructions.replace(
            init,
            Instruction::CallFunction {
                name: "main".to_string(),
            },
        );
    }

    let bytecode = Bytecode {
        instructions: compiler.instructions,
        functions_by_address: compiler.functions_by_address,
        functions_by_name: compiler.functions_by_name,
    };

    (bytecode, source_map)
}

struct Compiler<'r> {
    queue: VecDeque<CompileUnit>,
    instructions: Instructions,
    functions_by_address: BTreeMap<InstructionAddr, Function>,
    functions_by_name: BTreeMap<String, Function>,
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

                    self.instructions.replace(
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
        let start = self.compile_block(start);

        self.functions_by_address.insert(
            start,
            Function {
                arguments: arguments.clone(),
                start,
            },
        );
        self.functions_by_name
            .insert(name, Function { arguments, start });
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
        let addr = self.instructions.push(instruction);
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
