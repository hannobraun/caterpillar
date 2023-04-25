use crossterm::style::Stylize;

use crate::cp;

pub struct TestReport {
    pub module: String,
    pub name: String,
    pub result: Result<(), Error>,
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
