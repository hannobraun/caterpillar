use crate::cp;

pub fn define(
    functions: &mut cp::Functions,
    tests: &mut cp::Functions,
) -> anyhow::Result<()> {
    let code = r#"
        mod bool {
            test "true" { true }
            test "false not" { false not }
            test "and - true true" { true true and }
            test "and - true false" { true false and not }
            test "and - false true" { false true and not }
            test "and - false false" { false false and not }
        }

        mod binding {
            test "binding" { true false => true_ false_ . true_ }
            test "tokenization" { true=>true_.true_ }
        }

        mod basics {
            test "drop" { true false drop }
            test "clone" { true clone drop }
        }

        mod block {
            test "eval" { { true } eval }
            test "lazy evaluation" { true { drop } drop }
            test "tokenization" { {true}eval{true}eval and }
        }

        mod array {
            test "unwrap" { [ true ] unwrap }
            test "eager evaluation" { true false [ drop ] drop }
            test "tokenization" { [true]unwrap[true]unwrap and }
        }

        mod fn_ {
            fn f { true }
            test "fn" { f }
        }

        mod if_ {
            test "then" { true { true } { false } if }
            test "else" { false { false } { true } if }
        }

        mod string {
            test "=" { "a" "a" = }
            test "= not" { "a" "b" = not }
            test "tokenization" { "a""a"="b""b"= and }
        }
    "#;

    let mut data_stack = cp::DataStack::new();
    let mut bindings = cp::Bindings::new();

    cp::execute(code, &mut data_stack, &mut bindings, functions, tests)?;

    if !data_stack.is_empty() {
        anyhow::bail!("Importing tests left values on stack: {data_stack:?}")
    }

    Ok(())
}
