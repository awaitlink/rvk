//! Defines custom error types used in this crate

use super::api::APIError;
use serde_json::Error as SerdeError;
use std::convert::Into;
use std::fmt::{self, Display, Formatter};
use std::io;

/// The error kind
#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    API,
    Request,
    Serde,
    Other,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(match self {
            &ErrorKind::API => "API",
            &ErrorKind::Request => "Request",
            &ErrorKind::Serde => "Serde",
            &ErrorKind::Other => "Other",
        })
    }
}

/// An error
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    description: String,
}

impl Error {
    /// Creates a new `Error`
    pub fn new(kind: ErrorKind, description: String) -> Error {
        Error { kind, description }
    }

    /// Convenience function for creating errors with ErrorKind::API
    pub fn api_err(description: &str) -> Error {
        Error::new(ErrorKind::API, description.to_string())
    }

    /// Convenience function for creating errors with ErrorKind::Other
    pub fn other_err(description: &str) -> Error {
        Error::new(ErrorKind::Other, description.to_string())
    }

    /// Convenience function for creating errors with ErrorKind::Serde
    pub fn serde_err(description: &str) -> Error {
        Error::new(ErrorKind::Serde, description.to_string())
    }

    /// Convenience function for creating errors with ErrorKind::Request
    pub fn request_err(description: &str) -> Error {
        Error::new(ErrorKind::Request, description.to_string())
    }

    /// Adds additional information to the error
    pub fn add_info(mut self, info: &str) -> Error {
        self.description += &("; ".to_string() + info);
        self
    }

    /// Returns this Error's kind
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns this Error's description
    pub fn description(&self) -> &String {
        &self.description
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&format!(
            "[rvk] {} error: {}",
            self.kind(),
            self.description()
        ))
    }
}

impl From<APIError> for Error {
    fn from(err: APIError) -> Error {
        Error::api_err(&format!("code: {}, message: {}", err.code, err.msg))
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Error {
        Error::serde_err(&err.to_string())
    }
}

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self.to_string())
    }
}
