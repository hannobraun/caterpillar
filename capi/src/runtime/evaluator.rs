use super::{
    builtins, stack, Bindings, BuiltinEffect, Code, Function, Instruction,
    Operands, Stack, StackUnderflow, Value,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evaluator {
    code: Code,
}

impl Evaluator {
    pub fn new(code: Code) -> Self {
        Self { code }
    }

    pub fn step(
        &mut self,
        stack: &mut Stack,
    ) -> Result<EvaluatorState, EvaluatorEffect> {
        let mut location_tmp = None;
        let Some(evaluate_result) = stack.consume_next_instruction(
            |location, instruction, data, bindings| {
                location_tmp = Some(location);
                evaluate_instruction(instruction, &self.code, data, bindings)
            },
        ) else {
            return Ok(EvaluatorState::Finished);
        };

        match evaluate_result {
            Ok(Some(call_stack_update)) => match call_stack_update {
                CallStackUpdate::Push(function) => {
                    stack.push_frame(function)?;
                }
                CallStackUpdate::Pop => {
                    stack
                        .pop_frame()
                        .expect("Currently executing; stack can't be empty");
                }
            },
            Ok(None) => {}
            Err(effect) => {
                return Err(effect);
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

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum EvaluatorEffect {
    #[error("Binding expression left values on stack")]
    BindingLeftValuesOnStack,

    #[error("Builtin effect: {self:?}")]
    Builtin(BuiltinEffect),

    #[error(transparent)]
    CallStack(#[from] stack::PushError),

    #[error(transparent)]
    StackError(#[from] StackUnderflow),

    #[error("Unknown builtin: {name}")]
    UnknownBuiltin { name: String },
}

fn evaluate_instruction(
    instruction: Instruction,
    code: &Code,
    operands: &mut Operands,
    bindings: &mut Bindings,
) -> Result<Option<CallStackUpdate>, EvaluatorEffect> {
    match instruction {
        Instruction::BindingEvaluate { name } => {
            let value = bindings.get(&name).copied().expect(
                "Binding instruction only generated for existing bindings",
            );
            operands.push(value);
        }
        Instruction::BindingsDefine { names } => {
            for name in names.into_iter().rev() {
                let value = operands.pop()?;
                bindings.insert(name, value);
            }

            if !operands.is_empty() {
                return Err(EvaluatorEffect::BindingLeftValuesOnStack);
            }
        }
        Instruction::CallBuiltin { name } => {
            let result = match name.as_str() {
                "add" => builtins::add(operands),
                "add_wrap_unsigned" => builtins::add_wrap_unsigned(operands),
                "brk" => builtins::brk(),
                "copy" => builtins::copy(operands),
                "div" => builtins::div(operands),
                "drop" => builtins::drop(operands),
                "eq" => builtins::eq(operands),
                "greater" => builtins::greater(operands),
                "load" => builtins::load(operands),
                "mul" => builtins::mul(operands),
                "neg" => builtins::neg(operands),
                "read_input" => builtins::read_input(),
                "read_random" => builtins::read_random(),
                "remainder" => builtins::remainder(operands),
                "store" => builtins::store(operands),
                "sub" => builtins::sub(operands),
                "submit_frame" => builtins::submit_frame(),
                "write_tile" => builtins::write_tile(operands),
                _ => return Err(EvaluatorEffect::UnknownBuiltin { name }),
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
                return Err(EvaluatorEffect::Builtin(effect));
            }
        }
        Instruction::CallFunction { name } => {
            let function = code.functions.get(&name).cloned().unwrap();
            return Ok(Some(CallStackUpdate::Push(function)));
        }
        Instruction::Push { value } => operands.push(value),
        Instruction::ReturnIfNonZero => {
            let value = operands.pop()?;
            if value != Value(0) {
                return Ok(Some(CallStackUpdate::Pop));
            }
        }
        Instruction::ReturnIfZero => {
            let value = operands.pop()?;
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
