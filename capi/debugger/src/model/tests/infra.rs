use capi_compiler::{
    code::{FragmentLocation, NamedFunctions},
    Compiler,
};
use capi_game_engine::{
    command::Command, game_engine::GameEngine, host::GameEngineHost,
    memory::Memory,
};
use capi_protocol::updates::Updates;

use crate::model::{
    ActiveFunctions, ActiveFunctionsEntry, DebugBranch, DebugFragment,
    DebugFragmentKind, DebugFunction, PersistentState, TransientState,
    UserAction,
};

pub fn debugger() -> TestDebugger {
    TestDebugger::default()
}

#[derive(Debug, Default)]
pub struct TestDebugger {
    current_time: f64,
    queued_commands: Vec<Command>,
    memory: Memory,
    updates: Updates,
    game_engine: Option<GameEngine>,
    persistent: PersistentState,
    transient: Option<TransientState>,
}

impl TestDebugger {
    pub fn provide_source_code(&mut self, source: &str) -> &mut Self {
        let mut compiler = Compiler::default();
        let output = compiler.compile::<GameEngineHost>(source);

        let command = self.persistent.on_new_code(output);
        self.queued_commands.push(command);

        self.update_transient_state();

        self
    }

    pub fn run_program(&mut self) -> &mut Self {
        self.game_engine = Some(GameEngine::new());

        self.process_commands();
        self.process_updates();
        self.update_transient_state();

        self
    }

    pub fn on_user_action(
        &mut self,
        action: UserAction,
    ) -> anyhow::Result<&mut Self> {
        let transient = self.transient_state();
        let commands = self.persistent.on_user_action(action, &transient)?;

        for command in commands {
            self.queued_commands.push(command);
        }

        self.process_commands();
        self.process_updates();
        self.update_transient_state();

        Ok(self)
    }

    fn process_commands(&mut self) {
        if let Some(game_engine) = &mut self.game_engine {
            for command in self.queued_commands.drain(..) {
                game_engine.on_command(command);
            }

            let mut pixels = [];
            game_engine.run_until_end_of_frame(self.current_time, &mut pixels);
            self.current_time += 1.;
        }
    }

    fn process_updates(&mut self) {
        if let Some(game_engine) = &self.game_engine {
            self.updates
                .queue_updates(&game_engine.runtime, &self.memory);
            for update in self.updates.take_queued_updates() {
                self.persistent.on_update_from_host(update);
            }
        }
    }

    fn update_transient_state(&mut self) {
        self.transient = Some(self.persistent.generate_transient_state());
    }

    pub fn expect_code(&self) -> &NamedFunctions {
        &self.persistent.code.inner.as_ref().unwrap().named_functions
    }

    pub fn expect_fragment(
        &mut self,
        location: &FragmentLocation,
    ) -> DebugFragment {
        let Some(fragment) = self
            .transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .into_iter()
            .find_map(|function| {
                function.branches.iter().find_map(|branch| {
                    branch.body.iter().find_map(|fragment| {
                        if &fragment.data.location == location {
                            Some(fragment.clone())
                        } else {
                            None
                        }
                    })
                })
            })
        else {
            panic!("Expected to find fragment with ID `{location:?}`");
        };

        fragment
    }

    pub fn transient_state(&mut self) -> TransientState {
        self.transient
            .get_or_insert_with(|| self.persistent.generate_transient_state())
            .clone()
    }
}

pub trait ActiveFunctionsExt {
    fn expect_entries(&self) -> Vec<ActiveFunctionsEntry>;
    fn names(&self) -> Vec<String>;
}

impl ActiveFunctionsExt for ActiveFunctions {
    fn expect_entries(&self) -> Vec<ActiveFunctionsEntry> {
        self.entries().unwrap().inner.to_vec()
    }

    fn names(&self) -> Vec<String> {
        self.expect_entries()
            .expect_functions()
            .into_iter()
            .filter_map(|function| function.name)
            .collect()
    }
}

pub trait ActiveFunctionsEntriesExt {
    fn expect_functions(&self) -> Vec<DebugFunction>;
}

impl ActiveFunctionsEntriesExt for Vec<ActiveFunctionsEntry> {
    fn expect_functions(&self) -> Vec<DebugFunction> {
        self.iter()
            .map(|entry| match entry {
                ActiveFunctionsEntry::Function(function) => function.clone(),
                ActiveFunctionsEntry::Gap => {
                    panic!(
                        "Expected function, encountered gap. Entries:\n\
                        {self:#?}"
                    );
                }
            })
            .collect()
    }
}

pub trait FunctionsExt {
    fn expect_leaf(self, name: &str) -> DebugFunction;
    fn with_name(self, name: &str) -> DebugFunction;
}

impl FunctionsExt for Vec<DebugFunction> {
    fn expect_leaf(mut self, name: &str) -> DebugFunction {
        let f = self.remove(0);
        assert_eq!(f.name.as_deref(), Some(name));
        f
    }

    fn with_name(self, name: &str) -> DebugFunction {
        self.into_iter()
            .find(|function| function.name.as_deref() == Some(name))
            .unwrap()
    }
}

pub trait DebugFunctionExt {
    fn active_fragment(self) -> DebugFragment;
    fn only_branch(self) -> DebugBranch;
}

impl DebugFunctionExt for DebugFunction {
    fn active_fragment(self) -> DebugFragment {
        self.branches
            .into_iter()
            .find_map(|branch| {
                branch
                    .body
                    .into_iter()
                    .find(|fragment| fragment.data.state.is_active())
            })
            .expect("Expected to find an active fragment")
    }

    fn only_branch(mut self) -> DebugBranch {
        let branch = self.branches.remove(0);

        assert!(
            self.branches.is_empty(),
            "Expected one branch, but there are multiple."
        );

        branch
    }
}

pub trait DebugBranchExt {
    #[allow(unused)] // currently unused, but might come in handy later
    fn fragment(&self, i: usize) -> DebugFragment;
}

impl DebugBranchExt for DebugBranch {
    fn fragment(&self, i: usize) -> DebugFragment {
        let Some(fragment) = self.body.get(i) else {
            panic!("{i}-th fragment in `{:?}` not available", self.body);
        };

        fragment.clone()
    }
}

pub trait DebugFragmentExt {
    fn expect_call_to_function(self, called_fn: &str) -> Self;
    fn expect_call_to_host_function(self, called_host_fn: &str) -> Self;
    fn expect_call_to_intrinsic(self, called_intrinsic: &str) -> Self;
    fn expect_function(self) -> DebugFunction;
}

impl DebugFragmentExt for DebugFragment {
    fn expect_call_to_function(self, called_fn: &str) -> Self {
        let DebugFragmentKind::CallToFunction { name } = &self.kind else {
            panic!("Expected call to function.");
        };
        assert_eq!(called_fn, name);

        self
    }

    fn expect_call_to_host_function(self, called_host_fn: &str) -> Self {
        let DebugFragmentKind::CallToHostFunction { name } = &self.kind else {
            panic!("Expected call to function.");
        };
        assert_eq!(called_host_fn, name);

        self
    }

    fn expect_call_to_intrinsic(self, called_intrinsic: &str) -> Self {
        let DebugFragmentKind::CallToIntrinsic { name } = &self.kind else {
            panic!("Expected call to function.");
        };
        assert_eq!(called_intrinsic, name);

        self
    }

    fn expect_function(self) -> DebugFunction {
        let DebugFragmentKind::Function { function } = self.kind else {
            panic!("Expected function");
        };

        function
    }
}
