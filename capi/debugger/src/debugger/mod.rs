mod active_functions;
mod debugger;
mod expression;
mod function;
mod remote_process;

pub use self::{
    active_functions::ActiveFunctions,
    debugger::Debugger,
    expression::{Expression, OtherExpression},
    function::Function,
    remote_process::RemoteProcess,
};

#[cfg(test)]
mod tests {
    use capi_compiler::{
        compile,
        repr::{fragments::FragmentExpression, syntax::Script},
    };
    use capi_process::{CoreEffect, Effect, Process};
    use capi_protocol::{
        host::GameEngineHost,
        memory::Memory,
        updates::{SourceCode, Update, Updates},
    };

    use crate::debugger::{
        active_functions::ActiveFunctionsMessage, ActiveFunctions, Expression,
        RemoteProcess,
    };

    use super::{Debugger, Function, OtherExpression};

    #[test]
    fn source_code() {
        let mut remote_process = RemoteProcess::default();

        let debugger = remote_process.to_debugger();
        assert_eq!(
            debugger.active_functions,
            ActiveFunctions::Message {
                message: ActiveFunctionsMessage::NoServer
            }
        );
        assert!(debugger.operands.is_none());
        assert!(debugger.memory.is_none());

        remote_process.on_update(Update::SourceCode(SourceCode::default()));

        let debugger = remote_process.to_debugger();
        assert_eq!(
            debugger.active_functions,
            ActiveFunctions::Message {
                message: ActiveFunctionsMessage::NoProcess
            }
        );
        assert!(debugger.operands.is_none());
        assert!(debugger.memory.is_none());
    }

    #[test]
    fn uninitialized_process() {
        let mut remote_process = RemoteProcess::default();
        remote_process.on_update(Update::SourceCode(SourceCode::default()));

        let process = Process::default();
        let memory = Memory::default();
        let mut updates = Updates::default();

        updates.queue_updates(&process, &memory);
        for update in updates.take_queued_updates() {
            remote_process.on_update(update);
        }

        let debugger = remote_process.to_debugger();
        assert_eq!(
            debugger.active_functions,
            ActiveFunctions::Message {
                message: ActiveFunctionsMessage::ProcessRunning
            }
        );
        assert!(debugger.operands.is_none());
        assert_eq!(debugger.memory, Some(Memory::default()));
    }

    #[test]
    fn code_within_block() {
        // If execution is stopped within a block, the function that contains
        // that block should appear as an active function, and the current
        // instruction should be visible.

        let debugger = setup(|script| {
            script.function("main", [], |s| {
                s.block(|s| {
                    s.r("brk");
                })
                .r("eval");
            });
        });

        let other = debugger
            .active_functions
            .expect_functions()
            .remove(0)
            .body
            .remove(0)
            .expect_block()
            .remove(0)
            .expect_other();
        assert_eq!(other.effect, Some(Effect::Core(CoreEffect::Breakpoint)));

        let builtin = other.expression.expect_builtin();
        assert_eq!(builtin, "brk");
    }

    fn setup(f: impl FnOnce(&mut Script)) -> Debugger {
        let mut script = Script::default();
        f(&mut script);

        let (fragments, bytecode, source_map) =
            compile::<GameEngineHost>(script);

        let mut remote_process = RemoteProcess::default();
        remote_process.on_update(Update::SourceCode(SourceCode {
            fragments: fragments.clone(),
            source_map,
        }));

        let mut process = Process::default();
        process.reset(&bytecode, []);
        while process.state().can_step() {
            process.step(&bytecode);
        }

        let memory = Memory::default();
        let mut updates = Updates::default();

        updates.queue_updates(&process, &memory);
        for update in updates.take_queued_updates() {
            remote_process.on_update(update);
        }

        remote_process.to_debugger()
    }

    trait ActiveFunctionsExt {
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

    trait ExpressionExt {
        fn expect_block(self) -> Vec<Expression>;
        fn expect_other(self) -> OtherExpression;
    }

    impl ExpressionExt for Expression {
        fn expect_block(self) -> Vec<Expression> {
            let Expression::Block { expressions } = self else {
                panic!("Expected block");
            };

            expressions
        }

        fn expect_other(self) -> OtherExpression {
            let Expression::Other(other) = self else {
                panic!("Expected other expression");
            };

            other
        }
    }

    trait FragmentExpressionExt {
        fn expect_builtin(self) -> String;
    }

    impl FragmentExpressionExt for FragmentExpression {
        fn expect_builtin(self) -> String {
            let FragmentExpression::ResolvedBuiltinFunction { name: builtin } =
                self
            else {
                panic!("Expected builtin");
            };

            builtin
        }
    }
}
