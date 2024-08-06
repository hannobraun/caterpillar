use std::collections::{BTreeMap, BTreeSet, VecDeque};

use capi_process::{Bytecode, Instruction, InstructionAddress, Instructions};

use crate::{
    fragments::{
        Fragment, FragmentExpression, FragmentId, FragmentMap, FragmentPayload,
        Fragments, Function,
    },
    source_map::SourceMap,
    syntax::Pattern,
};

pub fn generate_bytecode(fragments: Fragments) -> (Bytecode, SourceMap) {
    let mut instructions = Instructions::default();
    let mut source_map = SourceMap::default();

    // This is a placeholder for the instruction that's going to call the entry
    // function.
    let main = instructions.push(Instruction::Panic);
    instructions.push(Instruction::Return);

    let mut compiler = Compiler {
        queue: VecDeque::new(),
        instructions,
        calls_to_user_defined_functions: Vec::new(),
        function_arguments_by_address: BTreeMap::new(),
        function_addresses_by_name: BTreeMap::new(),
        source_map: &mut source_map,
        fragments: &fragments.inner,
    };

    compiler.compile_context(fragments.root);
    compiler.compile();

    if let Some(address) = compiler.function_addresses_by_name.get("main") {
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
            main,
            Instruction::CallFunction {
                address: *address,
                is_tail_call: true,
            },
        );
    }

    for call in compiler.calls_to_user_defined_functions {
        let Some(address) = compiler.function_addresses_by_name.get(&call.name)
        else {
            unreachable!(
                "Expecting function `{}` to exist. If it didn't, the previous \
                compilation step would not have generated the fragment that \
                caused us to assume that it does.",
                call.name,
            );
        };

        compiler.instructions.replace(
            call.address,
            Instruction::CallFunction {
                address: *address,
                is_tail_call: call.is_tail_call,
            },
        );
    }

    let bytecode = Bytecode {
        instructions: compiler.instructions,
        function_arguments: compiler.function_arguments_by_address,
    };

    (bytecode, source_map)
}

struct Compiler<'r> {
    queue: VecDeque<CompileUnit>,
    instructions: Instructions,
    calls_to_user_defined_functions: Vec<CallToUserDefinedFunction>,
    function_arguments_by_address: BTreeMap<InstructionAddress, Vec<String>>,
    function_addresses_by_name: BTreeMap<String, InstructionAddress>,
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
                    let start = self.compile_context(start);

                    self.instructions.replace(
                        address,
                        Instruction::MakeClosure {
                            address: start,
                            environment,
                        },
                    );
                }
                CompileUnit::Function(function) => {
                    self.compile_function(function);
                }
            }
        }
    }

    fn compile_function(&mut self, function: Function) {
        let address = self.compile_context(function.start);
        let arguments = function
            .arguments
            .into_iter()
            .filter_map(|pattern| match pattern {
                Pattern::Identifier { name } => Some(name),
                Pattern::Literal { .. } => {
                    // The parameter list of a function is used to provide the
                    // arguments to the function at runtime. But literal
                    // patterns aren't relevant to the function itself. They are
                    // only used to select which function to call in the first
                    // place.
                    None
                }
            })
            .collect();

        self.function_arguments_by_address
            .insert(address, arguments);
        self.function_addresses_by_name
            .insert(function.name, address);
    }

    fn compile_context(&mut self, start: FragmentId) -> InstructionAddress {
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
                        // We are currently compiling a function or block
                        // (otherwise we wouldn't be encountering any
                        // expression), and the instructions for that will be
                        // executed linearly.
                        //
                        // Which means we can't just start compiling this block
                        // right now. Its instructions would go into the middle
                        // of those other instructions and mess everything up.
                        //
                        // What _should_ happen instead, is that the block is
                        // turned into a closure, that can be passed around as a
                        // value and called whenever so desired.
                        //
                        // So for now, let's just generate this instruction as
                        // a placeholder, to be replaced with another
                        // instruction that creates that closure, once we have
                        // everything in place to make that happen.
                        let address =
                            self.generate(Instruction::Panic, fragment.id());

                        // And to make it happen later, we need to put what we
                        // already have into a queue. Once whatever's currently
                        // being compiled is out of the way, we can process
                        // that.
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
                        name,
                        is_tail_call,
                    } => {
                        // We know that this expression refers to a user-defined
                        // function, but we might not have compiled that
                        // function yet.
                        //
                        // For now, just generate a placeholder that we can
                        // replace with the call later.
                        let address =
                            self.generate(Instruction::Panic, fragment.id());

                        // We can't leave it at that, however. We need to make
                        // sure this placeholder actually gets replace later,
                        // and we're doing that by adding it to this list.
                        self.calls_to_user_defined_functions.push(
                            CallToUserDefinedFunction {
                                name: name.clone(),
                                address,
                                is_tail_call: *is_tail_call,
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
                    .push_back(CompileUnit::Function(function.clone()));
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
    Function(Function),
}

struct CallToUserDefinedFunction {
    name: String,
    address: InstructionAddress,
    is_tail_call: bool,
}
