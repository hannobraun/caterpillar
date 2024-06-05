use crate::{
    builtins::BuiltinEffect,
    call_stack::{Bindings, CallStack, CallStackOverflow},
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
    bindings: Bindings,
}

impl Evaluator {
    pub fn new(code: Code, entry: Function) -> Self {
        Self {
            code,
            call_stack: CallStack::new(entry),
            data_stack: DataStack::default(),
            bindings: Bindings::default(),
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

    pub fn step(&mut self) -> Result<EvaluatorState, EvaluatorEffect> {
        let address = loop {
            let Some(mut frame) = self.call_stack.pop() else {
                return Ok(EvaluatorState::Finished);
            };

            if let Some(address) = frame.function.pop_front() {
                // Don't put the stack frame back, if it is empty. This is
                // essentially tail call optimization.
                //
                // This will lead to trouble, if the last instruction in the
                // function (the one whose address we just acquired, and are
                // about to execute) is an explicit return instruction. Those
                // will pop *another* stack frame, which is one too many.
                //
                // I've decided not to address that, for the moment. First, that
                // is a weird pattern anyway, and doesn't really make sense in
                // the language. Second, explicit return instructions are a
                // stopgap anyway, and will go away once we have anonymous
                // functions that we can use for more advanced control flow.
                if !frame.function.is_empty() {
                    self.call_stack.push(frame).expect(
                        "Just popped a stack frame; pushing one can't overflow",
                    );
                }

                break address;
            }
        };

        let instruction = self.code.instructions.get(&address).clone();
        let evaluate_result = evaluate_instruction(
            instruction,
            &self.code,
            &mut self.data_stack,
            &mut self.call_stack,
            &mut self.bindings,
        );
        match evaluate_result {
            Ok(Some(call_stack_update)) => match call_stack_update {},
            Ok(None) => {}
            Err(effect) => {
                return Ok(EvaluatorState::Effect(EvaluatorEffect {
                    effect,
                    address,
                }));
            }
        }

        Ok(EvaluatorState::Running {
            just_executed: address,
        })
    }
}

#[derive(Debug)]
#[must_use]
pub enum EvaluatorState {
    Running { just_executed: InstructionAddress },
    Finished,
    Effect(EvaluatorEffect),
}

#[derive(Debug)]
pub struct EvaluatorEffect {
    pub effect: EvaluatorEffectKind,
    pub address: InstructionAddress,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum EvaluatorEffectKind {
    Builtin(BuiltinEffect),
    CallStack(CallStackOverflow),
    StackError(StackUnderflow),
    UnknownBuiltin { name: String },
}

impl From<CallStackOverflow> for EvaluatorEffectKind {
    fn from(err: CallStackOverflow) -> Self {
        Self::CallStack(err)
    }
}

impl From<StackUnderflow> for EvaluatorEffectKind {
    fn from(err: StackUnderflow) -> Self {
        Self::StackError(err)
    }
}

fn evaluate_instruction(
    instruction: Instruction,
    code: &Code,
    data_stack: &mut DataStack,
    call_stack: &mut CallStack,
    bindings: &mut Bindings,
) -> Result<Option<CallStackUpdate>, EvaluatorEffectKind> {
    match instruction {
        Instruction::BindingDefine { name } => {
            let value = data_stack.pop()?;
            bindings.insert(name, value);
        }
        Instruction::BindingEvaluate { name } => {
            let value = bindings.get(&name).copied().expect(
                "Binding instruction only generated for existing bindings",
            );
            data_stack.push(value);
        }
        Instruction::CallBuiltin { name } => {
            let result = match name.as_str() {
                "add" => builtins::add(data_stack),
                "add_wrap_unsigned" => builtins::add_wrap_unsigned(data_stack),
                "copy" => builtins::copy(data_stack),
                "div" => builtins::div(data_stack),
                "drop" => builtins::drop(data_stack),
                "eq" => builtins::eq(data_stack),
                "greater" => builtins::greater(data_stack),
                "load" => builtins::load(data_stack),
                "mul" => builtins::mul(data_stack),
                "neg" => builtins::neg(data_stack),
                "place" => builtins::place(data_stack),
                "read_input" => builtins::read_input(),
                "read_random" => builtins::read_random(),
                "remainder" => builtins::remainder(data_stack),
                "store" => builtins::store(data_stack),
                "sub" => builtins::sub(data_stack),
                "submit_frame" => builtins::submit_frame(),
                "take" => builtins::take(data_stack),
                "write_tile" => builtins::write_tile(data_stack),
                _ => return Err(EvaluatorEffectKind::UnknownBuiltin { name }),
            };

            // This is a bit weird. An error is an effect, and effects can be
            // returned as a `Result::Ok` by the builtins. But error by itself
            // can also be returned as a `Result::Err`.
            //
            // This enables builtins to to stack operations using `?`
            // internally, without requiring effects to always be returned as
            // errors, which they aren't per se.
            //
            // Anyway, here we deal with this situation by unifying both
            // variants.
            let effect = match result {
                Ok(effect) => effect,
                Err(err) => Some(BuiltinEffect::Error(err)),
            };

            if let Some(effect) = effect {
                return Err(EvaluatorEffectKind::Builtin(effect));
            }
        }
        Instruction::CallFunction { name } => {
            let function = code.functions.get(&name).cloned().unwrap();
            call_stack.push(function)?;
        }
        Instruction::Push { value } => data_stack.push(value),
        Instruction::ReturnIfNonZero => {
            let value = data_stack.pop()?;
            if value != Value(0) {
                call_stack.pop();
            }
        }
        Instruction::ReturnIfZero => {
            let value = data_stack.pop()?;
            if value == Value(0) {
                call_stack.pop();
            }
        }
    }

    Ok(None)
}

enum CallStackUpdate {}
