use capi_compiler::{compile, fragments};
use capi_game_engine::{host::GameEngineHost, memory::Memory};
use capi_process::{Instructions, Process, Value};
use capi_protocol::updates::{Code, Updates};

use crate::debugger::{
    ActiveFunctions, Debugger, Expression, Function, OtherExpression,
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
    fn expect_functions(self) -> Vec<Function>;
}

impl ActiveFunctionsExt for ActiveFunctions {
    fn expect_functions(self) -> Vec<Function> {
        let ActiveFunctions::Functions { functions } = self else {
            panic!("Expected active functions to be displayed");
        };

        functions
    }
}

pub trait ExpressionExt {
    fn expect_block(self) -> Vec<Expression>;
    fn expect_other(self) -> OtherExpression;
}

impl ExpressionExt for Expression {
    fn expect_block(self) -> Vec<Expression> {
        let Expression::Function { mut function } = self else {
            panic!("Expected block");
        };

        function.branches.remove(0).body
    }

    fn expect_other(self) -> OtherExpression {
        let Expression::Other(other) = self else {
            panic!("Expected other expression");
        };

        other
    }
}

pub trait FragmentExpressionExt {
    fn expect_builtin_function(self) -> String;
    #[allow(unused)] // currently not in use, but likely to be useful soon
    fn expect_user_function(self) -> String;
}

impl FragmentExpressionExt for fragments::Payload {
    fn expect_builtin_function(self) -> String {
        let fragments::Payload::ResolvedBuiltinFunction { name } = self else {
            panic!("Expected builtin function");
        };

        name
    }

    fn expect_user_function(self) -> String {
        let fragments::Payload::CallToFunction { name, .. } = self else {
            panic!("Expected user function");
        };

        name
    }
}
