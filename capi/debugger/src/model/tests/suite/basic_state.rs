use crate::model::{
    active_functions::ActiveFunctionsMessage, tests::infra::init,
    ActiveFunctions,
};

#[test]
fn no_server() {
    // If `RemoteProcess` has received no updates at all, the active functions
    // view should display that no server is available.

    let (_, transient) = init().into_state();

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

    let (_, transient) = init().provide_source_code("").into_state();

    assert_eq!(
        transient.active_functions,
        ActiveFunctions::Message {
            message: ActiveFunctionsMessage::NoProcess
        }
    );
}
