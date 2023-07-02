use crossterm::style::Stylize;

use crate::cp;

pub fn print(test_reports: &[cp::TestReport]) {
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
