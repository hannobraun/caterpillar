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
    use capi_process::Process;
    use capi_protocol::{
        memory::Memory,
        updates::{SourceCode, Update, Updates},
    };

    use crate::debugger::{
        active_functions::ActiveFunctionsMessage, ActiveFunctions,
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
}
