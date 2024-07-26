mod active_functions;
mod debugger;
mod expression;
mod function;
mod remote_process;

pub use self::{
    active_functions::ActiveFunctions, debugger::Debugger,
    expression::Expression, function::Function, remote_process::RemoteProcess,
};

#[cfg(test)]
mod tests {
    use capi_compiler::{
        compile,
        repr::{fragments::FragmentExpression, syntax::Script},
    };
    use capi_process::Process;
    use capi_protocol::{
        host::GameEngineHost,
        memory::Memory,
        updates::{SourceCode, Update, Updates},
    };

    use crate::debugger::{
        active_functions::ActiveFunctionsMessage, ActiveFunctions, Expression,
        RemoteProcess,
    };

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
        let mut script = Script::default();
        script.function("main", [], |s| {
            s.block(|s| {
                s.r("brk");
            })
            .r("eval");
        });
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

        let debugger = remote_process.to_debugger();
        let ActiveFunctions::Functions { mut functions } =
            debugger.active_functions
        else {
            panic!("Expected active functions to be displayed");
        };
        let mut function = functions.remove(0);
        let block = function.body.remove(0);
        let Expression::Block { mut expressions } = block else {
            panic!("Expected block");
        };
        let Expression::Other { expression, .. } = expressions.remove(0) else {
            panic!("Expected builtin");
        };
        let FragmentExpression::ResolvedBuiltinFunction { name: builtin } =
            expression
        else {
            panic!("Expected builtin");
        };
        assert_eq!(builtin, "brk");
    }
}
