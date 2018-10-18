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
use hyper::client::HttpConnector as HyperHttpConnector;
use serde_json::Value;

#[derive(Debug)]
pub struct HttpConnector {
    client: Client<HyperHttpConnector>,//TODO https...
}

impl HttpConnector {
    pub fn new() -> HttpConnector {
        HttpConnector {
            client: Client::new(),
        }
    }

}

impl From<CCXTError> for Box<Error> {
    fn from(error: CCXTError) -> Box<Error> {
        Box::new(error.into())
    }
}

impl Connector for HttpConnector {
    fn request(&self, request: Request) -> ConnectorFuture<Value> {
        println!("@@ Send -> {:?}", request.path);
        Box::new(match request.method {
            _ => {
                self.client
                        .get(request.path)
                        .and_then(|res| res.into_body().concat2())
                        .map_err(|e| {
                            println!("{}", e);
                            CCXTError::Undefined
                        })
                        .and_then(|body| Ok(serde_json::from_slice(&body)?))
                        .map_err(|e| {
                            println!("{}", e);
                            CCXTError::ApiUrlMalformated.into()
                        })
            }
        })
    }
}