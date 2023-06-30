use std::collections::BTreeSet;

use crate::{
    cp::{self, AnalyzerEvent, FunctionBody},
    test_report::{Error, TestReport},
};

pub fn define(functions: &mut cp::Functions) -> anyhow::Result<cp::Functions> {
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
            test "fn" { fn f { true } f }
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

        mod std {
            test "times" { { true } 2 times drop }
        }
    "#;

    let mut data_stack = cp::DataStack::new();
    let mut bindings = cp::Bindings::new();
    let mut tests = cp::Functions::new();

    cp::execute(code, &mut data_stack, &mut bindings, functions, &mut tests)?;

    if !data_stack.is_empty() {
        anyhow::bail!("Importing tests left values on stack: {data_stack:?}")
    }

    Ok(tests)
}

pub fn run(
    functions: &mut cp::Functions,
    tests: &cp::Functions,
) -> anyhow::Result<Vec<TestReport>> {
    let mut updated = functions.clear_updated();
    let mut found_new_updated;

    loop {
        found_new_updated = false;

        for (name, function) in &*functions {
            if updated.contains(name) {
                continue;
            }

            if let FunctionBody::UserDefined(analyzer_output) = &function.body {
                for event in analyzer_output.all_events_recursive() {
                    if let AnalyzerEvent::EvalFunction { name: called } = event
                    {
                        if updated.contains(called) {
                            updated.insert(name.clone());
                            found_new_updated = true;
                        }
                    }
                }
            }
        }

        if !found_new_updated {
            break;
        }
    }

    let mut tests_to_run = BTreeSet::new();
    let mut found_new_tests_to_run;

    loop {
        found_new_tests_to_run = false;

        for (name, function) in tests {
            if tests_to_run.contains(name) {
                continue;
            }

            if let FunctionBody::UserDefined(analyzer_output) = &function.body {
                for event in analyzer_output.all_events_recursive() {
                    if let AnalyzerEvent::EvalFunction { name: called } = event
                    {
                        if updated.contains(called) {
                            tests_to_run.insert(name.clone());
                            found_new_tests_to_run = true;
                        }
                    }
                }
            }
        }

        if !found_new_tests_to_run {
            break;
        }
    }

    dbg!(tests_to_run);

    let mut results = Vec::new();

    for (name, function) in tests {
        let mut data_stack = cp::DataStack::new();
        let mut bindings = cp::Bindings::new();
        let tests = cp::Functions::new();

        let mut evaluator = cp::Evaluator {
            data_stack: &mut data_stack,
            bindings: &mut bindings,
            functions,
            tests: &tests,
        };

        let result = evaluator.evaluate_function(function);

        let result = result
            .map_err(Error::Evaluator)
            .and_then(|()| {
                let test_passed = data_stack.pop_bool()?;
                if test_passed {
                    Ok(())
                } else {
                    Err(Error::TestFailed)
                }
            })
            .and_then(|()| {
                if data_stack.is_empty() {
                    Ok(())
                } else {
                    Err(Error::TestReturnedTooMuch)
                }
            });

        results.push(TestReport {
            module: function.module.clone(),
            name: name.clone(),
            result,
        })
    }

    Ok(results)
}
