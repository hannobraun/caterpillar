use std::collections::BTreeMap;

use crate::runtime::{
    Function, Instruction, Location, MissingOperand, Operands, Value,
};

use super::event::Event;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Stack {
    state: State,
}

impl Stack {
    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn push_frame(
        &mut self,
        function: Function,
    ) -> Result<(), PushFrameError> {
        const RECURSION_LIMIT: usize = 8;
        if self.state.num_frames() >= RECURSION_LIMIT {
            return Err(PushFrameError::Overflow);
        }

        if self.state.num_frames() == 0 {
            // If there's no calling frame, then there's no place to take
            // arguments from. Make sure that the function doesn't expect any.
            assert_eq!(function.arguments.len(), 0);
        }

        let mut arguments = Bindings::new();

        for argument in function.arguments.iter().rev() {
            let value = self.pop_operand()?;
            arguments.insert(argument.clone(), value);
        }

        self.state.frames.push(StackFrame {
            function,
            bindings: Bindings::default(),
            operands: Operands::default(),
        });

        for (name, value) in arguments {
            self.define_binding(name, value);
        }

        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<(), StackIsEmpty> {
        if self.state.num_frames() == 0 {
            return Err(StackIsEmpty);
        }

        let return_values = self
            .state()
            .operands()
            .expect("Just confirmed that stack is not empty")
            .values()
            .collect::<Vec<_>>();

        self.state.frames.pop();

        if self.state.num_frames() == 0 {
            // We just popped the last frame. The return values have nowhere to
            // go.
            return Ok(());
        }

        for value in return_values {
            self.push_operand(value);
        }

        Ok(())
    }

    pub fn define_binding(&mut self, name: String, value: impl Into<Value>) {
        let value = value.into();
        let frame = self.state.frames.last_mut().unwrap();
        frame.bindings.insert(name, value);
    }

    pub fn push_operand(&mut self, operand: impl Into<Value>) {
        let operand = operand.into();
        let frame = self.state.frames.last_mut().unwrap();
        frame.operands.push(operand);
    }

    pub fn pop_operand(&mut self) -> Result<Value, MissingOperand> {
        // This is a big hack, first copying the current top operand, then
        // telling `State` to pop it and throw away the result.
        //
        // Unfortunately, I can't come up with a design that meets the following
        // requirements:
        //
        // - Events can have return values.
        // - There's no duplication between the primary and the "event replay"
        //   use cases.
        // - There's no lifetime in `Event` that would prevent it from being
        //   stored.
        //
        // I'll keep thinking. For now this should do, even though I don't like
        // it.

        let operand = self.state.operands().unwrap().values().last();
        self.emit_event(Event::PopOperand);
        operand.ok_or(MissingOperand)
    }

    pub fn consume_next_instruction(&mut self) -> Option<Instruction> {
        loop {
            let frame = self.state.frames.last_mut()?;

            let Some(instruction) = frame.function.consume_next_instruction()
            else {
                self.pop_frame()
                    .expect("Just accessed frame; must be able to pop it");
                continue;
            };

            return Some(instruction);
        }
    }

    fn emit_event(&mut self, event: Event) {
        self.state.evolve(event);
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct State {
    pub frames: Vec<StackFrame>,
}

impl State {
    pub fn num_frames(&self) -> usize {
        self.frames.len()
    }

    pub fn bindings(&self) -> Option<&Bindings> {
        self.frames.last().map(|frame| &frame.bindings)
    }

    pub fn operands(&self) -> Option<&Operands> {
        self.frames.last().map(|frame| &frame.operands)
    }

    pub fn next_instruction_in_current_frame(&self) -> Option<Location> {
        self.frames
            .last()?
            .function
            .next_instruction()
            .map(|(location, _)| location)
    }

    pub fn next_instruction_overall(&self) -> Option<Location> {
        for frame in self.frames.iter().rev() {
            if let Some((location, _)) = frame.function.next_instruction() {
                return Some(location);
            }
        }

        None
    }

    pub fn is_next_instruction_in_any_frame(
        &self,
        location: &Location,
    ) -> bool {
        self.frames.iter().any(|frame| {
            frame
                .function
                .next_instruction()
                .map(|(location, _instruction)| location)
                == Some(location.clone().next())
        })
    }

    pub fn all_next_instructions_in_frames(
        &self,
    ) -> impl Iterator<Item = Location> + '_ {
        self.frames
            .iter()
            .filter_map(|frame| frame.function.next_instruction())
            .map(|(location, _instruction)| location)
    }

    pub fn evolve(&mut self, event: Event) {
        match event {
            Event::PopOperand => {
                let frame = self.frames.last_mut().expect(
                    "`Event::PopOperand` implies existence of stack frame",
                );
                frame.operands.pop().ok();
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StackFrame {
    pub function: Function,
    pub bindings: Bindings,
    pub operands: Operands,
}

pub type Bindings = BTreeMap<String, Value>;

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum PushFrameError {
    #[error(transparent)]
    MissingOperand(#[from] MissingOperand),

    #[error("Reached recursion limit")]
    Overflow,
}

#[derive(Debug)]
pub struct StackIsEmpty;
