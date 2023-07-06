mod cp;

use sycamore::prelude::*;

fn main() -> anyhow::Result<()> {
    let (mut functions, tests) = cp::define_code()?;
    let test_reports = cp::run_tests(&mut functions, &tests)?;

    sycamore::render(|cx| {
        let test_reports = create_signal(cx, test_reports);

        view! { cx,
            p { "Hello, world!" }
            ul {
                Indexed(
                    iterable=test_reports,
                    view=|cx, test_report| view! {cx,
                        li { (test_report.name) }
                    }
                )
            }
        }
    });

    Ok(())
}
