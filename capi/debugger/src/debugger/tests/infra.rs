use capi_compiler::{
    compile,
    fragments::{self, Payload},
    intrinsics::Intrinsic,
};
use capi_game_engine::{host::GameEngineHost, memory::Memory};
use capi_process::{Instructions, Process, Value};
use capi_protocol::updates::{Code, Updates};

use crate::debugger::{
    active_functions::ActiveFunctionsEntry, ActiveFunctions, Branch,
    DebugFragment, DebugFragmentKind, DebugFunction, Debugger, OtherExpression,
    RemoteProcess,
};

pub fn init() -> TestInfra {
    TestInfra::default()
}

#[derive(Default)]
pub struct TestInfra {
    remote_process: RemoteProcess,
    instructions: Option<Instructions>,
}

impl TestInfra {
    pub fn provide_source_code(&mut self, source: &str) -> &mut Self {
        let (fragments, instructions, source_map) =
            compile::<GameEngineHost>(source);

        self.remote_process.on_code_update(Code {
            fragments: fragments.clone(),
            instructions: instructions.clone(),
            source_map,
        });

        self.instructions = Some(instructions);

        self
    }

    pub fn run_process(&mut self) -> &mut Self {
        let instructions = self.instructions.as_ref().expect(
            "Must provide source code via `TestSetup::source_code` before \
            running process.",
        );

        let mut process = Process::default();
        process.reset([0, 0].map(Value::from));
        while process.can_step() {
            process.step(instructions);
        }

        let memory = Memory::default();
        let mut updates = Updates::default();

        updates.queue_updates(&process, &memory);
        for update in updates.take_queued_updates() {
            self.remote_process.on_runtime_update(update);
        }

        self
    }

    pub fn to_debugger(&self) -> Debugger {
        self.remote_process.to_debugger()
    }
}

pub trait ActiveFunctionsExt {
    fn expect_entries(&self) -> Vec<ActiveFunctionsEntry>;
    fn names(&self) -> Vec<String>;
}

impl ActiveFunctionsExt for ActiveFunctions {
    fn expect_entries(&self) -> Vec<ActiveFunctionsEntry> {
        let ActiveFunctions::Entries { entries } = self else {
            panic!("Expected active functions to be displayed");
        };

        entries.clone()
    }

    fn names(&self) -> Vec<String> {
        self.expect_entries()
            .functions()
            .into_iter()
            .filter_map(|function| function.name)
            .collect()
    }
}

pub trait ActiveFunctionsEntriesExt {
    fn functions(&self) -> Vec<DebugFunction>;
}

impl ActiveFunctionsEntriesExt for Vec<ActiveFunctionsEntry> {
    fn functions(&self) -> Vec<DebugFunction> {
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
                branch.body.into_iter().find(|fragment| fragment.is_active)
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

pub trait DebugFragmentExt {
    fn expect_call_to(self, name: &str);
    fn expect_function(self) -> DebugFunction;
    fn expect_other(self) -> OtherExpression;
}

impl DebugFragmentExt for DebugFragment {
    fn expect_call_to(self, called_fn: &str) {
        let DebugFragmentKind::Other(OtherExpression { payload, .. }) =
            self.kind
        else {
            panic!()
        };

        let Payload::CallToFunction { name, .. } = payload else {
            panic!()
        };

        assert_eq!(called_fn, name);
    }

    fn expect_function(self) -> DebugFunction {
        let DebugFragmentKind::Function { function } = self.kind else {
            panic!("Expected function");
        };

        function
    }

    fn expect_other(self) -> OtherExpression {
        let DebugFragmentKind::Other(other) = self.kind else {
            panic!("Expected other expression");
        };

        other
    }
}

pub trait FragmentExpressionExt {
    fn expect_intrinsic(self) -> Intrinsic;

    #[allow(unused)] // currently not in use, but likely to be useful soon
    fn expect_user_function(self) -> String;
}

impl FragmentExpressionExt for fragments::Payload {
    fn expect_intrinsic(self) -> Intrinsic {
        let fragments::Payload::CallToIntrinsic { intrinsic, .. } = self else {
            panic!("Expected call to intrinsic function.");
        };

        intrinsic
    }

    fn expect_user_function(self) -> String {
        let fragments::Payload::CallToFunction { name, .. } = self else {
            panic!("Expected user function");
        };

        name
    }
}
