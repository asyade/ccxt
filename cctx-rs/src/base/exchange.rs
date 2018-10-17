//!
//! Base exchange traits that will be implemented for all plateformes
//! 

// pub const USER_AGENTS_CHROME: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 Safari/537.36";
// pub const USER_AGENT_CHROME39: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.71 Safari/537.36";

pub struct ApiMetaInfos {
    publicAPI: bool,
    privateAPI: bool,
    CORS: bool,
    cancelOrder: bool,
    cancelOrders: bool,
    createDepositAddress: bool,
    createOrder: bool,
    createMarketOrder: bool,
    createLimitOrder: bool,
    deposit: bool,
    editOrder: bool,
    fetchBalance: bool,
    fetchClosedOrders: bool,
    fetchCurrencies: bool,
    fetchDepositAddress: bool,
    fetchDeposits: bool,
    fetchFundingFees: bool,
    fetchL2OrderBook: bool,
    fetchMarkets: bool,
    fetchMyTrades: bool,
    fetchOHLCV: bool,
    fetchOpenOrders: bool,
    fetchOrder: bool,
    fetchOrderBook: bool,
    fetchOrderBooks: bool,
    fetchOrders: bool,
    fetchTicker: bool,
    fetchTickers: bool,
    fetchTrades: bool,
    fetchTradingFees: bool,
    fetchTradingLimits: bool,
    fetchTransactions: bool,
    fetchWithdrawals: bool,
    withdraw: bool,
}

///
/// The generic Exchange wrapper
/// 
pub trait Exchange  {
    fn get_market(symbole: &str);
    fn describe();
    fn load_markets();
    fn fetch_markets();
    fn fetch_currencies();
    fn fetch_ticker();
    fn fetch_order_book();
    fn fetch_ohlcv();
    fn fetch_treads();
}

///
/// Connector is by default an Exchange associated type
/// It make connection betwen Exchanger and target plateforme
/// For now there is only an http connector but we can add more like WebSocket connector
/// 
pub trait Connector {
    fn post();
    fn put();
}

pub trait ExchangeApi : Sized
{
    fn get();
    fn set();
}
