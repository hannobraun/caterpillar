use capi_compiler::{
    compile,
    fragments::{FragmentId, Fragments},
};
use capi_game_engine::{
    command::Command, game_engine::GameEngine, host::GameEngineHost,
    memory::Memory,
};
use capi_protocol::updates::{Code, Updates};

use crate::model::{
    ActiveFunctions, ActiveFunctionsEntry, Branch, DebugFragment,
    DebugFragmentKind, DebugFunction, PersistentState, UserAction,
};

pub fn debugger() -> TestDebugger {
    TestDebugger::default()
}

#[derive(Default)]
pub struct TestDebugger {
    pub queued_commands: Vec<Command>,
    pub game_engine: Option<GameEngine>,
    pub state: PersistentState,
}

impl TestDebugger {
    pub fn provide_source_code(&mut self, source: &str) -> &mut Self {
        let (fragments, instructions, source_map) =
            compile::<GameEngineHost>(source);

        self.state.on_new_code(Code {
            fragments,
            instructions,
            source_map,
        });

        self
    }

    pub fn run_program(&mut self) -> &mut Self {
        let game_engine = GameEngine::new();

        self.game_engine = Some(game_engine);
        self.process_commands();

        let game_engine = self
            .game_engine
            .as_mut()
            .expect("Just set `self.game_engine` to `Some`");

        let instructions = &self
            .state
            .code
            .as_ref()
            .expect(
                "Must provide source code via `provide_source_code` before \
                calling `TestDebugger::run_program`.",
            )
            .instructions;

        while game_engine.process.state().is_running() {
            game_engine.process.evaluate_next_instruction(instructions);
        }

        let memory = Memory::default();
        let mut updates = Updates::default();

        updates.queue_updates(&game_engine.process, &memory);
        for update in updates.take_queued_updates() {
            self.state.on_update_from_runtime(update);
        }

        self
    }

    pub fn on_user_action(
        &mut self,
        action: UserAction,
    ) -> anyhow::Result<&mut Self> {
        let (command, _) = self.state.on_user_action(action)?;

        if let Some(command) = command {
            self.queued_commands.push(command);
        }

        self.process_commands();

        Ok(self)
    }

    fn process_commands(&mut self) {
        if let Some(game_engine) = &mut self.game_engine {
            for command in self.queued_commands.drain(..) {
                game_engine.on_command(command);
            }
        }
    }

    pub fn expect_code(&self) -> &Fragments {
        &self.state.code.as_ref().unwrap().fragments
    }

    pub fn expect_fragment(&self, id: &FragmentId) -> DebugFragment {
        let Some(fragment) = self
            .state
            .generate_transient_state()
            .active_functions
            .expect_entries()
            .expect_functions()
            .into_iter()
            .find_map(|function| {
                function.branches.iter().find_map(|branch| {
                    branch.body.iter().find_map(|fragment| {
                        if fragment.data.id == *id {
                            Some(fragment.clone())
                        } else {
                            None
                        }
                    })
                })
            })
        else {
            panic!("Expected to find fragment with ID `{id}`");
        };

        fragment
    }
}

pub trait ActiveFunctionsExt {
    fn expect_entries(&self) -> Vec<ActiveFunctionsEntry>;
    fn names(&self) -> Vec<String>;
}

impl ActiveFunctionsExt for ActiveFunctions {
    fn expect_entries(&self) -> Vec<ActiveFunctionsEntry> {
        let ActiveFunctions::Entries { entries } = self else {
            panic!(
                "Expected active functions to display entries. Actual state:\n\
                {self:#?}"
            );
        };

        entries.clone()
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
    fn with_name(self, name: &str) -> DebugFunction;
}

impl FunctionsExt for Vec<DebugFunction> {
    fn with_name(self, name: &str) -> DebugFunction {
        self.into_iter()
            .find(|function| function.name.as_deref() == Some(name))
            .unwrap()
    }
}

pub trait DebugFunctionExt {
    fn active_fragment(self) -> DebugFragment;
    fn only_branch(self) -> Branch;
}

impl DebugFunctionExt for DebugFunction {
    fn active_fragment(self) -> DebugFragment {
        self.branches
            .into_iter()
            .find_map(|branch| {
                branch
                    .body
                    .into_iter()
                    .find(|fragment| fragment.data.is_active)
            })
            .expect("Expected to find an active fragment")
    }

    fn only_branch(mut self) -> Branch {
        let branch = self.branches.remove(0);

        assert!(
            self.branches.is_empty(),
            "Expected one branch, but there are multiple."
        );

        branch
    }
}

pub trait BranchExt {
    #[allow(unused)] // currently unused, but might come in handy later
    fn fragment(&self, i: usize) -> DebugFragment;
}

impl BranchExt for Branch {
    fn fragment(&self, i: usize) -> DebugFragment {
        let Some(fragment) = self.body.get(i) else {
            panic!("{i}-th fragment in `{:?}` not available", self.body);
        };

        fragment.clone()
    }
}

pub trait DebugFragmentExt {
    fn expect_call_to_function(self, called_fn: &str);
    fn expect_call_to_host_function(self, called_host_fn: &str);
    fn expect_call_to_intrinsic(self, called_intrinsic: &str);
    fn expect_function(self) -> DebugFunction;
}

impl DebugFragmentExt for DebugFragment {
    fn expect_call_to_function(self, called_fn: &str) {
        let DebugFragmentKind::CallToFunction { name } = self.kind else {
            panic!("Expected call to function.");
        };

        assert_eq!(called_fn, name);
    }

    fn expect_call_to_host_function(self, called_host_fn: &str) {
        let DebugFragmentKind::CallToHostFunction { name } = self.kind else {
            panic!("Expected call to function.");
        };

        assert_eq!(called_host_fn, name);
    }

    fn expect_call_to_intrinsic(self, called_intrinsic: &str) {
        let DebugFragmentKind::CallToIntrinsic { name } = self.kind else {
            panic!("Expected call to function.");
        };

        assert_eq!(called_intrinsic, name);
    }

    fn expect_function(self) -> DebugFunction {
        let DebugFragmentKind::Function { function } = self.kind else {
            panic!("Expected function");
        };

        function
    }
}
