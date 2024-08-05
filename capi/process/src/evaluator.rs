use crate::{
    builtins::builtin, stack::PushStackFrameError, Bytecode, CoreEffect,
    Effect, Host, Instruction, Stack, Value,
};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Evaluator {
    pub stack: Stack,
    pub next_closure: u32,
}

impl Evaluator {
    pub fn step<H: Host>(
        &mut self,
        bytecode: &Bytecode,
    ) -> Result<EvaluatorState, Effect<H::Effect>> {
        let Some(addr) = self.stack.take_next_instruction() else {
            return Ok(EvaluatorState::Finished);
        };

        let instruction = bytecode
            .instructions
            .get(&addr)
            .expect("Expected instruction referenced on stack to exist");

        match instruction {
            Instruction::BindingEvaluate { name } => {
                let Some(bindings) = self.stack.bindings() else {
                    unreachable!(
                        "Can't access bindings, but we're currently executing. \
                        An active stack frame, and therefore bindings, must \
                        exist."
                    );
                };
                let Some(value) = bindings.get(name).copied() else {
                    unreachable!(
                        "Can't find binding `{name}`, but instruction that \
                        evaluates bindings should only be generated for \
                        bindings that exist.\n\
                        \n\
                        Current stack:\n\
                        {:#?}",
                        self.stack,
                    );
                };
                self.stack.push_operand(value);
            }
            Instruction::BindingsDefine { names } => {
                for name in names.iter().rev() {
                    let value = self.stack.pop_operand()?;
                    self.stack.define_binding(name.clone(), value);
                }

                if self.stack.operands_in_current_stack_frame().count() > 0 {
                    return Err(Effect::Core(
                        CoreEffect::BindingLeftValuesOnStack,
                    ));
                }
            }
            Instruction::CallBuiltin { name } => {
                match (H::function(name), builtin(name)) {
                    (Some(_), Some(_)) => {
                        // As of this writing, users can not define custom
                        // hosts, so the damage of this being a runtime panic is
                        // limited. But ideally, it should be detected at
                        // compile-time.
                        panic!(
                        "`{name}` refers to both a built-in function and a \
                        host function.\n"
                    );
                    }
                    (Some(f), None) => f(&mut self.stack)?,
                    (None, Some(f)) => {
                        f(&mut self.stack, &bytecode.instructions)?
                    }
                    (None, None) => {
                        return Err(Effect::Core(CoreEffect::UnknownBuiltin {
                            name: name.clone(),
                        }));
                    }
                }
            }
            Instruction::CallFunction { address, .. } => {
                let function =
                    bytecode.functions.get(address).cloned().unwrap();

                let arguments = function
                    .arguments
                    .into_iter()
                    .rev()
                    .map(|name| {
                        let value = self.stack.pop_operand()?;
                        Ok((name, value))
                    })
                    .collect::<Result<Vec<_>, PushStackFrameError>>()?;
                let is_tail_call = {
                    let next_instruction = bytecode
                        .instructions
                        .get(&self.stack.next_instruction)
                        .expect(
                            "Expected instruction referenced on stack to exist",
                        );

                    *next_instruction == Instruction::Return
                };

                if is_tail_call {
                    self.stack.reuse_frame(arguments);
                } else {
                    self.stack.push_frame(arguments)?;
                }

                self.stack.next_instruction = function.start;
            }
            Instruction::MakeClosure {
                address,
                environment,
            } => {
                let Some(bindings) = self.stack.bindings() else {
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
                                "Binding that is specified in block \
                                environment must exist."
                            );
                        };

                        (name, value)
                    })
                    .collect();

                let index = {
                    let next_closure = self.next_closure;
                    self.next_closure += 1;
                    next_closure
                };
                self.stack.closures.insert(index, (*address, environment));

                self.stack.push_operand(index);
            }
            Instruction::Push { value } => self.stack.push_operand(*value),
            Instruction::Return => {
                self.stack.pop_frame();
            }
            Instruction::ReturnIfNonZero => {
                let value = self.stack.pop_operand()?;
                if value != Value([0, 0, 0, 0]) {
                    self.stack.pop_frame();
                }
            }
            Instruction::ReturnIfZero => {
                let value = self.stack.pop_operand()?;
                if value == Value([0, 0, 0, 0]) {
                    self.stack.pop_frame();
                }
            }
            Instruction::Panic => return Err(Effect::Core(CoreEffect::Panic)),
        }

        Ok(EvaluatorState::Running)
    }
}

#[derive(Debug)]
#[must_use]
pub enum EvaluatorState {
    Running,
    Finished,
}
