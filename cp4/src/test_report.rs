use crossterm::style::Stylize;

use crate::cp;

pub struct TestReport {
    pub module: String,
    pub name: String,
    pub result: Result<(), Error>,
}

impl TestReport {
    pub fn new(
        module: String,
        name: String,
        result: Result<(), cp::EvaluatorError>,
        mut data_stack: cp::DataStack,
    ) -> Self {
        let result = result
            .map_err(Error::Evaluator)
            .and_then(|()| {
                if data_stack.pop_bool()? {
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

        TestReport {
            module,
            name,
            result,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Evaluator(cp::EvaluatorError),

    #[error(transparent)]
    ReturnValue(#[from] cp::DataStackError),

    #[error("Test did not return `true`")]
    TestFailed,

    #[error("Test returned too many values")]
    TestReturnedTooMuch,
}

pub fn print(test_reports: &[TestReport]) {
    for test_report in test_reports {
        match &test_report.result {
            Ok(()) => {
                print!("{}", "PASS".bold().green());
            }
            Err(_) => {
                print!("{}", "FAIL".bold().red());
            }
        }

        print!(" {} - {}", test_report.module, test_report.name);

        if let Err(err) = &test_report.result {
            print!("\n    {err}");
        }

        println!();
    }
}
