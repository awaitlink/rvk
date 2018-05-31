//! Defines an API client, response, and error

use super::error::Error;
use super::Params;
use reqwest::Client;
use serde_json::{from_value, Value};

/// An API response, which is either an actual response or an error
pub type APIResponse = Result<Value, Error>;

/// An error returned by the API
#[derive(Deserialize)]
pub struct APIError {
    pub code: u64,
    pub msg: String,
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
    pub fn call_method(&self, method_name: &str, mut params: Params<'a>) -> APIResponse {
        params.insert("v", self.api_version);
        params.insert("access_token", self.token);

        let mut res = self.client
            .get(&("https://api.vk.com/method/".to_owned() + method_name))
            .query(&params)
            .send()
            .map_err(|e| Error::request_err("Can't make a request").add_info(&e.to_string()))?;

        let data: Value = res.json().map_err(|e| {
            Error::serde_err("Can't deserialize API response").add_info(&e.to_string())
        })?;

        let response = data.as_object()
            .ok_or(Error::serde_err("API response is not an object!"))?;

        match response.get("response") {
            Some(ok) => Ok(ok.clone()),
            None => match response.get("error") {
                Some(err) => Err(from_value::<APIError>(err.clone())?.into()),
                None => Err(Error::other_err(
                    "The API responded with neither a response nor an error!",
                )),
            },
        }
    }
}
