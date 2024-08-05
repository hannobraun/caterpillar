use std::collections::{BTreeMap, BTreeSet, VecDeque};

use capi_process::{
    Bytecode, Function, Instruction, InstructionAddress, Instructions,
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
        calls_to_user_defined_functions: Vec::new(),
        functions_by_address: BTreeMap::new(),
        functions_by_name: BTreeMap::new(),
        source_map: &mut source_map,
        fragments: &fragments.inner,
    };

    // This is a placeholder for the instruction that's going to call the entry
    // function.
    let init = compiler.instructions.push(Instruction::Panic);
    compiler.instructions.push(Instruction::Return);

    compiler.queue.extend(
        fragments
            .inner
            .iter_from(fragments.root)
            .filter_map(|fragment| {
                if let FragmentPayload::Function(function) = &fragment.payload {
                    Some(function)
                } else {
                    None
                }
            })
            .cloned()
            .map(CompileUnit::Function),
    );
    compiler.compile();

    if let Some(main) = compiler.functions_by_name.get("main") {
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
                address: main.start,
            },
        );
    }

    for CallToUserDefinedFunction { name, address } in
        compiler.calls_to_user_defined_functions
    {
        let Some(function) = compiler.functions_by_name.get(&name) else {
            unreachable!(
                "Expecting function `{}` to exist. If it didn't, the previous \
                compilation step would not have generated the fragment that \
                caused us to assume that it does.",
                name,
            );
        };

        let address_of_function = function.start;

        compiler.instructions.replace(
            address,
            Instruction::CallFunction {
                address: address_of_function,
            },
        );
    }

    let bytecode = Bytecode {
        instructions: compiler.instructions,
        functions: compiler.functions_by_address,
    };

    (bytecode, source_map)
}

struct Compiler<'r> {
    queue: VecDeque<CompileUnit>,
    instructions: Instructions,
    calls_to_user_defined_functions: Vec<CallToUserDefinedFunction>,
    functions_by_address: BTreeMap<InstructionAddress, Function>,
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
                    address,
                } => {
                    let start = self.compile_block(start);

                    self.instructions.replace(
                        address,
                        Instruction::MakeClosure {
                            address: start,
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

    fn compile_block(&mut self, start: FragmentId) -> InstructionAddress {
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
    ) -> Option<InstructionAddress> {
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
                        let address =
                            self.generate(Instruction::Panic, fragment.id());

                        self.queue.push_front(CompileUnit::Block {
                            start: *start,
                            environment: environment.clone(),
                            address,
                        });

                        address
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
                    FragmentExpression::ResolvedUserFunction {
                        name, ..
                    } => {
                        let address =
                            self.generate(Instruction::Panic, fragment.id());
                        self.calls_to_user_defined_functions.push(
                            CallToUserDefinedFunction {
                                name: name.clone(),
                                address,
                            },
                        );
                        address
                    }
                    FragmentExpression::UnresolvedIdentifier { name: _ } => {
                        self.generate(Instruction::Panic, fragment.id())
                    }
                    FragmentExpression::Value(value) => self.generate(
                        Instruction::Push { value: *value },
                        fragment.id(),
                    ),
                }
            }
            FragmentPayload::Function(function) => {
                self.queue
                    .push_front(CompileUnit::Function(function.clone()));
                return None;
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
    ) -> InstructionAddress {
        let addr = self.instructions.push(instruction);
        self.source_map.define_mapping(addr, fragment_id);
        addr
    }
}

enum CompileUnit {
    Block {
        start: FragmentId,
        environment: BTreeSet<String>,
        address: InstructionAddress,
    },
    Function(fragments::Function),
}

struct CallToUserDefinedFunction {
    name: String,
    address: InstructionAddress,
}
