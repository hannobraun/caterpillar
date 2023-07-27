pub mod intrinsics;
pub mod std;
pub mod tests;

use super::Functions;

pub fn define_code() -> anyhow::Result<(Functions, Functions)> {
    let mut functions = Functions::new();
    let mut tests = Functions::new();

    intrinsics::define(&mut functions);
    std::define(&mut functions, &mut tests)?;

    tests::define(&mut functions, &mut tests)?;

    Ok((functions, tests))
}
