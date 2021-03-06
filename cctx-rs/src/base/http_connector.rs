//!
//! Basic http connector that can make generic request from all exchanger
//! 
use super::exchange::*;
use super::errors::*;
use failure::Error;
use hyper::{Client};
use hyper::rt::{Future, Stream};
use hyper::client::HttpConnector as HyperHttpConnector;
use hyper_tls::HttpsConnector;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct HttpConnector {
    client: Client<HttpsConnector<HyperHttpConnector>>,
}

impl HttpConnector {
    pub fn new() -> HttpConnector {
        let https = HttpsConnector::new(4).unwrap();//TODO see how many blocking dns thread we want
        HttpConnector {
            client: Client::builder().build::<_, hyper::Body>(https)
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
        Box::new(match request.method {
            RequestMethod::Get(params) => {
                let mut concat = format!("{}?", request.path);
                for (index, param) in params.iter().enumerate() {
                    if index == params.len() - 1 {
                        concat.push_str(param);
                    } else {
                        concat.push_str(format!("{}&", param).as_str());
                    }
                }
                println!("@@ Send -> {:?}", concat);
                let concat = concat.parse();//IMPORTRTANT NOT UNWRAP
                self.client
                        .get(concat.unwrap_or_default())
                        .and_then(|res| res.into_body().concat2())
                        .map_err(|e| {
                            println!("@@ Send error : {}", e);
                            CCXTError::Undefined
                        })
                        .and_then(|body| Ok(serde_json::from_slice(&body)?))
                        .map_err(|e| {
                            println!("@@ Send error : {}", e);
                            CCXTError::ApiUrlMalformated.into()
                        })
            }
            _ => unimplemented!()
        })
    }
}