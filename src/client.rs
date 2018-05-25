use std::collections::HashMap;
use futures::Future;
use tokio_core::reactor::Core;
use hyper::{
    Client, Body,
    client::{
        HttpConnector,
        FutureResponse,
    }
};
use hyper_tls::HttpsConnector;

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

    pub fn call_method(&self, method_name: &str, params: HashMap<&str, &str>) -> FutureResponse {
        let mut url = "https://api.vk.com/method/".to_owned();
        url += &(method_name.to_owned() + "?v=" + self.api_version + "&token=" + self.token);

        for (name, value) in params.iter() {
            url += &("&".to_owned() + name + "=" + value);
        }

        let url = url.parse().unwrap();
        self.client.get(url)
    }
}