use super::{intrinsics, runtime::functions::Intrinsic};

pub fn all() -> impl IntoIterator<Item = (&'static str, Intrinsic)> {
    [
        ("+", intrinsics::add as Intrinsic),
        ("clone", intrinsics::clone),
        ("delay_ms", intrinsics::delay_ms),
        ("eval", intrinsics::eval),
        ("fn", intrinsics::fn_),
        ("nop", intrinsics::nop),
        ("over", intrinsics::over),
        ("ping", intrinsics::ping),
        ("print", intrinsics::print),
        ("swap", intrinsics::swap),
    ]
}
