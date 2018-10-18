//!
//! Base exchange traits that will be implemented for all plateformes
//! 
use super::http_connector::HttpConnector;
use super::errors::*;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::collections::HashMap;
use failure::Error;
use hyper::rt::{Future};
use futures::future::{ok, err};
use serde_json::value::Value;
use serde_json;
use hyper::Uri;
use hyper;

// pub const USER_AGENTS_CHROME: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 Safari/537.36";
// pub const USER_AGENT_CHROME39: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.71 Safari/537.36";
// 

///
/// Connector is by default an Exchange associated type
/// It make connection betwen Exchanger and target plateforme
/// For now there is only an http connector but we can add more like WebSocket connector
/// 

///
/// Http request body (will be json)
/// 
pub trait RequestBody {}

///
/// Http request parametter
/// 
pub struct RequestParam<'a> {
    key: &'a str,
    value: &'a str,
}

///
/// Http request type enum with is parametters
/// 
pub enum RequestMethod<'a> {
    Get(Vec<RequestParam<'a>>),
    Post(Vec<RequestParam<'a>>, &'a RequestBody),
}

pub struct Request<'a> {
    pub path: hyper::Uri,
    pub method: RequestMethod<'a>
}

impl <'a>Request<'a> {
    pub fn new(path: Uri, method: RequestMethod<'a>) -> Self {
        Request {
            path: path,
            method,
        }
    }
}

///
/// Http request response value (Parsed json)
/// @TODO maybe do not parse the json and parse it later strongely typed
/// 
pub enum RequestResponse {
    Error,
    Success(Value),
}

pub type ConnectorFuture<T> = Box<Future<Item=T, Error=Error> + Send>;

pub trait Connector {
    fn request(&self, request: Request) -> ConnectorFuture<Value>;
}

///
/// @TODO
/// 
pub struct Credentials {}


///
///
/// The generic Exchange wrapper
/// 
/// 

pub type CCXTFut<T> = Box<Future<Item=T, Error=CCXTError>>;

pub struct LoadMarketsResult {

}

pub trait ExchangeTrait {
    //fn get_market(&self, symbole: &str);
    fn load_markets(&mut self) -> CCXTFut<LoadMarketsResult>;
    //fn fetch_markets(&self);
    //fn fetch_currencies(&self);
    //fn fetch_ticker(&self);
    //fn fetch_order_book(&self);
    //fn fetch_ohlcv(&self);
    //fn fetch_treads(&self);
}


#[derive(Debug, Clone)]
pub enum ExchangeApiRoute {
    Static(String),
    Formatable(String),
}

impl ExchangeApiRoute {
    pub fn get_str(&self, params: &[&str]) -> String {
        match self {
            ExchangeApiRoute::Static(s) => s.clone(),
            ExchangeApiRoute::Formatable(format) => {
                let mut value = String::new();
                let mut varname: Option<String> = None;
                let mut param_index: usize = 0;
                format.chars().enumerate().for_each(|(_, c)| {
                    if varname.is_none() && c != '{' {
                        value.push(c);
                    } else if c == '{' {
                        varname = Some(String::new());
                    } else if c == '}' {
                        varname = None;
                        if param_index < params.len() {value.push_str(params[param_index])}
                        param_index += 1;
                    }
                });
                value
            }
        }
    }
}

