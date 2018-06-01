extern crate rvk;

use rvk::error::{APIError, Error};

#[test]
fn api_error() {
    let api_err = APIError::new(0, "test".to_string());
    let err: Error = api_err.clone().into();

    match err {
        Error::API(e) => assert_eq!(e, api_err),
        _ => unreachable!()
    }
}

#[test]
fn other_error_from_str() {
    let other_err = "error";
    let err: Error = other_err.clone().into();

    match err {
        Error::Other(s) => assert_eq!(s, other_err),
        _ => unreachable!()
    }
}

#[test]
fn other_error_from_string() {
    let other_err = "error".to_string();
    let err: Error = other_err.clone().into();

    match err {
        Error::Other(s) => assert_eq!(s, other_err),
        _ => unreachable!()
    }
}
