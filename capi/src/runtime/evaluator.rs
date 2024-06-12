use crate::code::Code;

use super::{
    builtins, Bindings, BuiltinEffect, CallStackOverflow, DataStack, Function,
    Instruction, Location, Stack, StackFrame, StackUnderflow, Value,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Evaluator {
    code: Code,
    stack: Stack,
}

impl Evaluator {
    pub fn new(code: Code, entry: Function) -> Self {
        Self {
            code,
            stack: Stack::new(entry),
        }
    }

    pub fn next_instruction(&self) -> Location {
        self.stack.next().unwrap()
    }

    pub fn call_stack(&self) -> &Stack {
        &self.stack
    }

    pub fn data_stack(&self) -> &DataStack {
        &self.stack.top().unwrap().data_stack
    }

    pub fn reset(&mut self, entry: Function) {
        self.stack = Stack::new(entry);
    }

    pub fn push(&mut self, values: impl IntoIterator<Item = Value>) {
        for value in values {
            self.stack.top_mut().unwrap().data_stack.push(value);
        }
    }

    pub fn step(&mut self) -> Result<EvaluatorState, EvaluatorEffect> {
        let (mut frame, location, instruction) = loop {
            let Some(mut frame) = self.stack.pop() else {
                return Ok(EvaluatorState::Finished);
            };

            if let Some((location, instruction)) =
                frame.function.consume_next_instruction()
            {
                break (frame, location, instruction);
            }

            // If the function has no more instructions, we don't put it back,
            // meaning it returns.
        };

        let evaluate_result = evaluate_instruction(
            instruction,
            &self.code,
            &mut frame.data_stack,
            &mut frame.bindings,
        );

        // Don't put the stack frame back, if it is empty. This is tail call
        // optimization.
        //
        // This will lead to trouble, if the last instruction in the function
        // (the one we just executed) is an explicit return instruction. Those
        // pop *another* stack frame, which is one too many.
        //
        // I've decided not to address that, for the moment:
        //
        // 1. That is a weird pattern anyway, and doesn't really make sense to
        //    write.
        // 2. Explicit return instructions are a stopgap anyway, until we have
        //    more advanced control flow.
        if frame.function.next_instruction().is_some() {
            self.stack.push(frame).expect(
                "Just popped a stack frame; pushing one can't overflow",
            );
        } else {
            for value in frame.data_stack.values() {
                if let Some(stack_frame) = self.stack.top_mut() {
                    stack_frame.data_stack.push(value);
                } else {
                    // If we end up here, one of the following happened:
                    //
                    // 1. We've returned from the top-level function.
                    // 2. The top-level function has made a tail call.
                    //
                    // In case of 1, what we're doing here is irrelevant,
                    // because the program is over.
                    //
                    // In case of 2, we might be leaving useless stuff on the
                    // stack, I guess. Not critical right now, but longer-term,
                    // we can clean that up.
                }
            }
        }

        match evaluate_result {
            Ok(Some(call_stack_update)) => match call_stack_update {
                CallStackUpdate::Push(function) => {
                    let arguments = function.arguments.clone();
                    let mut stack_frame = StackFrame::new(function);

                    for argument in arguments.into_iter().rev() {
                        let value = self
                            .stack
                            .top_mut()
                            .unwrap()
                            .data_stack
                            .pop()
                            .map_err(|effect| EvaluatorEffect {
                                effect: effect.into(),
                                location: location.clone(),
                            })?;
                        stack_frame.bindings.insert(argument.clone(), value);
                    }

                    self.stack.push(stack_frame).map_err(|effect| {
                        EvaluatorEffect {
                            effect: effect.into(),
                            location: location.clone(),
                        }
                    })?;
                }
                CallStackUpdate::Pop => {
                    if let Some(stack_frame) = self.stack.pop() {
                        for value in stack_frame.data_stack.values() {
                            self.stack
                                .top_mut()
                                .unwrap()
                                .data_stack
                                .push(value);
                        }
                    }
                }
            },
            Ok(None) => {}
            Err(effect) => {
                return Err(EvaluatorEffect { effect, location });
            }
        }

        Ok(EvaluatorState::Running {
            just_executed: location,
        })
    }
}

#[derive(Debug)]
#[must_use]
pub enum EvaluatorState {
    Running { just_executed: Location },
    Finished,
}

#[derive(Debug)]
pub struct EvaluatorEffect {
    pub effect: EvaluatorEffectKind,
    pub location: Location,
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    thiserror::Error,
)]
pub enum EvaluatorEffectKind {
    #[error("Binding expression left values on stack")]
    BindingLeftValuesOnStack,

    #[error("Builtin effect: {self:?}")]
    Builtin(BuiltinEffect),

    #[error(transparent)]
    CallStack(#[from] CallStackOverflow),

    #[error(transparent)]
    StackError(#[from] StackUnderflow),

    #[error("Unknown builtin: {name}")]
    UnknownBuiltin { name: String },
}

fn evaluate_instruction(
    instruction: Instruction,
    code: &Code,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
) -> Result<Option<CallStackUpdate>, EvaluatorEffectKind> {
    match instruction {
        Instruction::BindingEvaluate { name } => {
            let value = bindings.get(&name).copied().expect(
                "Binding instruction only generated for existing bindings",
            );
            data_stack.push(value);
        }
        Instruction::BindingsDefine { names } => {
            for name in names.into_iter().rev() {
                let value = data_stack.pop()?;
                bindings.insert(name, value);
            }

            if !data_stack.is_empty() {
                return Err(EvaluatorEffectKind::BindingLeftValuesOnStack);
            }
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
                "read_input" => builtins::read_input(),
                "read_random" => builtins::read_random(),
                "remainder" => builtins::remainder(data_stack),
                "store" => builtins::store(data_stack),
                "sub" => builtins::sub(data_stack),
                "submit_frame" => builtins::submit_frame(),
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
            return Ok(Some(CallStackUpdate::Push(function)));
        }
        Instruction::Push { value } => data_stack.push(value),
        Instruction::ReturnIfNonZero => {
            let value = data_stack.pop()?;
            if value != Value(0) {
                return Ok(Some(CallStackUpdate::Pop));
            }
        }
        Instruction::ReturnIfZero => {
            let value = data_stack.pop()?;
            if value == Value(0) {
                return Ok(Some(CallStackUpdate::Pop));
            }
        }
    }

    Ok(None)
}

enum CallStackUpdate {
    Push(Function),
    Pop,
}
