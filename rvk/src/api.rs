//! Performs calls to the VK API.

use crate::error::{APIError, Result};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde_json::{from_value, Map, Value};
use std::collections::HashMap;

/// A HashMap which contains method parameters
pub type Params = HashMap<String, String>;

/// An API client used to call API methods.
#[derive(Debug)]
pub struct APIClient {
    client: Client,
    api_version: String,
    token: String,
}

impl APIClient {
    /// Creates a new `APIClient`, given a VK API version and an access token.
    ///
    /// # Panics
    /// This method panics if native TLS backend cannot be created or initialized by the `reqwest` crate.
    ///
    /// See [reqwest docs](https://docs.rs/reqwest/0.10/reqwest/struct.Client.html#panic) for more information.
    pub fn new(api_version: impl Into<String>, token: impl Into<String>) -> APIClient {
        APIClient {
            client: Client::new(),
            api_version: api_version.into(),
            token: token.into(),
        }
    }

    /// Calls an API method, given its name and parameters.
    pub async fn call_method<T: DeserializeOwned>(
        &self,
        method_name: &str,
        mut params: Params,
    ) -> Result<T> {
        params.insert("v".into(), self.api_version.clone());
        params.insert("access_token".into(), self.token.clone());

        let response_result: Result<Response> = self
            .client
            .get(&("https://api.vk.com/method/".to_owned() + method_name))
            .query(&params)
            .send()
            .await
            .map_err(|e| e.into());
        let response = response_result?;

        let value_result: Result<Value> = response.json().await.map_err(|e| e.into());
        let mut value = value_result?;

        let api_response_result: Result<&mut Map<String, Value>> = value
            .as_object_mut()
            .ok_or_else(|| "API response is not an object!".into());
        let api_response = api_response_result?;

        match api_response.remove("response") {
            Some(ok) => Ok(from_value::<T>(ok)?),
            None => match api_response.remove("error") {
                Some(err) => Err(from_value::<APIError>(err)?.into()),
                None => Err("The API responded with neither a response nor an error!".into()),
            },
        }
    }
}
