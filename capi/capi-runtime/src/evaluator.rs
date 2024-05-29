use std::collections::BTreeMap;

use crate::{
    builtins::BuiltinEffect, data_stack::StackUnderflow,
    instructions::Instruction, InstructionAddress, Value,
};

use super::{builtins, code::Code, data_stack::DataStack};

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Evaluator {
    pub code: Code,
    pub next_instruction: InstructionAddress,
    pub call_stack: Vec<InstructionAddress>,
    pub data_stack: DataStack,
    pub bindings: BTreeMap<String, Value>,
}

impl Evaluator {
    pub fn update(&mut self, code: Code, entry: InstructionAddress) {
        self.code = code;
        self.next_instruction = entry;
    }

    pub fn reset(&mut self, entry: InstructionAddress) {
        self.call_stack.clear();
        self.data_stack.clear();
        self.next_instruction = entry;
    }

    pub fn step(&mut self) -> EvaluatorState {
        let current_instruction = self.next_instruction;
        self.next_instruction.increment();

        let instruction = self.code.instructions.get(&current_instruction);

        match instruction {
            Instruction::BindingDefine { name } => {
                let value = match self.data_stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return EvaluatorState::Effect {
                            effect: EvaluatorEffect::StackError(err),
                            address: current_instruction,
                        }
                    }
                };
                self.bindings.insert(name.clone(), value);
            }
            Instruction::BindingEvaluate { name } => {
                let value = self.bindings.get(name).copied().unwrap();
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
                    "greater" => builtins::greater(&mut self.data_stack),
                    "load" => builtins::load(&mut self.data_stack),
                    "mul" => builtins::mul(&mut self.data_stack),
                    "neg" => builtins::neg(&mut self.data_stack),
                    "place" => builtins::place(&mut self.data_stack),
                    "read_input" => builtins::read_input(),
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
                            address: current_instruction,
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
                        address: current_instruction,
                    };
                }
            }
            Instruction::CallFunction { name } => {
                let address = self.code.symbols.resolve_name(name);
                self.call_stack.push(self.next_instruction);
                self.next_instruction = address;
            }
            Instruction::Push { value } => self.data_stack.push(*value),
            Instruction::Return => {
                let Some(return_address) = self.call_stack.pop() else {
                    return EvaluatorState::Finished;
                };

                self.next_instruction = return_address;
            }
            Instruction::ReturnIfNonZero => {
                let a = self.data_stack.pop().unwrap();

                if a != Value(0) {
                    // Here we just duplicate the code from the regular return
                    // instruction above, which isn't great. Getting rid of the
                    // duplication completely isn't easy though, due to the
                    // `return`. And since I suspect that this instruction is
                    // temporary, until the language grows more features, I'm
                    // inclined to just leave this be.

                    let Some(return_address) = self.call_stack.pop() else {
                        return EvaluatorState::Finished;
                    };

                    self.next_instruction = return_address;
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

                    let Some(return_address) = self.call_stack.pop() else {
                        return EvaluatorState::Finished;
                    };

                    self.next_instruction = return_address;
                }
            }
        }

        EvaluatorState::Running {
            just_executed: current_instruction,
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
    StackError(StackUnderflow),
    UnknownBuiltin { name: String },
}
