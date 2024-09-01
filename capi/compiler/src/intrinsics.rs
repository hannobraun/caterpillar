use std::fmt;

macro_rules! intrinsics {
    (
        $(
            #[$attrs:meta]
            $name:expr,
            $variant:ident;
        )*
    ) => {
        /// # Special functions that are known to the compiler
        ///
        /// When encountering a call to an intrinsic, the compiler will directly
        /// translate that into the appropriate instructions.
        #[derive(
            Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize,
        )]
        pub enum Intrinsic {
            $($variant,)*
        }

        impl Intrinsic {
            pub fn from_name(name: &str) -> Option<Intrinsic> {
                let intrinsic = match name {
                    $($name => Self::$variant,)*

                    _ => {
                        return None;
                    }
                };

                Some(intrinsic)
            }
        }

        impl fmt::Display for Intrinsic {
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
    "add_s8", AddS8;

    /// # Add two signed 32-bit integers, triggering an error on overflow
    "add_s32", AddS32;

    /// # Add two unsigned 8-bit integers, triggering an error on overflow
    "add_u8", AddU8;

    /// # Add two unsigned 8-bit integers, wrapping on overflow
    "add_u8_wrap", AddU8Wrap;

    /// # Logical and
    "and", And;

    /// # Trigger a breakpoint
    "brk", Brk;

    /// # Copy a value
    "copy", Copy;

    /// # Divide two signed 32-bit integers
    "div_s32", DivS32;

    /// # Divide two unsigned 8-bit integers
    "div_u8", DivU8;

    /// # Drop a value
    "drop", Drop;

    /// # Compare two values for equality
    "eq", Eq;

    /// # Evaluate an anonymous function
    "eval", Eval;

    /// # Determine if the first of two signed 8-bit numbers is greater
    "greater_s8", GreaterS8;

    /// # Determine if the first of two signed 32-bit numbers is greater
    "greater_s32", GreaterS32;

    /// # Determine if the first of two unsigned 8-bit numbers is greater
    "greater_u8", GreaterU8;

    /// # Multiply two signed 32-bit numbers, triggering an error on overflow
    "mul_s32", MulS32;

    /// # Multiply two unsigned 8-bit numbers, wrapping on overflow
    "mul_u8_wrap", MulU8Wrap;

    /// # Negate a signed 32-bit number
    "neg_s32", NegS32;

    /// # Logical not
    "not", Not;

    /// # Compute the remainder of the division of two signed 32-bit numbers
    "remainder_s32", RemainderS32;

    /// # Convert a signed 32-bit number to a signed 8-bit number
    "s32_to_s8", S32ToS8;

    /// # Subtract two signed 32-bit numbers, triggering an error on overflow
    "sub_s32", SubS32;
}
