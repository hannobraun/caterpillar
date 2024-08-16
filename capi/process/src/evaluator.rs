use std::collections::BTreeMap;

use crate::{
    builtins::builtin_by_name, function::Pattern, Effect, Function,
    Instruction, Instructions, Stack,
};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Evaluator {
    pub stack: Stack,
    pub closures: BTreeMap<u32, Function>,
    pub next_closure: u32,
}

impl Evaluator {
    pub fn step(&mut self, instructions: &Instructions) -> Result<(), Effect> {
        if self.stack.no_frames_left() {
            return Ok(());
        }

        let Some(addr) = self.stack.take_next_instruction() else {
            return Ok(());
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
            Instruction::CallBuiltin { name } => match builtin_by_name(name) {
                Some(f) => {
                    f(&mut self.stack)?;
                }
                None => {
                    return Err(Effect::UnknownBuiltin);
                }
            },
            Instruction::CallFunction {
                function,
                is_tail_call,
            } => {
                let mut any_member_matched = false;

                for branch in &function.branches {
                    let mut used_operands = Vec::new();
                    let mut argument_operands = Vec::new();
                    let mut bound_arguments = Vec::new();

                    let mut member_matches = true;
                    for parameter in branch.parameters.iter().rev() {
                        let operand = self.stack.pop_operand()?;
                        used_operands.push(operand);

                        match parameter {
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

                        self.stack.next_instruction = branch.start;
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
            Instruction::Eval { is_tail_call } => {
                // This duplicates code from other places, which is unfortunate,
                // but works for now.
                //
                // See implementation note on `Instruction::Eval` for context on
                // this.

                let function = {
                    let index = self.stack.pop_operand()?;
                    let index = index.to_u32();

                    self.closures
                        .remove(&index)
                        .ok_or(Effect::InvalidFunction)?
                };

                let mut any_member_matched = false;

                for branch in &function.branches {
                    let mut used_operands = Vec::new();
                    let mut argument_operands = Vec::new();
                    let mut bound_arguments = Vec::new();

                    let mut member_matches = true;
                    for parameter in branch.parameters.iter().rev() {
                        let operand = self.stack.pop_operand()?;
                        used_operands.push(operand);

                        match parameter {
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

                        self.stack
                            .bindings_mut()
                            .expect(
                                "Currently executing; stack frame must exist",
                            )
                            .extend(function.environment);

                        self.stack.next_instruction = branch.start;
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
                branches,
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
                self.closures.insert(
                    index,
                    Function {
                        branches: branches.clone(),
                        environment,
                    },
                );

                self.stack.push_operand(index);
            }
            Instruction::Push { value } => {
                self.stack.push_operand(*value);
            }
            Instruction::Return => {
                self.stack.pop_frame();
            }
            Instruction::TriggerEffect { effect } => {
                return Err(*effect);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        evaluator::Evaluator, stack::StackElement, Branch, Function,
        Instruction, InstructionAddress, Instructions, Pattern, Value,
    };

    #[test]
    fn call_cluster() {
        let mut evaluator = Evaluator::default();
        evaluator.stack.push_operand(1);
        evaluator.stack.push_operand(2);

        let mut instructions = Instructions::default();
        instructions.push(Instruction::CallFunction {
            function: Function {
                branches: vec![
                    Branch {
                        parameters: vec![
                            Pattern::Literal {
                                value: Value::from(0),
                            },
                            Pattern::Identifier {
                                name: String::from("x"),
                            },
                        ],
                        start: InstructionAddress { index: 1 },
                    },
                    Branch {
                        parameters: vec![
                            Pattern::Literal {
                                value: Value::from(1),
                            },
                            Pattern::Identifier {
                                name: String::from("x"),
                            },
                        ],
                        start: InstructionAddress { index: 2 },
                    },
                    Branch {
                        parameters: vec![
                            Pattern::Literal {
                                value: Value::from(2),
                            },
                            Pattern::Identifier {
                                name: String::from("x"),
                            },
                        ],
                        start: InstructionAddress { index: 3 },
                    },
                ],
                environment: BTreeMap::new(),
            },
            is_tail_call: true,
        });

        evaluator.step(&instructions).unwrap();
        assert!(!evaluator.stack.no_frames_left());

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
