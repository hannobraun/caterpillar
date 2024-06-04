use std::collections::BTreeMap;

use crate::{
    builtins::BuiltinEffect,
    call_stack::{CallStack, CallStackOverflow},
    data_stack::StackUnderflow,
    instructions::Instruction,
    runtime::Function,
    InstructionAddress, Value,
};

use super::{builtins, code::Code, data_stack::DataStack};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Evaluator {
    code: Code,
    call_stack: CallStack,
    data_stack: DataStack,
    bindings: BTreeMap<String, Value>,
}

impl Evaluator {
    pub fn new(code: Code, entry: Function) -> Self {
        Self {
            code,
            call_stack: CallStack::new(entry),
            data_stack: DataStack::default(),
            bindings: BTreeMap::default(),
        }
    }

    pub fn next_instruction(&self) -> InstructionAddress {
        self.call_stack.next().unwrap()
    }

    pub fn call_stack(&self) -> &CallStack {
        &self.call_stack
    }

    pub fn data_stack(&self) -> &DataStack {
        &self.data_stack
    }

    pub fn reset(&mut self, entry: Function) {
        self.call_stack = CallStack::new(entry);
        self.data_stack.clear();
    }

    pub fn push(&mut self, values: impl IntoIterator<Item = Value>) {
        for value in values {
            self.data_stack.push(value);
        }
    }

    pub fn step(&mut self) -> EvaluatorState {
        let address = loop {
            let Some(mut function) = self.call_stack.pop() else {
                return EvaluatorState::Finished;
            };

            if let Some(address) = function.pop_front() {
                self.call_stack.push(function).expect(
                    "Just popped a stack frame; pushing one can't overflow",
                );

                break address;
            }
        };

        let instruction = self.code.instructions.get(&address);

        match instruction {
            Instruction::BindingDefine { name } => {
                let value = match self.data_stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return EvaluatorState::Effect {
                            effect: EvaluatorEffect::StackError(err),
                            address,
                        }
                    }
                };
                self.bindings.insert(name.clone(), value);
            }
            Instruction::BindingEvaluate { name } => {
                let value = self.bindings.get(name).copied().expect(
                    "Binding instruction only generated for existing bindings",
                );
                self.data_stack.push(value);
            }
            Instruction::CallBuiltin { name } => {
                let result = match name.as_str() {
                    "add" => builtins::add(&mut self.data_stack),
                    "add_wrap_unsigned" => {
                        builtins::add_wrap_unsigned(&mut self.data_stack)
                    }
                    "copy" => builtins::copy(&mut self.data_stack),
                    "div" => builtins::div(&mut self.data_stack),
                    "drop" => builtins::drop(&mut self.data_stack),
                    "eq" => builtins::eq(&mut self.data_stack),
                    "greater" => builtins::greater(&mut self.data_stack),
                    "load" => builtins::load(&mut self.data_stack),
                    "mul" => builtins::mul(&mut self.data_stack),
                    "neg" => builtins::neg(&mut self.data_stack),
                    "place" => builtins::place(&mut self.data_stack),
                    "read_input" => builtins::read_input(),
                    "read_random" => builtins::read_random(),
                    "remainder" => builtins::remainder(&mut self.data_stack),
                    "store" => builtins::store(&mut self.data_stack),
                    "sub" => builtins::sub(&mut self.data_stack),
                    "submit_frame" => builtins::submit_frame(),
                    "take" => builtins::take(&mut self.data_stack),
                    "write_tile" => builtins::write_tile(&mut self.data_stack),
                    _ => {
                        return EvaluatorState::Effect {
                            effect: EvaluatorEffect::UnknownBuiltin {
                                name: name.clone(),
                            },
                            address,
                        }
                    }
                };

                // This is a bit weird. An error is an effect, and effects can
                // be returned as a `Result::Ok` by the builtins. But error by
                // itself can also be returned as a `Result::Err`.
                //
                // This enables builtins to to stack operations using `?`
                // internally, without requiring effects to always be returned
                // as errors, which they aren't per se.
                //
                // Anyway, here we deal with this situation by unifying both
                // variants.
                let effect = match result {
                    Ok(effect) => effect,
                    Err(err) => Some(BuiltinEffect::Error(err)),
                };

                if let Some(effect) = effect {
                    return EvaluatorState::Effect {
                        effect: EvaluatorEffect::Builtin(effect),
                        address,
                    };
                }
            }
            Instruction::CallFunction { name } => {
                let function = self.code.functions.get(name).cloned().unwrap();

                if let Err(err) = self.call_stack.push(function) {
                    return EvaluatorState::Effect {
                        effect: EvaluatorEffect::CallStack(err),
                        address,
                    };
                }
            }
            Instruction::Push { value } => self.data_stack.push(*value),
            Instruction::ReturnIfNonZero => {
                let a = self.data_stack.pop().unwrap();

                if a != Value(0) {
                    // Here we just duplicate the code from the regular return
                    // instruction above, which isn't great. Getting rid of the
                    // duplication completely isn't easy though, due to the
                    // `return`. And since I suspect that this instruction is
                    // temporary, until the language grows more features, I'm
                    // inclined to just leave this be.

                    self.call_stack.pop();
                }
            }
            Instruction::ReturnIfZero => {
                let a = self.data_stack.pop().unwrap();

                if a == Value(0) {
                    // Here we just duplicate the code from the regular return
                    // instruction above, which isn't great. Getting rid of the
                    // duplication completely isn't easy though, due to the
                    // `return`. And since I suspect that this instruction is
                    // temporary, until the language grows more features, I'm
                    // inclined to just leave this be.

                    self.call_stack.pop();
                }
            }
        }

        EvaluatorState::Running {
            just_executed: address,
        }
    }
}

#[derive(Debug)]
#[must_use]
pub enum EvaluatorState {
    Running {
        just_executed: InstructionAddress,
    },
    Finished,
    Effect {
        effect: EvaluatorEffect,
        address: InstructionAddress,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum EvaluatorEffect {
    Builtin(BuiltinEffect),
    CallStack(CallStackOverflow),
    StackError(StackUnderflow),
    UnknownBuiltin { name: String },
}
