use std::collections::BTreeMap;

use capi_compiler::host::{Host, HostFunction};
use num_enum::TryFromPrimitive;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GameEngineHost {
    functions_by_name: BTreeMap<String, GameEngineFunction>,
}

impl Default for GameEngineHost {
    fn default() -> Self {
        let mut functions_by_name = BTreeMap::new();

        for function in enum_iterator::all::<GameEngineFunction>() {
            functions_by_name.insert(function.name().to_owned(), function);
        }

        Self { functions_by_name }
    }
}

impl Host for GameEngineHost {
    fn effect_number_to_function_name(
        &self,
        effect: u8,
    ) -> Option<&'static str> {
        let function = GameEngineFunction::try_from_primitive(effect).ok()?;
        Some(function.name())
    }

    fn function_name_to_effect_number(
        &self,
        name: &str,
    ) -> Option<&dyn HostFunction> {
        let effect = self.functions_by_name.get(name)?;
        Some(effect)
    }
}

/// # An effect handled by the game engine host
///
/// ## Implementation Note
///
/// The host functions that are backed by these effects are not purely
/// functional. Long-term, they should be, but for now there's not much point to
/// it.
///
/// Without a type system, piping any values that represent I/O resources
/// through host functions is only complexity for no gain. And without a
/// _linear_ type system, there's no way to guarantee any sane semantics around
/// such functions anyway.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    enum_iterator::Sequence,
    num_enum::IntoPrimitive,
    num_enum::TryFromPrimitive,
    serde::Deserialize,
    serde::Serialize,
)]
#[repr(u8)]
pub enum GameEngineFunction {
    /// # Halt the game
    ///
    /// This essentially works like the `brk` intrinsic. It was added
    /// specifically to have a breakpoint-like effect in the game engine, for
    /// use in the debugger's test suite. Maybe it will find other uses later.
    ///
    /// ## Input
    ///
    /// none
    ///
    /// ## Output
    ///
    /// none
    Halt,

    /// # Load a value from a given memory address
    ///
    /// ## Input
    ///
    /// - `u8`: The address of the value to read.
    ///
    /// ## Output
    ///
    /// - `u8`: The value at the provided address address.
    Load,

    /// # Store a value at the given memory address
    ///
    /// ## Input
    ///
    /// - `u8`: The value to store.
    /// - `u8`: The address to store the value at.
    ///
    /// ## Output
    ///
    /// none
    Store,

    /// # Read the next input event from the buffer
    ///
    /// ## Input
    ///
    /// none
    ///
    /// ## Output
    ///
    /// - `u8`: A value representing the type of input event.
    ReadInput,

    /// # Read a random value from the buffer
    ///
    /// ## Input
    ///
    /// none
    ///
    /// ## Output
    ///
    /// - `s32`: The random value.
    ReadRandom,

    /// # Set a pixel in the frame buffer
    ///
    /// ## Input
    ///
    /// - `u8`: The x-coordinate of the pixel.
    /// - `u8`: The y-coordinate of the pixel.
    /// - `u8`: The red channel value of the pixel.
    /// - `u8`: The green channel value of the pixel.
    /// - `u8`: The blue channel value of the pixel.
    /// - `u8`: The alpha channel value of the pixel.
    ///
    /// ## Output
    ///
    /// none
    SetPixel,

    /// # Submit the current frame, causing the game engine to display it
    ///
    /// This must be called regularly, or the game engine will freeze.
    ///
    /// ## Input
    ///
    /// none
    ///
    /// ## Output
    ///
    /// none
    ///
    /// ## Implementation Note
    ///
    /// The possibility of the game engine freezing is undesirable. At some
    /// future point, there will likely be an enforced timeout. This is tracked
    /// in the following issue:
    /// <https://github.com/hannobraun/caterpillar/issues/42>
    SubmitFrame,
}

impl HostFunction for GameEngineFunction {
    fn number(&self) -> u8 {
        (*self).into()
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Halt => "halt",
            Self::Load => "load",
            Self::Store => "store",
            Self::ReadInput => "read_input",
            Self::ReadRandom => "read_random",
            Self::SetPixel => "set_pixel",
            Self::SubmitFrame => "submit_frame",
        }
    }
}
