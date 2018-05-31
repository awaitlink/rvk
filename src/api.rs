//! Defines an API client, response, and error

use super::Params;
use reqwest::Client;
use serde_json::Value;
use std::error::Error as StdError;
use std::io::{Error, ErrorKind};

/// An API response, which is either an actual response or an error
pub type APIResponse = Result<Value, APIError>;

/// An error returned by the API
pub struct APIError {
    code: u64,
    msg: String,
}

impl APIError {
    /// Creates a new `APIError`
    pub fn new(code: u64, msg: String) -> APIError {
        APIError { code, msg }
    }

    /// Returns the error code
    pub fn code(&self) -> u64 {
        self.code
    }

    /// Returns the error message
    pub fn msg(&self) -> &String {
        &self.msg
    }

    /// Tries to convert a Value to an APIError
    // TODO: Implement `TryFrom` when it lands to stable
    pub fn from_value(err: &Value) -> Result<APIError, Error> {
        let error = err.as_object()
            .ok_or(other_error("Can't represent the error as an object!"))?;

        let code = error
            .get("error_code")
            .ok_or(other_error("Can't get the error code!"))?
            .as_u64()
            .ok_or(other_error("Can't represent the error code as u64!"))?;

        let msg = error
            .get("error_msg")
            .ok_or(other_error("Can't get the error message!"))?
            .as_str()
            .ok_or(other_error(
                "Can't represent the error message as a string!",
            ))?;

        Ok(APIError::new(code, msg.to_owned()))
    }
}

/// An API client used to call API methods
pub struct APIClient<'a> {
    client: Client,
    token: &'a str,
    api_version: &'a str,
}

impl<'a> APIClient<'a> {
    /// Creates a new `APIClient`, given an access token
    ///
    /// # Panics
    /// This method panics if native TLS backend cannot be created or initialized by the `reqwest` crate.
    pub fn new(token: &str) -> APIClient {
        APIClient {
            client: Client::new(),
            token,
            api_version: "5.78",
        }
    }

    /// Calls an API method, given its name and parameters
    pub fn call_method(
        &self,
        method_name: &str,
        mut params: Params<'a>,
    ) -> Result<APIResponse, Error> {
        params.insert("v", self.api_version);
        params.insert("access_token", self.token);

        let mut res = self.client
            .get(&("https://api.vk.com/method/".to_owned() + method_name))
            .query(&params)
            .send()
            .map_err(|e| other_error(&format!("Can't make a request! {}", e.description())))?;

        let data: Value = res.json().map_err(|e| {
            other_error(&format!(
                "Can't deserialize API response! {}",
                e.description()
            ))
        })?;

        let response = data.as_object()
            .ok_or(other_error("API response is not an object!"))?;

        match response.get("response") {
            Some(ok) => Ok(Ok(ok.clone())),
            None => match response.get("error") {
                Some(err) => Ok(Err(APIError::from_value(err)?)),
                None => Err(other_error(
                    "The API responded with neither a response nor an error!",
                )),
            },
        }
    }
}

/// Convenience function for making `std::io::Error`
fn other_error(msg: &str) -> Error {
    Error::new(ErrorKind::Other, msg)
}
