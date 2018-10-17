//!
//! Base exchange traits that will be implemented for all plateformes
//! 

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
pub trait Exchange {
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

pub enum RequestApi {
    Public,
    Private,
}

pub struct RequestParam<'a> {
    key: &'a str,
    value: &'a str,
}

pub enum RequestMethod<'a> {
    Get(Vec<RequestParam<'a>>),
    Post(Vec<RequestParam<'a>>, &'a RequestBody),
}

pub struct Request<'a> {
    path: &'a str,
    methode: RequestMethod<'a>
}

pub trait Connector {
    fn request(&self, request: Request);
}

pub struct Credentials {}

pub trait Plateform {
    type Connector: Connector = HTTPConnector;
    fn get_creds(&self) -> Credentials;
    fn get_connector(&self) -> &Connector;
}

impl Exchange for Plateform {
    fn get_market(&self, symbole: &str) { unimplemented!() }
    fn load_markets(&self) { unimplemented!() }
    fn fetch_markets(&self) { unimplemented!() }
    fn fetch_currencies(&self) { unimplemented!() }
    fn fetch_ticker(&self) { unimplemented!() }
    fn fetch_order_book(&self) { unimplemented!() }
    fn fetch_ohlcv(&self) { unimplemented!() }
    fn fetch_treads(&self) { unimplemented!() }
}