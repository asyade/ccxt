//!
//! Base exchange traits that will be implemented for all plateformes
//! 
use super::http_connector::HTTPConnector;
use super::errors::*;

use std::collections::HashMap;
use failure::Error;
use hyper::rt::{Future};
use serde_json::value::Value;
use serde_json;

// pub const USER_AGENTS_CHROME: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 Safari/537.36";
// pub const USER_AGENT_CHROME39: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.71 Safari/537.36";
// 
// pub struct ApiMetaInfos {
    // publicAPI: bool,
    // privateAPI: bool,
    // CORS: bool,
    // cancelOrder: bool,
    // cancelOrders: bool,
    // createDepositAddress: bool,
    // createOrder: bool,
    // createMarketOrder: bool,
    // createLimitOrder: bool,
    // deposit: bool,
    // editOrder: bool,
    // fetchBalance: bool,
    // fetchClosedOrders: bool,
    // fetchCurrencies: bool,
    // fetchDepositAddress: bool,
    // fetchDeposits: bool,
    // fetchFundingFees: bool,
    // fetchL2OrderBook: bool,
    // fetchMarkets: bool,
    // fetchMyTrades: bool,
    // fetchOHLCV: bool,
    // fetchOpenOrders: bool,
    // fetchOrder: bool,
    // fetchOrderBook: bool,
    // fetchOrderBooks: bool,
    // fetchOrders: bool,
    // fetchTicker: bool,
    // fetchTickers: bool,
    // fetchTrades: bool,
    // fetchTradingFees: bool,
    // fetchTradingLimits: bool,
    // fetchTransactions: bool,
    // fetchWithdrawals: bool,
    // withdraw: bool,
//} 

///
/// The generic Exchange wrapper
/// 
pub trait ExchangeTrait {
    fn get_market(&self, symbole: &str);
    fn load_markets(&self);
    fn fetch_markets(&self);
    fn fetch_currencies(&self);
    fn fetch_ticker(&self);
    fn fetch_order_book(&self);
    fn fetch_ohlcv(&self);
    fn fetch_treads(&self);
}

///
/// Connector is by default an Exchange associated type
/// It make connection betwen Exchanger and target plateforme
/// For now there is only an http connector but we can add more like WebSocket connector
/// 

pub trait RequestBody {}

pub struct RequestParam<'a> {
    key: &'a str,
    value: &'a str,
}

pub enum RequestMethod<'a> {
    Get(Vec<RequestParam<'a>>),
    Post(Vec<RequestParam<'a>>, &'a RequestBody),
}

pub struct Request<'a> {
    pub path: &'a str,
    pub methode: RequestMethod<'a>
}

pub enum RequestResponse {
    Error,
    Success(Value),
}


pub type ConnectorFuture<T> = Box<Future<Item=T, Error=Error>>;
pub trait Connector {
    fn request(&self, request: Request) -> Result<ConnectorFuture<RequestResponse>, Error>;
}

pub struct Credentials {}

#[derive(Debug)]
pub enum ExchangeApiRoute {
    Static(String),
    #[derive(Debug)]
    Formatable{
        format: String,
        values: Vec<String>
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Exchange {
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

impl Default for Exchange {
    fn default() -> Exchange{
        Exchange {
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

impl Exchange {

    pub fn call_api(&self, api: &str, method: ExchangeApiMethod, route: &str) -> Result<(), Error> {
        if let Some(api) = self.api.get(api) {
            if let Some(methods) = match method {
                ExchangeApiMethod::Get => &api.get,
                ExchangeApiMethod::Post => &api.post,
            }
            {
                if let Some(method) = methods.get(route) {
                    println!("Called !");
                    return Ok(())
                }
            }
        }
        Err(CCXTError::Undefined.into())
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
    pub fn from_json(settings: &str) -> Result<Exchange, Error> {
        let settings: Value = serde_json::from_str(settings)?;
        let mut new_exchange = Exchange::default();

        println!("Generating exhcange ...");
        //Load id
        if !settings.is_object() {return Err(CCXTError::Undefined.into())}
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
                            let new_routes: HashMap<String, ExchangeApiRoute> = HashMap::new();
                            routes.iter().for_each(|e|{
                                if let Value::String(e) = e {
                                    new_routes.insert(e.clone(), ExchangeApiRoute::Static(e.clone())); //TODO parse formate
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