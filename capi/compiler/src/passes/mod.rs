mod generate_fragments;
mod generate_instructions;
mod resolve_identifiers;
mod tail_position;

pub use {
    generate_fragments::generate_fragments,
    generate_instructions::generate_instructions,
    resolve_identifiers::resolve_identifiers,
    tail_position::determine_tail_positions,
};
