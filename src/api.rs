//! Works with the API

use super::error::{APIError, Result};
use super::Params;
use reqwest::{Client, Response};
use serde_json::{from_value, Map, Value};

/// An API response, which is either an actual response or an error
pub type APIResponse = Result<Value>;

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

        let response_result: Result<Response> = self.client
            .get(&("https://api.vk.com/method/".to_owned() + method_name))
            .query(&params)
            .send()
            .map_err(|e| e.into());
        let mut response = response_result?;

        let value_result: Result<Value> = response.json().map_err(|e| e.into());
        let value = value_result?;

        let api_response_result: Result<&Map<String, Value>> = value
            .as_object()
            .ok_or("API response is not an object!".into());
        let api_response = api_response_result?;

        match api_response.get("response") {
            Some(ok) => Ok(ok.clone()),
            None => match api_response.get("error") {
                Some(err) => Err(from_value::<APIError>(err.clone())?.into()),
                None => Err("The API responded with neither a response nor an error!".into()),
            },
        }
    }
}
