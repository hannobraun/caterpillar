use std::collections::BTreeMap;

use crate::{
    function::Pattern, Effect, Function, Instruction, InstructionAddress,
    Instructions, Stack,
};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Evaluator {
    pub stack: Stack,
    pub next_instruction: InstructionAddress,

    closures: BTreeMap<u32, Function>,
    next_closure: u32,
}

impl Evaluator {
    pub fn active_instructions(
        &self,
    ) -> impl Iterator<Item = InstructionAddress> + '_ {
        self.stack.return_addresses().chain([self.next_instruction])
    }

    pub fn step(&mut self, instructions: &Instructions) -> Result<(), Effect> {
        if self.stack.no_frames_left() {
            return Ok(());
        }

        self.step_inner(instructions)
    }

    fn step_inner(
        &mut self,
        instructions: &Instructions,
    ) -> Result<(), Effect> {
        let address = self.next_instruction;
        self.next_instruction.index += 1;

        let instruction = instructions
            .get(&address)
            .expect("Expected instruction referenced on stack to exist");

        match instruction {
            Instruction::AddS8 => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_i8()?;
                let b = b.to_i8()?;

                let Some(c) = a.checked_add(b) else {
                    return Err(Effect::IntegerOverflow);
                };

                self.stack.push_operand(c);
            }
            Instruction::AddS32 => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_i32();
                let b = b.to_i32();

                let Some(c) = a.checked_add(b) else {
                    return Err(Effect::IntegerOverflow);
                };

                self.stack.push_operand(c);
            }
            Instruction::AddU8 => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_u8()?;
                let b = b.to_u8()?;

                let Some(c) = a.checked_add(b) else {
                    return Err(Effect::IntegerOverflow);
                };

                self.stack.push_operand(c);
            }
            Instruction::AddU8Wrap => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_u8()?;
                let b = b.to_u8()?;

                let c = a.wrapping_add(b);
                self.stack.push_operand(c);
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
                            self.stack.push_frame(self.next_instruction)?;
                        }

                        self.next_instruction = branch.start;
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
            Instruction::ConvertS32ToS8 => {
                let v = self.stack.pop_operand()?;

                let v = v.to_i32();
                let v: i8 = v.try_into()?;

                self.stack.push_operand(v);
            }
            Instruction::Copy => {
                let a = self.stack.pop_operand()?;

                self.stack.push_operand(a);
                self.stack.push_operand(a);
            }
            Instruction::DivS32 => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_i32();
                let b = b.to_i32();

                if b == 0 {
                    return Err(Effect::DivideByZero);
                }
                let Some(c) = a.checked_div(b) else {
                    // Can't be divide by zero. Already handled that.
                    return Err(Effect::IntegerOverflow);
                };

                self.stack.push_operand(c);
            }
            Instruction::DivU8 => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_u8()?;
                let b = b.to_u8()?;

                if b == 0 {
                    return Err(Effect::DivideByZero);
                }
                let Some(c) = a.checked_div(b) else {
                    // Can't be divide by zero. Already handled that.
                    return Err(Effect::IntegerOverflow);
                };

                self.stack.push_operand(c);
            }
            Instruction::Drop => {
                self.stack.pop_operand()?;
            }
            Instruction::Eq => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let c = if a.0 == b.0 { 1 } else { 0 };

                self.stack.push_operand(c);
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
                            self.stack.push_frame(self.next_instruction)?;
                        }

                        self.stack
                            .bindings_mut()
                            .expect(
                                "Currently executing; stack frame must exist",
                            )
                            .extend(function.environment);

                        self.next_instruction = branch.start;
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
            Instruction::GreaterS8 => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_i8()?;
                let b = b.to_i8()?;

                let c = if a > b { 1 } else { 0 };

                self.stack.push_operand(c);
            }
            Instruction::GreaterS32 => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_i32();
                let b = b.to_i32();

                let c = if a > b { 1 } else { 0 };

                self.stack.push_operand(c);
            }
            Instruction::GreaterU8 => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_u8()?;
                let b = b.to_u8()?;

                let c = if a > b { 1 } else { 0 };

                self.stack.push_operand(c);
            }
            Instruction::LogicalAnd => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let c = if a.0 == [0; 4] || b.0 == [0; 4] { 0 } else { 1 };

                self.stack.push_operand(c);
            }
            Instruction::LogicalNot => {
                let a = self.stack.pop_operand()?;

                let b = if a.0 == [0; 4] { 1 } else { 0 };
                self.stack.push_operand(b);
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
                                "Binding `{name}`, from the environment of an \
                                anonymous function, does not exist.\n\
                                \n\
                                A binding that is part of a function's \
                                environment, must exist in the parent scope \
                                of that function.",
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
            Instruction::MulS32 => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_i32();
                let b = b.to_i32();

                let Some(c) = a.checked_mul(b) else {
                    return Err(Effect::IntegerOverflow);
                };

                self.stack.push_operand(c);
            }
            Instruction::MulU8Wrap => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_u8()?;
                let b = b.to_u8()?;

                let c = a.wrapping_mul(b);
                self.stack.push_operand(c);
            }
            Instruction::NegS32 => {
                let a = self.stack.pop_operand()?;

                let a = a.to_i32();

                if a == i32::MIN {
                    return Err(Effect::IntegerOverflow);
                }
                let b = -a;

                self.stack.push_operand(b);
            }
            Instruction::Push { value } => {
                self.stack.push_operand(*value);
            }
            Instruction::RemainderS32 => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_i32();
                let b = b.to_i32();

                if b == 0 {
                    return Err(Effect::DivideByZero);
                }
                let c = a % b;

                self.stack.push_operand(c);
            }
            Instruction::Return => {
                if let Some(return_address) = self.stack.pop_frame() {
                    self.next_instruction = return_address;
                }
            }
            Instruction::SubS32 => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_i32();
                let b = b.to_i32();

                let Some(c) = a.checked_sub(b) else {
                    return Err(Effect::IntegerOverflow);
                };

                self.stack.push_operand(c);
            }
            Instruction::SubU8 => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_u8()?;
                let b = b.to_u8()?;

                let Some(c) = a.checked_sub(b) else {
                    return Err(Effect::IntegerOverflow);
                };

                self.stack.push_operand(c);
            }
            Instruction::SubU8Wrap => {
                let b = self.stack.pop_operand()?;
                let a = self.stack.pop_operand()?;

                let a = a.to_u8()?;
                let b = b.to_u8()?;

                let c = a.wrapping_sub(b);
                self.stack.push_operand(c);
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

        assert_eq!(evaluator.next_instruction.index, 2);
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