impl Display for ExchangeApiRoute {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ExchangeApiRoute::Static(value) => {
                write!(f, "{}", value)
            },
            ExchangeApiRoute::Formatable(format) => {
                write!(f, "{}", format)
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExchangeApi {
    get: Option<HashMap<String, ExchangeApiRoute>>,
    post: Option<HashMap<String, ExchangeApiRoute>>,
}

pub enum ExchangeApiMethod {
    Get,
    Post,
}

impl Default for ExchangeApi {
    fn default() -> ExchangeApi {
        ExchangeApi {
            get: None,
            post: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Exchange<C: Connector + Debug + Clone> {
    connector: Option<Box<C>>,
    settings: Option<Value>,
    id: String,
    name: String,
    countries: Vec<String>,
    urls: HashMap<String, String>,
    api_urls: HashMap<String, String>,
    api: HashMap<String, ExchangeApi>,
    rate_limit: Option<u32>,
    certified: bool,
}

impl <C: Debug + Connector + Clone>Default for Exchange<C>  {
    fn default() -> Exchange<C>{
        Exchange {
            ///@TODO Non parsed value, will be deleted
            connector: None,
            settings: None,
            id: String::new(),
            name: String::new(),
            countries: Vec::new(),
            urls: HashMap::new(),
            api_urls: HashMap::new(),
            api: HashMap::new(),
            rate_limit: None,
            certified: false,
        }
    }
}

impl <T: Debug + Connector + Clone> Exchange<T> {

    pub fn call_api(&self, api: &str, method: ExchangeApiMethod, route: &str, params: &[&str]) -> ConnectorFuture<Value> {
        // Get api definition from given api name and check if it's None
        let api_def = self.api.get(api);
        if api_def.is_none() { return Box::new(err(CCXTError::ApiUrlNotFound.into())) }
        let api_def = api_def.unwrap();
        // Now we can get methods (like get, post, put) contained into the api definition
        let methods = match method {
            ExchangeApiMethod::Get => &api_def.get,
            ExchangeApiMethod::Post => &api_def.post,
        };
        if methods.is_none() { return Box::new(err(CCXTError::ApiMethodNotFound.into())) }
        // If we're sure that the method exist looking for the wanted route
        let route = methods.as_ref().unwrap().get(route);
        if route.is_none() { return Box::new(err(CCXTError::ApiMethodNotFound.into())) }
        let route = route.unwrap();
        // So we've got te route, now we want to link our api name with the real api url
        let api_url = self.api_urls.get(api);
        if api_url.is_none() { return Box::new(err(CCXTError::ApiUrlNotFound.into())) }        
        let api_url = api_url.unwrap();
        // And we've got the Api url and the route, so let's formate it
        let request_url = format!("{}/{}", api_url, route.get_str(params)).parse();
        if request_url.is_err() { return Box::from(err(CCXTError::ApiUrlMalformated.into())) }

        // Make the http request (lot of things TODO here)
        let request = Request::new(request_url.unwrap(), RequestMethod::Get(Vec::new()));
        if let Some(connector) = self.connector.as_ref() {
            Box::from(connector.request(request))
        } else {
            Box::from(err(CCXTError::ApiUrlNotFound.into()))
        }
    }

    pub fn set_connector(&mut self, connector: Box<T>) {
        self.connector = Some(connector);
    }

    ///
    /// Ex json:
    /// {
    ///     "id": "my-id",
    ///     "name": "my-name",
    ///     "urls": {
    ///         "url1": "www.231",   
    ///         "url2": "www.989",  
    ///     }
    ///     "api_urls": {
    ///         "url1": "www.231",   
    ///         "url2": "www.989",  
    ///     }
    /// }
    /// 
    pub fn from_json<C: Debug + Connector + Clone>(settings: &str) -> Result<Exchange<C>, Error> {
        let settings: Value = serde_json::from_str(settings)?;
        let mut new_exchange = Exchange::default();

        println!("Generating exhcange ...");
        //Load id
        if !settings.is_object() { return Err(CCXTLoadingError::UndefinedField{ field: String::from("settings")}.into())}
        if let Value::String(id) = &settings["id"] {
            new_exchange.id = id.clone();
        } else {
            return Err(CCXTLoadingError::UndefinedField{ field: String::from("id")}.into())
        }
        //Load name
        if let Value::String(name) = &settings["name"] {
            new_exchange.name = name.clone();
        } else {
            return Err(CCXTLoadingError::UndefinedField{ field: String::from("id")}.into())
        }
        //Load urls 
        if let Value::Object(urls) = &settings["urls"] {
            urls.iter().for_each(|(k, value)| {
                match value {
                    Value::String(o) => {
                        new_exchange.urls.insert(k.clone(), o.clone());
                    },
                    _ => {}
                }
            });
        } else {
            return Err(CCXTLoadingError::UndefinedField{ field: String::from("urls")}.into())
        }
        // Load api urls
        if let Value::Object(urls) = &settings["api-urls"] {
            urls.iter().for_each(|(k, value)| {
                match value {
                    Value::String(o) => {
                        new_exchange.api_urls.insert(k.clone(), o.clone());
                    },
                    _ => {}
                }
            });
        }
        // Load apis
        if let Value::Object(api) = &settings["api"] {
            api.iter().for_each(|(k, value)| {
                if let Value::Object(method) = value {
                    let mut newapi = ExchangeApi::default();
                    method.iter().for_each(|(k, routes)|{
                        if let Value::Array(routes) = routes {
                            let mut new_routes: HashMap<String, ExchangeApiRoute> = HashMap::new();
                            routes.iter().for_each(|e|{
                                if let Value::String(e) = e {
                                    new_routes.insert(e.clone(), {
                                        if e.contains("{") {
                                            ExchangeApiRoute::Formatable(e.clone())
                                        } else {
                                            ExchangeApiRoute::Static(e.clone())
                                        }
                                    });
                                }
                            });
                            match k.clone().as_str() {
                                "get" => {
                                    newapi.get = Some(new_routes);
                                },
                                "post" => {
                                    newapi.post = Some(new_routes);
                                },
                                _ => {
                                    println!("Invalide api : {:?}", routes)
                                }
                            };
                        }
                    });
                    new_exchange.api.insert(k.clone(), newapi);
                }
            });
        }
        Ok(new_exchange)
    }

}