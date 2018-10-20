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
use std::sync::{Arc, RwLock};

macro_rules! try_block {
    ($block:block) => (
        (|| -> Result<(), Error> { $block;  Ok(()) })().is_ok()
    );
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

macro_rules! as_f64 {
    ($val:expr, $err:expr) => (
        $val.as_f64().ok_or::<Error>(CCXTLoadingError::UndefinedField{field: String::from($err)}.into())
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
 
macro_rules! get_api {
    ($m:expr, $api:expr, $route:expr, $($params:expr), *) => ($m.call_api($api, ApiMethod::Get, $route, &[$($params),*]))
}

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
    Get(Vec<String>),
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
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Ohlcv {
    pub timestamp: i64,
    pub open: f64,
    pub highest: f64,
    pub lowest: f64,
    pub losing: f64,
    pub volume: f64,
}

#[repr(u32)]
pub enum CandleTime {
    _1M=1,
    _2M=2,
    _3M=3,
    _4M=4,
    _5M=5,
    _10M=10,
    _15M=15,
    _20M=20,
    _30M=30,
    _1H=60,
    _2H=60*2,
    _3H=60*3,
    _4H=60*4,
    _5H=60*5,
    _1D=60*24,
    _2D=60*24*2,
    _3D=60*24*3,
    _1W=60*24*7,
}

pub type FetchOhlcvResult = CCXTFut<Vec<Ohlcv>>;
pub type LoadMarketResult = CCXTFut<Arc<RwLock<Option<HashMap<String, Market>>>>>;

pub trait ExchangeTrait {
    fn fetch_ohlcv(&self, symbol: &str, timeframe: CandleTime, since: i64, limit: i64) -> FetchOhlcvResult;
    fn load_markets(&mut self) -> LoadMarketResult;
    //fn get_market(&self, symbole: &str);
    //fn fetch_markets(&self);
    //fn fetch_currencies(&self);
    //fn fetch_ticker(&self);
    //fn fetch_order_book(&self);
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
    pub common_currencies: HashMap<String, String>,
    rate_limit: Option<u32>,
    pub market: Arc<RwLock<Option<HashMap<String, Market>>>>,
    certified: bool,
}

impl <C: Debug + Connector + Clone>Default for Exchange<C>  {
    fn default() -> Exchange<C>{
        let mut currencies: HashMap<String, String> = HashMap::new();
        currencies.insert("XBT".into(), "BTC".into());
        currencies.insert("BCC".into(), "BCH".into());
        currencies.insert("DRK".into(), "DASH".into());
        Exchange {
            ///@TODO Non parsed value, will be deleted
            connector: None,
            settings: None,
            id: String::new(),
            name: String::new(),
            countries: Vec::new(),
            urls: HashMap::new(),
            api_urls: HashMap::new(),
            market: Arc::new(RwLock::new(None)),
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
        let get_param_count = api_url.chars().filter(|c| *c == '{').count();
        let url = format!("{}/{}", api_url, method.get_str(params)).parse()?;
        let mut get = Vec::new();
        for param in params[get_param_count..].iter() {
            get.push(String::from(*param));
        }
        Ok(Request::new(url, RequestMethod::Get(get)))
    }

}



impl <T: Debug + Connector + Clone> Exchange<T> {

    pub fn get_market_by_symbol(&self, symbol: &str) -> Option<Market> {
        let market = self.market.read().unwrap();
        market.as_ref().unwrap().get(symbol).and_then(|elem|Some(elem.clone()))
    }

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
                    newroutes.insert(route.clone().replace("{", "").replace("}", ""), {
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