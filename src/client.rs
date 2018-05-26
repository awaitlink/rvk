use std::collections::HashMap;

use futures::{Future, Stream};
use tokio_core::reactor::Core;

use hyper::{Body, Client, client::HttpConnector};
use hyper_tls::HttpsConnector;

use serde_json::{from_slice, Value};

pub type APIResponse = Result<Value, APIError>;

pub struct APIError {
    code: u64,
    msg: String,
}

impl APIError {
    pub fn new(code: u64, msg: String) -> APIError {
        APIError { code, msg }
    }

    pub fn code(&self) -> u64 {
        self.code
    }

    pub fn msg(&self) -> &String {
        &self.msg
    }
}

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

    pub fn call_method<F>(
        &self,
        method_name: &str,
        params: HashMap<&str, &str>,
        then: F,
    ) -> impl Future
    where
        F: Fn(APIResponse),
    {
        let mut url = "https://api.vk.com/method/".to_owned() + method_name + "?v="
            + self.api_version + "&access_token=" + self.token;

        for (name, value) in params.iter() {
            url += &("&".to_owned() + name + "=" + value);
        }

        let url = url.parse().unwrap();

        self.client.get(url).and_then(|res| {
            res.body().concat2().map(move |body| {
                let data: Value = from_slice(&body).unwrap();
                let response = data.as_object().unwrap();

                match response.get("response") {
                    Some(ok) => then(Ok(ok.clone())),
                    None => match response.get("error") {
                        Some(err) => {
                            let error = err.as_object().unwrap();

                            let code = error.get("error_code").unwrap().as_u64().unwrap();
                            let msg = error.get("error_msg").unwrap().as_str().unwrap();

                            let api_err = APIError::new(code, msg.to_owned());

                            then(Err(api_err));
                        }
                        None => unreachable!(),
                    },
                };
            })
        })
    }
}
