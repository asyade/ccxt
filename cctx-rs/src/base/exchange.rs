//!
//! Base exchange traits that will be implemented for all plateformes
//! 
use super::errors::*;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::collections::HashMap;
use failure::Error;
use hyper::rt::{Future};
use serde_json::value::Value;
use serde_json;
use hyper::Uri;
use hyper;

// pub const USER_AGENTS_CHROME: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 Safari/537.36";
// pub const USER_AGENT_CHROME39: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.71 Safari/537.36";
// 

///
///
/// Connector is by default an Exchange associated type
/// It make connection betwen Exchanger and target plateforme
/// For now there is only an http connector but we can add more like WebSocket connector
/// 
/// 

///
/// Http request body (will be json)
/// 
pub trait RequestBody {}

///
/// Http request parametter
/// 
pub struct RequestParam<'a> {
    _key: &'a str,
    _value: &'a str,
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

pub type CCXTFut<T> = Box<Future<Item=T, Error=Error> + Send>;

pub enum CCXTSymbol {
    Undefined,
}

///
/// amount (min, max)
/// price (min, max)
/// cost (min, max)
pub struct MarketLimits {
    pub amount: (f64, f64),
    pub price: (f64, f64),
    pub cost: (f64, f64),
}

impl MarketLimits {
    pub fn new(amount: (f64, f64), price: (f64, f64), cost: (f64, f64)) -> Self {
        MarketLimits {amount, price, cost}
    }
}

pub struct Market {
    pub id: String,
    pub symbol: String,
    pub base_id: String,
    pub quote_id: String,//Todo make enum referencin all symbols
    pub active: bool,
    pub precision: (f64,f64),
    pub limits: MarketLimits,
    pub info: Option<Value>,//Remove it if it's possible
}

pub struct LoadMarketsResult {
    pub markets: Vec<Market>
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

pub enum ApiMethod {
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
    common_currencies: HashMap<String, String>,
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
            common_currencies: HashMap::new(),
            rate_limit: None,
            certified: false,
        }
    }
}

impl <T>Exchange<T> where T: Connector + Debug + Clone {

    fn parse_api_call(&self, api: &str, method: ApiMethod, route: &str, params: &[&str]) -> Result<Request, Error> {
        let api_def = self.api.get(api).ok_or(CCXTError::ApiUrlNotFound)?;
        let method = match method {
            ApiMethod::Get => &api_def.get,
            ApiMethod::Post => &api_def.post,            
        }
            .as_ref()
            .ok_or(CCXTError::ApiUrlNotFound)?
            .get(route)
            .ok_or(CCXTError::ApiMethodNotFound)?;

        let api_url = self.api_urls.get(api).ok_or(CCXTError::ApiUrlNotFound)?;
        let url = format!("{}/{}", api_url, method.get_str(params)).parse()?;
        Ok(Request::new(url, RequestMethod::Get(Vec::new())))
    }

}

macro_rules! as_str {
    ($val:expr, $err:expr) => (
        $val.as_str().ok_or::<Error>(CCXTLoadingError::UndefinedField{field: String::from($err)}.into())
    );
}

macro_rules! as_object {
    ($val:expr, $err:expr) => (
        $val.as_object().ok_or::<Error>(CCXTLoadingError::UndefinedField{field: String::from($err)}.into())
    );
}

macro_rules! as_array {
    ($val:expr, $err:expr) => (
        $val.as_array().ok_or::<Error>(CCXTLoadingError::UndefinedField{field: String::from($err)}.into())
    );
}

macro_rules! as_i64 {
    ($val:expr, $err:expr) => (
        $val.as_i64().ok_or::<Error>(CCXTLoadingError::UndefinedField{field: String::from($err)}.into())
    );
}

macro_rules! as_i64_or {
    ($val:expr, $default:expr) => (
        $val.as_i64().unwrap_or($default)
    );
}

impl <T: Debug + Connector + Clone> Exchange<T> {

    pub fn get_currencies(&self) -> &HashMap<String, String> {
        return &self.common_currencies;
    }

    pub fn call_api(&self, api: &str, method: ApiMethod, route: &str, params: &[&str]) -> ConnectorFuture<Value> {
        let connector = try_future_box!(self.connector.as_ref().ok_or(CCXTError::Undefined));
        let request = try_future_box!(self.parse_api_call(api, method, route, params));
        Box::from(connector.request(request))
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
        as_object!(settings, "settings")?;
        new_exchange.id = String::from(as_str!(settings["id"], "id").unwrap_or(""));
        new_exchange.name = String::from(as_str!(settings["name"], "name").unwrap_or(""));

        //Load urls
        fn load_urls(list: &Value, output: &mut HashMap<String, String>) {
            as_object!(list, "urls").and_then(|settings| {for (key, param) in settings.iter() {
                as_str!(param, "urls->url").and_then(|value| {
                    output.insert(key.clone(), String::from(value));
                    Ok(())
                }).is_ok();
            } Ok(()) }).is_ok();
        }
        load_urls(&settings["urls"], &mut new_exchange.urls);
        load_urls(&settings["api-urls"], &mut new_exchange.api_urls);

        //Load api
        for (key, api) in as_object!(settings["api"], "api")? {
            let mut newapi = ExchangeApi::default();
            for (route_key, routes) in as_object!(api, format!("api->{}", key))? {
                let mut newroutes: HashMap<String, ExchangeApiRoute> = HashMap::new();
                for route in as_array!(routes, "api->method->routes")? {
                    let route = String::from(as_str!(route, "api->method->routes->route")?); 
                    newroutes.insert(route.clone(), {
                        if route.contains("{") {
                            ExchangeApiRoute::Formatable(route.clone())
                        } else {
                            ExchangeApiRoute::Static(route.clone())
                        }
                    });
                }
                if newroutes.len() == 0 {continue;} 
                match route_key.as_ref() {
                    "get" => { newapi.get = Some(newroutes); },
                    "post" => { newapi.get = Some(newroutes); },
                    _ => { println!("Undefined api methode : {}", route_key) }
                }
            }
            new_exchange.api.insert(String::from(key.as_ref()), newapi);
        }

        //Load copmmon currencies
        for (key, value) in as_object!(settings["commonCurrencies"], "commonCurrencies")? {
            new_exchange.common_currencies.insert(key.clone(), String::from(as_str!(value, "commonCurrencies->name")?));
        }
        Ok(new_exchange)
    }

}