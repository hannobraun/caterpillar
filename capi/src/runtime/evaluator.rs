use super::{
    builtins, stack, BuiltinEffect, Code, Instruction, Stack, StackUnderflow,
    Value,
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
        let Some(instruction) = stack.consume_next_instruction() else {
            return Ok(EvaluatorState::Finished);
        };

        evaluate_instruction(instruction, &self.code, stack)?;

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
    stack: &mut Stack,
) -> Result<(), EvaluatorEffect> {
    match instruction {
        Instruction::BindingEvaluate { name } => {
            let Some(value) = stack.bindings().get(&name).copied() else {
                panic!(
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
            for name in names.into_iter().rev() {
                let value = stack.pop_operand()?;
                stack.define_binding(name, value);
            }

            if !stack.operands().is_empty() {
                return Err(EvaluatorEffect::BindingLeftValuesOnStack);
            }
        }
        Instruction::CallBuiltin { name } => {
            let result = match name.as_str() {
                "add" => builtins::add(stack),
                "add_wrap_unsigned" => builtins::add_wrap_unsigned(stack),
                "brk" => builtins::brk(),
                "copy" => builtins::copy(stack),
                "div" => builtins::div(stack),
                "drop" => builtins::drop(stack),
                "eq" => builtins::eq(stack),
                "greater" => builtins::greater(stack),
                "load" => builtins::load(stack),
                "mul" => builtins::mul(stack),
                "neg" => builtins::neg(stack),
                "read_input" => builtins::read_input(),
                "read_random" => builtins::read_random(),
                "remainder" => builtins::remainder(stack),
                "store" => builtins::store(stack),
                "sub" => builtins::sub(stack),
                "submit_frame" => builtins::submit_frame(),
                "write_tile" => builtins::write_tile(stack),
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

            // If the current function is finished, pop its stack frame before
            // pushing the next one. This is tail call optimization.
            if stack.next_instruction_in_current_function().is_none() {
                stack
                    .pop_frame()
                    .expect("Currently executing; stack can't be empty");
            }

            stack.push_frame(function)?;
        }
        Instruction::Push { value } => stack.push_operand(value),
        Instruction::ReturnIfNonZero => {
            let value = stack.pop_operand()?;
            if value != Value(0) {
                stack
                    .pop_frame()
                    .expect("Currently executing; stack can't be empty");
            }
        }
        Instruction::ReturnIfZero => {
            let value = stack.pop_operand()?;
            if value == Value(0) {
                stack
                    .pop_frame()
                    .expect("Currently executing; stack can't be empty");
            }
        }
    }

    Ok(())
}
