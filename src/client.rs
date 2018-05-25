use std::collections::HashMap;

use futures::{Future, Stream};
use tokio_core::reactor::Core;

use hyper::{
    Client, Body,
    client::{
        HttpConnector,
    }
};
use hyper_tls::HttpsConnector;

use serde_json::{Value, from_slice};

pub struct APIClient<'a> {
    core: Core,
    client: Client<HttpsConnector<HttpConnector>, Body>,
    token: &'a str,
    api_version: &'a str,
}

impl<'a> APIClient<'a> {
    pub fn new(token: &str) -> APIClient {
        let core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);

        APIClient {
            core,
            client,
            token,
            api_version: "5.78",
        }
    }

    pub fn run<F: Future>(&mut self, work: F) -> Result<F::Item, F::Error> {
        self.core.run(work)
    }

    pub fn call_method<F>(&self, method_name: &str, params: HashMap<&str, &str>, then: F)
        -> impl Future
        where F: Fn(APIResponse)
    {
        let mut url = "https://api.vk.com/method/".to_owned();
        url += &(method_name.to_owned() + "?v=" + self.api_version + "&access_token=" + self.token);

        for (name, value) in params.iter() {
            url += &("&".to_owned() + name + "=" + value);
        }

        let url = url.parse().unwrap();

        self.client.get(url).and_then(|res| {
            res.body().concat2().map(move |body| {
                let data: Value = from_slice(&body).unwrap();
                then(APIResponse::Ok(data)); // TODO: Distinguish between Ok and Error
            })
        })
    }
}

pub enum APIResponse {
    Ok(Value),
    Error(u64, String),
}