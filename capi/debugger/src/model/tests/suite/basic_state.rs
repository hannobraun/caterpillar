use crate::model::{
    active_functions::ActiveFunctionsMessage, tests::infra::debugger,
    ActiveFunctions,
};

#[test]
fn no_server() {
    // If `RemoteProcess` has received no updates at all, the active functions
    // view should display that no server is available.

    let transient = debugger().state.generate_transient_state();

    assert_eq!(
        transient.active_functions,
        ActiveFunctions::Message {
            message: ActiveFunctionsMessage::NoServer
        }
    );
}

#[test]
fn no_process() {
    // If `RemoteProcess` has received a code update but no runtime updates, the
    // active functions view should display that no process is available.

    let transient = debugger()
        .provide_source_code("")
        .state
        .generate_transient_state();

    assert_eq!(
        transient.active_functions,
        ActiveFunctions::Message {
            message: ActiveFunctionsMessage::NoProcess
        }
    );
}
