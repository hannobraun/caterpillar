use std::collections::{BTreeMap, BTreeSet, VecDeque};

use capi_process::{Bytecode, Instruction, InstructionAddress, Instructions};

use crate::{
    fragments::{
        Fragment, FragmentExpression, FragmentId, FragmentMap, FragmentPayload,
        Fragments, Function,
    },
    placeholders::{CallToUserDefinedFunction, Placeholders},
    source_map::SourceMap,
    syntax::Pattern,
};

pub fn generate_bytecode(fragments: Fragments) -> (Bytecode, SourceMap) {
    let mut queue = VecDeque::new();
    let mut output = Output::default();

    // Create placeholder for call to `main` function, and the last return that
    // ends the process, if executed.
    let main = output.instructions.push(Instruction::Panic);
    output.instructions.push(Instruction::Return);

    // Seed the queue from the root context.
    compile_context(fragments.root, &fragments.inner, &mut output, &mut queue);

    let mut compiler = Compiler {
        queue,
        output,
        functions: Functions::default(),
        fragments: &fragments.inner,
    };

    while let Some(unit) = compiler.queue.pop_front() {
        match unit {
            CompileUnit::Block {
                start,
                environment,
                address,
            } => {
                let start = compile_context(
                    start,
                    compiler.fragments,
                    &mut compiler.output,
                    &mut compiler.queue,
                );

                compiler.output.instructions.replace(
                    address,
                    Instruction::MakeClosure {
                        address: start,
                        environment,
                    },
                );
            }
            CompileUnit::Function(function) => {
                compile_function(
                    function,
                    compiler.fragments,
                    &mut compiler.output,
                    &mut compiler.queue,
                    &mut compiler.functions,
                );
            }
        }
    }

    if let Some(address) = compiler.functions.addresses_by_name.get("main") {
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
        compiler.output.instructions.replace(
            main,
            Instruction::CallFunction {
                address: *address,
                is_tail_call: true,
            },
        );
    }

    for call in compiler.output.placeholders.inner {
        let Some(address) =
            compiler.functions.addresses_by_name.get(&call.name)
        else {
            unreachable!(
                "Expecting function `{}` to exist. If it didn't, the previous \
                compilation step would not have generated the fragment that \
                caused us to assume that it does.",
                call.name,
            );
        };

        compiler.output.instructions.replace(
            call.address,
            Instruction::CallFunction {
                address: *address,
                is_tail_call: call.is_tail_call,
            },
        );
    }

    let bytecode = Bytecode {
        instructions: compiler.output.instructions,
        function_arguments: compiler.functions.arguments_by_address,
    };

    (bytecode, compiler.output.source_map)
}

struct Compiler<'r> {
    queue: VecDeque<CompileUnit>,
    output: Output,
    functions: Functions,
    fragments: &'r FragmentMap,
}

fn compile_function(
    function: Function,
    fragments: &FragmentMap,
    output: &mut Output,
    queue: &mut VecDeque<CompileUnit>,
    functions: &mut Functions,
) {
    let address = compile_context(function.start, fragments, output, queue);
    let arguments = function
        .arguments
        .into_iter()
        .filter_map(|pattern| match pattern {
            Pattern::Identifier { name } => Some(name),
            Pattern::Literal { .. } => {
                // The parameter list of a function is used to provide the
                // arguments to the function at runtime. But literal patterns
                // aren't relevant to the function itself. They are only used to
                // select which function to call in the first place.
                None
            }
        })
        .collect();

    functions.arguments_by_address.insert(address, arguments);
    functions.addresses_by_name.insert(function.name, address);
}

fn compile_context(
    start: FragmentId,
    fragments: &FragmentMap,
    output: &mut Output,
    queue: &mut VecDeque<CompileUnit>,
) -> InstructionAddress {
    let mut first_instruction = None;

    for fragment in fragments.iter_from(start) {
        let addr = compile_fragment(fragment, output, queue);
        first_instruction = first_instruction.or(addr);
    }

    let Some(first_instruction) = first_instruction else {
        unreachable!(
            "Must have generated at least one instruction for the block: the \
            return instruction. If this has not happened, the fragments have \
            somehow been missing a terminator."
        );
    };

    first_instruction
}

