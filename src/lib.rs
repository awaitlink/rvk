extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::Future;
use tokio_core::reactor::Core;
use hyper::{
    Client, Body,
    client::{
        HttpConnector,
        FutureResponse,
    }
};

pub struct APIClient {
    api_key: String,
    client: Client<HttpConnector, Body>,
    core: Core,
}

impl APIClient {
    pub fn new(api_key: String) -> APIClient {
        let core = Core::new().unwrap();

        APIClient {
            api_key,
            client: Client::new(&core.handle()),
            core,
        }
    }

    pub fn run<F: Future>(&mut self, work: F) -> Result<F::Item, F::Error> {
        self.core.run(work)
    }

    pub fn request(&self, uri: &str) -> FutureResponse {
        let uri = uri.parse().unwrap();
        self.client.get(uri)
    }
}