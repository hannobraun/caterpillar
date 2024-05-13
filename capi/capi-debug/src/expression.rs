use capi_runtime::{
    Expression, ExpressionKind, Program, ProgramEffect, ProgramState,
};
use leptos::{component, view, IntoView, ReadSignal, SignalGet};

#[component]
pub fn Expression(
    program: ReadSignal<Option<Program>>,
    expression: Expression,
) -> impl IntoView {
    let is_comment = matches!(expression.kind, ExpressionKind::Comment { .. });
    let class = move || {
        let program = program.get()?;
        let state = program.state;

        let text_classes = if is_comment {
            "italic text-gray-500"
        } else {
            ""
        };

        let bg_class = match state {
            ProgramState::Effect { effect, address }
                if program
                    .source_map
                    .address_to_location(&address)
                    .as_ref()
                    == Some(&expression.location) =>
            {
                match effect {
                    ProgramEffect::Paused => "bg-green-300",
                    _ => "bg-red-300",
                }
            }
            _ => "",
        };

        Some(format!("px-0.5 {text_classes} {bg_class}"))
    };
    let line = format!("{}", expression.kind);

    view! {
        <span class=class>{line}</span>
    }
}