fn compile_fragment(
    fragment: &Fragment,
    output: &mut Output,
    queue: &mut VecDeque<CompileUnit>,
) -> Option<InstructionAddress> {
    let addr = match &fragment.payload {
        FragmentPayload::Expression { expression, .. } => {
            match expression {
                FragmentExpression::BindingDefinitions { names } => output
                    .generate_instruction(
                        Instruction::BindingsDefine {
                            names: names.clone(),
                        },
                        fragment.id(),
                    ),
                FragmentExpression::Block { start, environment } => {
                    // We are currently compiling a function or block (otherwise
                    // we wouldn't be encountering any expression), and the
                    // instructions for that will be executed linearly.
                    //
                    // Which means we can't just start compiling this block
                    // right now. Its instructions would go into the middle of
                    // those other instructions and mess everything up.
                    //
                    // What _should_ happen instead, is that the block is turned
                    // into a closure, that can be passed around as a value and
                    // called whenever so desired.
                    //
                    // So for now, let's just generate this instruction as a
                    // placeholder, to be replaced with another instruction that
                    // creates that closure, once we have everything in place to
                    // make that happen.
                    let address = output.generate_instruction(
                        Instruction::Panic,
                        fragment.id(),
                    );

                    // And to make it happen later, we need to put what we
                    // already have into a queue. Once whatever's currently
                    // being compiled is out of the way, we can process that.
                    queue.push_front(CompileUnit::Block {
                        start: *start,
                        environment: environment.clone(),
                        address,
                    });

                    address
                }
                FragmentExpression::Comment { .. } => {
                    return None;
                }
                FragmentExpression::ResolvedBinding { name } => output
                    .generate_instruction(
                        Instruction::BindingEvaluate { name: name.clone() },
                        fragment.id(),
                    ),
                FragmentExpression::ResolvedBuiltinFunction { name } => {
                    // Here we check for special built-in functions that are
                    // implemented differently, without making sure anywhere,
                    // that their name doesn't conflict with any user-defined
                    // functions.
                    //
                    // I think it's fine for now. This seems like a temporary
                    // hack anyway, while the language is not powerful enough to
                    // support real conditionals.
                    let instruction = if name == "return_if_non_zero" {
                        Instruction::ReturnIfNonZero
                    } else if name == "return_if_zero" {
                        Instruction::ReturnIfZero
                    } else {
                        Instruction::CallBuiltin { name: name.clone() }
                    };

                    output.generate_instruction(instruction, fragment.id())
                }
                FragmentExpression::ResolvedHostFunction { name } => output
                    .generate_instruction(
                        Instruction::CallBuiltin { name: name.clone() },
                        fragment.id(),
                    ),
                FragmentExpression::ResolvedUserFunction {
                    name,
                    is_tail_call,
                } => {
                    // We know that this expression refers to a user-defined
                    // function, but we might not have compiled that function
                    // yet.
                    //
                    // For now, just generate a placeholder that we can replace
                    // with the call later.
                    let address = output.generate_instruction(
                        Instruction::Panic,
                        fragment.id(),
                    );

                    // We can't leave it at that, however. We need to make sure
                    // this placeholder actually gets replace later, and we're
                    // doing that by adding it to this list.
                    output.placeholders.inner.push(CallToUserDefinedFunction {
                        name: name.clone(),
                        address,
                        is_tail_call: *is_tail_call,
                    });

                    address
                }
                FragmentExpression::UnresolvedIdentifier { name: _ } => output
                    .generate_instruction(Instruction::Panic, fragment.id()),
                FragmentExpression::Value(value) => output
                    .generate_instruction(
                        Instruction::Push { value: *value },
                        fragment.id(),
                    ),
            }
        }
        FragmentPayload::Function(function) => {
            queue.push_back(CompileUnit::Function(function.clone()));
            return None;
        }
        FragmentPayload::Terminator => {
            output.generate_instruction(Instruction::Return, fragment.id())
        }
    };

    Some(addr)
}

#[derive(Default)]
struct Output {
    instructions: Instructions,
    placeholders: Placeholders,
    source_map: SourceMap,
}

impl Output {
    fn generate_instruction(
        &mut self,
        instruction: Instruction,
        fragment_id: FragmentId,
    ) -> InstructionAddress {
        let addr = self.instructions.push(instruction);
        self.source_map.define_mapping(addr, fragment_id);
        addr
    }
}

#[derive(Default)]
struct Functions {
    arguments_by_address: BTreeMap<InstructionAddress, Vec<String>>,
    addresses_by_name: BTreeMap<String, InstructionAddress>,
}

enum CompileUnit {
    Block {
        start: FragmentId,
        environment: BTreeSet<String>,
        address: InstructionAddress,
    },
    Function(Function),
}
