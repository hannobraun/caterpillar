mod build_call_graph;
mod detect_changes;
mod generate_instructions;

pub use {
    build_call_graph::resolve_dependencies, detect_changes::detect_changes,
    generate_instructions::generate_instructions,
};
