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
    /// # Add two signed 8-bit integers
    "add_s8", AddS8;

    /// # Add two signed 32-bit integers
    "add_s32", AddS32;

    /// # Add two unsigned 8-bit integers
    "add_u8", AddU8;

    /// # Trigger a breakpoint
    "brk", Brk;

    /// # Evaluate an anonymous function
    "eval", Eval;
}
