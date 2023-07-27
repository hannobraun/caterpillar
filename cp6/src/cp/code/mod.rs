pub mod intrinsics;
pub mod std;

use super::Functions;

pub fn define_code() -> anyhow::Result<(Functions, Functions)> {
    let mut functions = Functions::new();
    let mut tests = Functions::new();

    intrinsics::code::define(&mut functions)?;
    intrinsics::tests::define(&mut functions, &mut tests)?;
    std::define(&mut functions, &mut tests)?;

    Ok((functions, tests))
}
