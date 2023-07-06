#[allow(unused)]
mod cp;

use sycamore::prelude::*;

fn main() -> anyhow::Result<()> {
    sycamore::render(|cx| {
        view! { cx,
            p { "Hello, world!" }
        }
    });

    Ok(())
}
