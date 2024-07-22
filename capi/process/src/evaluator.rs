use crate::{
    builtins::{self, Builtin},
    Bytecode, CoreEffect, Effect, Host, Instruction, Stack, Value,
};

pub fn evaluate<H: Host>(
    bytecode: &Bytecode,
    stack: &mut Stack,
) -> Result<EvaluatorState, Effect<H::Effect>> {
    let Some(addr) = stack.take_next_instruction() else {
        return Ok(EvaluatorState::Finished);
    };

    let instruction = bytecode
        .instructions
        .get(&addr)
        .expect("Expected instruction referenced on stack to exist");

    match instruction {
        Instruction::BindingEvaluate { name } => {
            let Some(bindings) = stack.bindings() else {
                unreachable!(
                    "Can't access bindings, but we're currently executing. An \
                    active stack frame, and therefore bindings, must exist."
                );
            };
            let Some(value) = bindings.get(name).copied() else {
                unreachable!(
                    "Can't find binding `{name}`, but instruction that \
                    evaluates bindings should only be generated for bindings \
                    that exist.\n\
                    \n\
                    Current stack:\n\
                    {stack:#?}"
                );
            };
            stack.push_operand(value);
        }
        Instruction::BindingsDefine { names } => {
            for name in names.iter().rev() {
                let value = stack.pop_operand()?;
                stack.define_binding(name.clone(), value);
            }

            let Some(operands) = stack.operands() else {
                unreachable!(
                    "Can't access operands, but we're currently executing. An \
                    active stack frame, and therefore operands, must exist."
                );
            };

            if !operands.is_empty() {
                return Err(Effect::Core(CoreEffect::BindingLeftValuesOnStack));
            }
        }
        Instruction::CallBuiltin { name } => {
            if let Some(f) = H::function(name) {
                f(stack)?
            } else {
                let builtin = match name.as_str() {
                    "add" => Some(builtins::add as Builtin),
                    "add_wrap_unsigned" => {
                        Some(builtins::add_wrap_unsigned as _)
                    }
                    "brk" => Some(builtins::brk as _),
                    "copy" => Some(builtins::copy as _),
                    "div" => Some(builtins::div as _),
                    "drop" => Some(builtins::drop as _),
                    "eq" => Some(builtins::eq as _),
                    "eval" => Some(builtins::eval as _),
                    "greater" => Some(builtins::greater as _),
                    "if" => Some(builtins::if_ as _),
                    "mul" => Some(builtins::mul as _),
                    "neg" => Some(builtins::neg as _),
                    "remainder" => Some(builtins::remainder as _),
                    "sub" => Some(builtins::sub as _),

                    _ => None,
                };

                match builtin {
                    Some(builtin) => {
                        builtin(stack, &bytecode.instructions)?;
                    }
                    None => {
                        return Err(Effect::Core(CoreEffect::UnknownBuiltin {
                            name: name.clone(),
                        }));
                    }
                }
            }
        }
        Instruction::CallFunction { name } => {
            let function = bytecode.functions.get(name).cloned().unwrap();
            stack.push_frame(function, &bytecode.instructions)?;
        }
        Instruction::MakeClosure { addr, environment } => {
            let Some(bindings) = stack.bindings() else {
                unreachable!(
                    "We're currently executing. A stack frame, and thus \
                    bindings, must exist."
                );
            };

            let environment = environment
                .iter()
                .cloned()
                .map(|name| {
                    let Some(value) = bindings.get(&name).cloned() else {
                        unreachable!(
                            "Binding that is specified in block environment \
                            must exist."
                        );
                    };

                    (name, value)
                })
                .collect();

            let index = {
                let next_closure = stack.next_closure;
                stack.next_closure += 1;
                next_closure
            };
            stack.closures.insert(index, (*addr, environment));

            stack.push_operand(Value(index.to_le_bytes()));
        }
        Instruction::Push { value } => stack.push_operand(*value),
        Instruction::Return => {
            stack
                .pop_frame()
                .expect("Currently executing; stack can't be empty");
        }
        Instruction::ReturnIfNonZero => {
            let value = stack.pop_operand()?;
            if value != Value([0, 0, 0, 0]) {
                stack
                    .pop_frame()
                    .expect("Currently executing; stack can't be empty");
            }
        }
        Instruction::ReturnIfZero => {
            let value = stack.pop_operand()?;
            if value == Value([0, 0, 0, 0]) {
                stack
                    .pop_frame()
                    .expect("Currently executing; stack can't be empty");
            }
        }
        Instruction::Panic => return Err(Effect::Core(CoreEffect::Panic)),
    }

    Ok(EvaluatorState::Running)
}

#[derive(Debug)]
#[must_use]
pub enum EvaluatorState {
    Running,
    Finished,
}
