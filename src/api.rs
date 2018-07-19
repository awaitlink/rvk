//! Works with the API

use super::error::{APIError, Result};
use super::API_VERSION;
use reqwest::{Client, Response};
use serde_json::{from_value, Map, Value};
use std::collections::HashMap;

/// A HashMap which contains method parameters
pub type Params = HashMap<String, String>;

/// An API client used to call API methods
pub struct APIClient {
    client: Client,
    token: String,
}

impl APIClient {
    /// Creates a new `APIClient`, given an access token
    ///
    /// # Panics
    /// This method panics if native TLS backend cannot be created or initialized by the `reqwest` crate.
    ///
    /// See [reqwest docs](https://docs.rs/reqwest/0.8.*/reqwest/struct.Client.html#panic) for more information.
    pub fn new(token: String) -> APIClient {
        APIClient {
            client: Client::new(),
            token,
        }
    }

    /// Calls an API method, given its name and parameters
    pub fn call_method(&self, method_name: &str, mut params: Params) -> Result<Value> {
        params.insert("v".into(), API_VERSION.into());
        params.insert("access_token".into(), self.token.clone());

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
            .ok_or_else(|| "API response is not an object!".into());
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
