use std::sync::Once;
#[macro_use]
use super::prelude::*;
use futures::Future;
use futures::future::{ok, err};
use serde_json::Value;
use std::collections::HashMap;

static INIT: Once = Once::new();
static mut BITFINEX_EXCHANGE: Option<Exchange<HttpConnector>> = None;

pub struct Bitfinex {
    exchange: Exchange<HttpConnector>,
}

impl Bitfinex {

    pub fn new() -> Self {
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
        Bitfinex { exchange }
    }

}

// #macro_rules! value_unpack_or {
    // () => {
        // 
    // };
// }

impl ExchangeTrait for Bitfinex {

    fn load_markets(&mut self) -> CCXTFut<LoadMarketsResult>{
        fn parse_markets(re: Value) -> Result<LoadMarketsResult, Error> {
            let result = LoadMarketsResult {
                markets: Vec::new(),
            };
            for market in as_array!(re, "markets")?.into_iter() {
                let pair = as_str!(market["pair"], "market->pair")?;
                let id = String::from(pair).to_uppercase();
                let base_id = &pair[0..3];
                let quote_id = &pair[3..6];
                let symbol = format!("{}/{}", base_id, quote_id);
                let price_precision = as_i64!(market["price_precision"], "market->precision")?;
                let limits_amount = (as_i64_or!(market["minimum_order_size"], 0) as f64, as_i64_or!(market["maximum_order_size"], 0) as f64);
                let limits_price = ((-price_precision).pow(10) as f64, price_precision.pow(10) as f64);
                let limits_cost = (limits_amount.0 * limits_price.0, 0.0);
                // result.markets.push(Market {
                    // id,
                    // symbol,
                    // 
                // });
            }
            Ok(result)
        }
        Box::from(self.exchange.call_api("public", ApiMethod::Get, "symbols_details", &[])
            .and_then(|re| {
                match parse_markets(re) {
                    Ok(result) => ok(result),
                    Err(result) => {
                        println!("load_markets->{}", result);
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
    use crate::base::exchange::ExchangeTrait;
    #[test]
    fn test_plateform() {
        let mut exchange: Bitfinex = Bitfinex::new();

        rt::run(future::lazy(move||{
            exchange.load_markets()
                    .map(|_|{})
                    .map_err(|_|{})
        }));
    }
}