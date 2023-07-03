pub mod intrinsics;
pub mod std;
pub mod tests;

use super::Functions;

pub fn define_code() -> anyhow::Result<(Functions, Functions)> {
    let mut functions = Functions::new();
    intrinsics::define(&mut functions);
    std::define(&mut functions)?;

    let tests = tests::define(&mut functions)?;

    Ok((functions, tests))
}
