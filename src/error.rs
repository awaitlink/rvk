//! Represents errors that can happen during a method call

use failure_derive::Fail;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Convenience type for defining `Result`s
pub type Result<T> = std::result::Result<T, Error>;

/// An error returned by the API
#[derive(Fail, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[fail(display = "API Error #{}: {}", error_code, error_msg)]
pub struct APIError {
    error_code: u64,
    error_msg: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl APIError {
    /// Creates a new `APIError`
    pub fn new(code: u64, msg: String, extra: HashMap<String, Value>) -> Self {
        Self {
            error_code: code,
            error_msg: msg,
            extra,
        }
    }

    /// Returns the code of this `APIError`
    ///
    /// ```
    /// use rvk::error::APIError;
    /// use std::collections::HashMap;
    ///
    /// let err = APIError::new(0, "test".to_string(), HashMap::new());
    /// assert_eq!(err.code(), 0);
    /// ```
    pub fn code(&self) -> u64 {
        self.error_code
    }

    /// Returns the message of this `APIError`
    ///
    /// ```
    /// use rvk::error::APIError;
    /// use std::collections::HashMap;
    ///
    /// let err = APIError::new(0, "test".to_string(), HashMap::new());
    /// assert_eq!(err.msg(), "test");
    /// ```
    pub fn msg(&self) -> &String {
        &self.error_msg
    }
}

/// A generic error
#[derive(Fail, Debug)]
pub enum Error {
    /// Errors from the API
    #[fail(display = "{}", _0)]
    API(#[cause] APIError),

    /// Errors with making a request
    #[fail(display = "Request error: {}", _0)]
    Request(#[cause] ::reqwest::Error),

    /// Serialization/Deserialization errors
    #[fail(display = "Serialization/Deserialization error: {}", _0)]
    Serde(#[cause] ::serde_json::error::Error),

    /// Other errors
    #[fail(display = "Other error: {}", _0)]
    Other(String),
}

impl From<APIError> for Error {
    fn from(e: APIError) -> Error {
        Error::API(e)
    }
}

impl From<::reqwest::Error> for Error {
    fn from(e: ::reqwest::Error) -> Error {
        Error::Request(e)
    }
}

impl From<::serde_json::error::Error> for Error {
    fn from(e: ::serde_json::error::Error) -> Error {
        Error::Serde(e)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::Other(s)
    }
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Error {
        s.to_string().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_error() {
        let api_err = APIError::new(0, "test".to_string(), HashMap::new());
        let err: Error = api_err.clone().into();

        match err {
            Error::API(e) => assert_eq!(e, api_err),
            _ => unreachable!(),
        }
    }

    #[test]
    fn other_error_from_str() {
        let other_err = "error";
        let err: Error = other_err.clone().into();

        match err {
            Error::Other(s) => assert_eq!(s, other_err),
            _ => unreachable!(),
        }
    }

    #[test]
    fn other_error_from_string() {
        let other_err = "error".to_string();
        let err: Error = other_err.clone().into();

        match err {
            Error::Other(s) => assert_eq!(s, other_err),
            _ => unreachable!(),
        }
    }
}
