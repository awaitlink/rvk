//! Represents errors that can happen during a method call.

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Convenience type for defining `Result`s.
pub type Result<T> = std::result::Result<T, Error>;

/// An error returned by the API.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct APIError {
    error_code: u64,
    error_msg: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl APIError {
    /// Creates a new `APIError`.
    pub fn new(code: u64, msg: String, extra: HashMap<String, Value>) -> Self {
        Self {
            error_code: code,
            error_msg: msg,
            extra,
        }
    }

    /// Returns the code of this `APIError`.
    ///
    /// ```
    /// # use rvk::error::APIError;
    /// # use std::collections::HashMap;
    ///
    /// let err = APIError::new(0, "test".into(), HashMap::new());
    /// assert_eq!(err.code(), 0);
    /// ```
    pub fn code(&self) -> u64 {
        self.error_code
    }

    /// Returns the message of this `APIError`.
    ///
    /// ```
    /// # use rvk::error::APIError;
    /// # use std::collections::HashMap;
    ///
    /// let err = APIError::new(0, "test".into(), HashMap::new());
    /// assert_eq!(err.msg(), "test");
    /// ```
    pub fn msg(&self) -> &String {
        &self.error_msg
    }

    /// Returns the extra fields of this `APIError`.
    ///
    /// ```
    /// # use rvk::error::APIError;
    /// # use std::collections::HashMap;
    /// # use serde_json::Value;
    ///
    /// let err = APIError::new(0, "test".into(), HashMap::new());
    /// assert_eq!(err.extra().clone(), HashMap::<String, Value>::new());
    /// ```
    pub fn extra(&self) -> &HashMap<String, Value> {
        &self.extra
    }
}

/// A generic error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Errors from the API.
    #[error("API Error #{}: {}", .0.error_code, .0.error_msg)]
    API(APIError),

    /// Errors with making a request.
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    /// Serialization/Deserialization errors.
    #[error("Serialization/Deserialization error: {0}")]
    Serde(#[from] serde_json::error::Error),

    /// Other errors.
    #[error("Other error: {0}")]
    Other(String),
}

impl From<APIError> for Error {
    fn from(e: APIError) -> Error {
        Error::API(e)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Error {
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
