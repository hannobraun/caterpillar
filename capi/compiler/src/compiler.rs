use capi_runtime::Instructions;

use crate::{
    fragments::Fragments,
    host::Host,
    passes::{
        detect_changes, determine_tail_positions, generate_fragments,
        generate_instructions, group_into_clusters, mark_recursive_calls,
        parse, resolve_identifiers, tokenize,
    },
    source_map::SourceMap,
};

/// # Entry point to the compiler API
#[derive(Default)]
pub struct Compiler {
    fragments: Option<Fragments>,
    instructions: Instructions,
    source_map: SourceMap,
}

impl Compiler {
    /// # Compile the provided source code
    pub fn compile<H: Host>(
        &mut self,
        source: &str,
    ) -> (Fragments, Instructions, SourceMap) {
        let tokens = tokenize(source);
        let mut functions = parse(tokens);
        determine_tail_positions(&mut functions);
        resolve_identifiers::<H>(&mut functions);
        let mut clusters = group_into_clusters(functions);
        mark_recursive_calls(&mut clusters);

        let fragments = generate_fragments(clusters);
        let changes = detect_changes(self.fragments.as_ref(), &fragments);

        self.fragments = Some(fragments.clone());

        generate_instructions(
            &fragments,
            &changes,
            &mut self.instructions,
            &mut self.source_map,
        );

        (
            fragments,
            self.instructions.clone(),
            self.source_map.clone(),
        )
    }
}
