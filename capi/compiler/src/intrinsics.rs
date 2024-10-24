use std::fmt;

macro_rules! intrinsics {
    (
        $(
            #[$attrs:meta]
            $name:expr,
            $variant:ident,
            $signature:expr;
        )*
    ) => {
        /// # Special functions that are known to the compiler
        ///
        /// When encountering a call to an intrinsic, the compiler will directly
        /// translate that into the appropriate instructions.
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            PartialEq,
            Ord,
            PartialOrd,
            serde::Deserialize,
            serde::Serialize,
            udigest::Digestable,
        )]
        pub enum IntrinsicFunction {
            $($variant,)*
        }

        impl IntrinsicFunction {
            pub fn from_name(name: &str) -> Option<IntrinsicFunction> {
                let intrinsic = match name {
                    $($name => Self::$variant,)*

                    _ => {
                        return None;
                    }
                };

                Some(intrinsic)
            }

            pub fn signature(&self) -> Option<[u32; 2]> {
                match self {
                    $(Self::$variant => $signature,)*
                }
            }
        }

        impl fmt::Display for IntrinsicFunction {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(Self::$variant => write!(f, $name),)*
                }
            }
        }
    };
}

intrinsics! {
    /// # Add two signed 8-bit integers, triggering an error on overflow
    "add_s8", AddS8, Some([2, 1]);

    /// # Add two signed 32-bit integers, triggering an error on overflow
    "add_s32", AddS32, Some([2, 1]);

    /// # Add two unsigned 8-bit integers, triggering an error on overflow
    "add_u8", AddU8, Some([2, 1]);

    /// # Add two unsigned 8-bit integers, wrapping on overflow
    "add_u8_wrap", AddU8Wrap, Some([2, 1]);

    /// # Logical and
    "and", And, Some([2, 1]);

    /// # Trigger a breakpoint
    "brk", Brk, Some([0, 0]);

    /// # Copy a value
    "copy", Copy, Some([1, 2]);

    /// # Divide two signed 32-bit integers
    "div_s32", DivS32, Some([2, 1]);

    /// # Divide two unsigned 8-bit integers
    "div_u8", DivU8, Some([2, 1]);

    /// # Drop a value
    "drop", Drop, Some([1, 0]);

    /// # Compare two values for equality
    "eq", Eq, Some([2, 1]);

    /// # Evaluate an anonymous function
    "eval", Eval, None;

    /// # Determine if the first of two signed 8-bit numbers is greater
    "greater_s8", GreaterS8, Some([2, 1]);

    /// # Determine if the first of two signed 32-bit numbers is greater
    "greater_s32", GreaterS32, Some([2, 1]);

    /// # Determine if the first of two unsigned 8-bit numbers is greater
    "greater_u8", GreaterU8, Some([2, 1]);

    /// # Multiply two signed 32-bit numbers, triggering an error on overflow
    "mul_s32", MulS32, Some([2, 1]);

    /// # Multiply two unsigned 8-bit numbers, wrapping on overflow
    "mul_u8_wrap", MulU8Wrap, Some([2, 1]);

    /// # Negate a signed 32-bit number
    "neg_s32", NegS32, Some([1, 1]);

    /// No operation
    "nop", Nop, Some([0, 0]);

    /// # Logical not
    "not", Not, Some([1, 1]);

    /// # Compute the remainder of the division of two signed 32-bit numbers
    "remainder_s32", RemainderS32, Some([2, 1]);

    /// # Convert a signed 32-bit number to a signed 8-bit number
    "s32_to_s8", S32ToS8, Some([1, 1]);

    /// # Subtract two signed 32-bit numbers, triggering an error on overflow
    "sub_s32", SubS32, Some([2, 1]);

    /// # Subtract two unsigned 8-bit numbers, triggering an error on overflow
    "sub_u8", SubU8, Some([2, 1]);

    /// # Subtract two unsigned 8-bit numbers, wrapping on overflow
    "sub_u8_wrap", SubU8Wrap, Some([2, 1]);
}
