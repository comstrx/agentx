use std::io;

use agentx::{AppError, AppFail, AppResult};

#[test]
fn display_messages_are_readable () {

    assert_eq!(AppError::message("boom").to_string(), "boom");
    assert!(AppError::not_found("config.toml").to_string().contains("config.toml"));
    assert!(AppError::invalid("tests", "maybe").to_string().contains("invalid tests"));
    assert_eq!(AppError::timeout("gate", 30).to_string(), "gate timed out after 30s");
    assert!(AppError::command("claude", 2, "stderr").to_string().contains("claude"));

}

#[test]
fn variants_match_their_constructors () {

    assert!(matches!(AppError::not_found("x"), AppError::NotFound(_)));
    assert!(matches!(AppError::unsupported("y"), AppError::Unsupported(_)));
    assert!(matches!(AppError::timeout("g", 1), AppError::Timeout { .. }));
    assert!(matches!(AppError::network("http://x", "down"), AppError::Network { .. }));

}

#[test]
fn or_fail_wraps_none () {

    let value: Option<i32> = None;
    let result: AppResult<i32> = value.or_fail("missing value");

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("missing value"));

}

#[test]
fn or_fail_passes_some_through () {

    let value: Option<i32> = Some(7);

    assert_eq!(value.or_fail("unused").unwrap(), 7);

}

#[test]
fn or_fail_wraps_a_result_error () {

    let result: Result<(), io::Error> = Err(io::Error::other("disk gone"));
    let wrapped = result.or_fail("while saving");

    assert!(wrapped.is_err());
    assert!(wrapped.unwrap_err().to_string().contains("while saving"));

}

#[test]
fn or_fail_with_is_lazy_on_success () {

    let value: Option<i32> = Some(3);

    assert_eq!(value.or_fail_with(|| "never built").unwrap(), 3);

}

#[test]
fn io_error_converts_into_app_error () {

    let error: AppError = io::Error::other("nope").into();

    assert!(matches!(error, AppError::Io(_)));

}
