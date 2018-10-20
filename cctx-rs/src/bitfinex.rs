use std::sync::{Once, Arc, RwLock};
use super::prelude::*;
use futures::Future;
use futures::future::{ok, err};
use serde_json::Value;
use hyper::rt;

static INIT: Once = Once::new();
static mut BITFINEX_EXCHANGE: Option<Exchange<HttpConnector>> = None;

pub struct Bitfinex {
    exchange: Exchange<HttpConnector>,
}

impl Bitfinex {

    pub fn new() -> CCXTFut<Self> {
        INIT.call_once(||{
            unsafe {
                BITFINEX_EXCHANGE = Some(Exchange::<HttpConnector>::from_json(r#"
                {
                    "id": "bitfinex",
                    "name": "Bitfinex",
                    "api-urls": {
                        "public": "https://api.bitfinex.com/v1",
                        "private": "https://api.bitfinex.com/v1"
                    },
                    "api": {
                        'v2': {
                            'get': [
                                'candles/trade:{timeframe}:{symbol}/{section}',
                                'candles/trade:{timeframe}:{symbol}/last',
                                'candles/trade:{timeframe}:{symbol}/hist'
                            ]
                        },
                        "public": {
                            "get": [
                                "book/{symbol}",
                                "lendbook/{currency}",
                                "lends/{currency}",
                                "pubticker/{symbol}",
                                "stats/{symbol}",
                                "symbols",
                                "symbols_details",
                                "tickers",
                                "today",
                                "trades/{symbol}"
                            ]
                        }
                    },
                    "commonCurrencies": {
                        "ABS": "ABYSS",
                        "AIO": "AION",
                        "ATM": "ATMI",
                        "BCC": "CST_BCC",
                        "BCU": "CST_BCU",
                        "CTX": "CTXC",
                        "DAD": "DADI",
                        "DAT": "DATA",
                        "DSH": "DASH",
                        "HOT": "Hydro Protocol",
                        "IOS": "IOST",
                        "IOT": "IOTA",
                        "IQX": "IQ",
                        "MIT": "MITH",
                        "MNA": "MANA",
                        "NCA": "NCASH",
                        "ORS": "ORS Group",
                        "POY": "POLY",
                        "QSH": "QASH",
                        "QTM": "QTUM",
                        "SEE": "SEER",
                        "SNG": "SNGLS",
                        "SPK": "SPANK",
                        "STJ": "STORJ",
                        "YYW": "YOYOW",
                        "USD": "USDT",
                        "UTN": "UTNP"
                    }
                }
            "#).unwrap())
            }
        });
        let connector = HttpConnector::new();
        let mut exchange = unsafe {BITFINEX_EXCHANGE.as_ref().unwrap().clone()};
        exchange.set_connector(Box::new(connector));
        let mut exchange = Bitfinex { exchange };
        Box::from(exchange.fetch_markets().and_then(|_| ok(exchange)))
    }

}

// #macro_rules! value_unpack_or {
    // () => {
        // 
    // };
// }

use std::collections::HashMap;
impl ExchangeTrait for Bitfinex {

    fn fetch_ohlcv(&self, symbol: &str, timeframe: CandleTime, since: i64, limit: i64) -> FetchOhlcvResult {
        let market = self.exchange.get_market_by_symbol(symbol).unwrap();//TODO not unwrap
        let limit = format!("limit={}", limit);
        let since = format!("since={}", since);
        let v2id = format!("t{}", market.id);
        Box::from(self.exchange.call_api("public", ApiMethod::Get, "candles/trade:timeframe:symbol/section", &[limit.as_str(), since.as_str()]).and_then(move |json| {
            let ohlcv = Vec::<Ohlcv>::new();
            
            ok(ohlcv)
        }))
    }

    fn fetch_markets(&mut self) -> LoadMarketResult {
        fn parse_markets(re: Value) -> Result<HashMap<String, Market>, Error> {
            let mut markets = HashMap::<String, Market>::new();
            for market in as_array!(re, "markets")?.into_iter() {
                let pair = as_str!(market["pair"], "market->pair")?;
                let id = String::from(pair).to_uppercase();
                let base_id = String::from(&pair[0..3]);
                let quote_id = String::from(&pair[3..6]);
                let symbol = format!("{}/{}", base_id, quote_id);
                let price_precision = as_i64!(market["price_precision"], "market->precision")?;
                let limits_amount = (as_i64_or!(market["minimum_order_size"], 0) as f64, as_i64_or!(market["maximum_order_size"], 0) as f64);
                let limits_price = ((-price_precision).pow(10) as f64, price_precision.pow(10) as f64);
                let limits_cost = (limits_amount.0 * limits_price.0, 0.0);
                markets.insert(symbol.clone(), Market {
                    id,
                    symbol,
                    base_id,
                    quote_id,
                    active: true,
                    precision: (price_precision as f64, price_precision as f64),
                    limits: MarketLimits::new(limits_amount, limits_price, limits_cost),
                    info: None,
                });
            }
            Ok(markets)
        }
        let lock = self.exchange.market.clone();
        Box::from(self.exchange.call_api("public", ApiMethod::Get, "symbols_details", &[])
            .and_then(move |re| {
                match parse_markets(re) {
                    Ok(result) =>{ 
                        *lock.write().unwrap() = Some(result);
                        ok(lock)
                    },
                    Err(result) => {
                        err(result)
                    },
                }
            }))
    }

}


#[cfg(test)]
mod tests {
    use hyper::rt;
    use super::Bitfinex;
    use futures::future;
    use futures::Future;
    use futures::future::{ok, err};
    use crate::prelude::*;
    use crate::base::exchange::ExchangeTrait;
    #[test]
    fn test_plateform() {
        rt::run(future::lazy(move||{
            Bitfinex::new().and_then(|exchange| {
                println!("New bitfined exhange\nMarket : {:?}", exchange.exchange.market);
                exchange.fetch_ohlcv("omg/usd", CandleTime::_1M, 10, 100)
            })
            .map(|_|{})
            .map_err(|_|{})
        }));
    }
}