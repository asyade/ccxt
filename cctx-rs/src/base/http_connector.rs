//!
//! Basic http connector that can make generic request from all exchanger
//! 
use super::exchange::*;
use super::errors::*;
use failure::Error;
use std::io;
use std::io::Write;
use hyper::{Client, Uri};
use hyper::rt::{self, Future, Stream};
use hyper::client::HttpConnector as HyperHTTPConnector;
use serde_json::Value;

pub struct HTTPConnector {
    client: Client<HyperHTTPConnector>,//TODO https...
}

impl HTTPConnector {
    pub fn new() -> HTTPConnector {
        HTTPConnector {
            client: Client::new(),
        }
    }

    fn fetch_json(&self, url: hyper::Uri) -> impl Future<Item=Value, Error=CCXTError> {
        self.client
            .get(url)
            .and_then(|res| res.into_body().concat2())
            .from_err::<CCXTError>()
            .and_then(|body| Ok(serde_json::from_slice(&body)?))
            .from_err()
    }
}

impl Connector for HTTPConnector {

    fn request(&self, request: Request) -> Result<ConnectorFuture<RequestResponse>, Error> {
        let url: Uri = request.path.parse()?;
        match request.methode {
            RequestMethod::Get(params) => {
                let future = self.fetch_json(url); 
            },
            RequestMethod::Post(params, body) => {

            }
        }        
        unimplemented!()
    }
}