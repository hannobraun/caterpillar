use crate::{
    builtins::builtin, instructions::Pattern, Effect, Host, HostEffect,
    Instruction, Instructions, Stack, Value,
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
        instructions: &Instructions,
    ) -> Result<EvaluatorState, Effect> {
        let Some(addr) = self.stack.take_next_instruction() else {
            return Ok(EvaluatorState::Finished);
        };

        let instruction = instructions
            .get(&addr)
            .expect("Expected instruction referenced on stack to exist");

        match instruction {
            Instruction::AssertBindingLeftNoOperands => {
                if self.stack.operands_in_current_stack_frame().count() > 0 {
                    return Err(Effect::BindingLeftValuesOnStack);
                }
            }
            Instruction::Bind { name } => {
                let value = self.stack.pop_operand()?;
                self.stack.define_binding(name.clone(), value);
            }
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
                    (Some(effect), None) => {
                        self.stack.push_operand(effect.to_number());
                        return Err(Effect::Host);
                    }
                    (None, Some(f)) => {
                        f(&mut self.stack, instructions)?;
                    }
                    (None, None) => {
                        return Err(Effect::UnknownBuiltin);
                    }
                }
            }
            Instruction::CallCluster {
                cluster,
                is_tail_call,
            } => {
                let mut any_member_matched = false;

                for (arguments, address) in cluster {
                    let mut used_operands = Vec::new();
                    let mut argument_operands = Vec::new();
                    let mut bound_arguments = Vec::new();

                    let mut member_matches = true;
                    for argument in arguments.iter().rev() {
                        let operand = self.stack.pop_operand()?;
                        used_operands.push(operand);

                        match argument {
                            Pattern::Identifier { name } => {
                                bound_arguments.push((name.clone(), operand));
                                argument_operands.push(operand);
                            }
                            Pattern::Literal { value } => {
                                member_matches &= *value == operand;
                            }
                        }
                    }

                    if member_matches {
                        for value in argument_operands.into_iter().rev() {
                            self.stack.push_operand(value);
                        }

                        if *is_tail_call {
                            self.stack.reuse_frame();
                        } else {
                            self.stack.push_frame()?;
                        }

                        self.stack.next_instruction = *address;
                        any_member_matched = true;

                        break;
                    } else {
                        for value in used_operands.into_iter().rev() {
                            self.stack.push_operand(value);
                        }
                    }
                }

                if !any_member_matched {
                    return Err(Effect::NoMatch);
                }
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
            Instruction::Push { value } => {
                self.stack.push_operand(*value);
            }
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
            Instruction::Panic => {
                return Err(Effect::Panic);
            }
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

#[cfg(test)]
mod tests {
    use crate::{
        evaluator::{Evaluator, EvaluatorState},
        stack::StackElement,
        Instruction, InstructionAddress, Instructions, NoHost, Pattern, Value,
    };

    #[test]
    fn call_cluster() {
        let mut evaluator = Evaluator::default();
        evaluator.stack.push_operand(1);
        evaluator.stack.push_operand(2);

        let mut instructions = Instructions::default();
        instructions.push(Instruction::CallCluster {
            cluster: vec![
                (
                    vec![
                        Pattern::Literal {
                            value: Value::from(0),
                        },
                        Pattern::Identifier {
                            name: String::from("x"),
                        },
                    ],
                    InstructionAddress { index: 1 },
                ),
                (
                    vec![
                        Pattern::Literal {
                            value: Value::from(1),
                        },
                        Pattern::Identifier {
                            name: String::from("x"),
                        },
                    ],
                    InstructionAddress { index: 2 },
                ),
                (
                    vec![
                        Pattern::Literal {
                            value: Value::from(2),
                        },
                        Pattern::Identifier {
                            name: String::from("x"),
                        },
                    ],
                    InstructionAddress { index: 3 },
                ),
            ],
            is_tail_call: true,
        });

        let EvaluatorState::Running =
            evaluator.step::<NoHost>(&instructions).unwrap()
        else {
            panic!("Did not expect evaluation to be finished.");
        };

        assert_eq!(evaluator.stack.next_instruction.index, 2);
        assert!(matches!(
            evaluator.stack.into_inner().as_slice(),
            &[
                StackElement::StartMarker,
                StackElement::Bindings(_),
                StackElement::Operand(Value([2, 0, 0, 0]))
            ]
        ));
    }
}
