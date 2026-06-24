use agentx::core::worker::Fault;
use agentx::{AppError, Worker};

#[test]
fn fault_classifies_exhausted () {

    assert!(matches!(Worker::fault(&AppError::message("Usage limit reached")), Fault::Exhausted));
    assert!(matches!(Worker::fault(&AppError::message("quota exceeded")), Fault::Exhausted));

}

#[test]
fn fault_classifies_session () {

    assert!(matches!(Worker::fault(&AppError::message("session not found")), Fault::Session));

}

#[test]
fn fault_classifies_fatal () {

    assert!(matches!(Worker::fault(&AppError::message("Invalid API key")), Fault::Fatal));
    assert!(matches!(Worker::fault(&AppError::message("model not found")), Fault::Fatal));

}

#[test]
fn fault_defaults_to_transient () {

    assert!(matches!(Worker::fault(&AppError::message("connection reset by peer")), Fault::Transient));
    assert!(matches!(Worker::fault(&AppError::timeout("gate", 5)), Fault::Transient));

}

#[test]
fn fault_reads_command_stderr () {

    let error = AppError::command("claude", 1, "fatal: invalid api key provided");

    assert!(matches!(Worker::fault(&error), Fault::Fatal));

}

#[test]
fn session_dsl_round_trip () {

    let mut worker = Worker::new("claude");

    assert!(worker.session().is_none());

    worker.set_session("abc-123");

    assert_eq!(worker.session(), Some("abc-123"));

    worker.clear();

    assert!(worker.session().is_none());

}
