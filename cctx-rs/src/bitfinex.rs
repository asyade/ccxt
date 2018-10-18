use std::sync::Once;
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
        Box::new(self.exchange.call_api("public", ApiMethod::Get, "symbols_details", &[])
            .and_then(|re| {
                //println!("{:?}", re);

                if let Value::Array(markets) = re {
                    markets.into_iter().for_each(|elem|{
                        let pair = elem["pair"].as_str();
                        let pair = if pair.is_none() { return } else { pair.unwrap() };
                        if pair.len() != 6 { return }
                        let baseId = &pair[0..3];
                        let quoteId = &pair[3..6];
                        let symbol = format!("{}/{}", baseId, quoteId);
                        
                        let price_precision = elem["price_precision"].as_str();
                        let price_precision = if price_precision.is_none() { return } else { price_precision.unwrap() };

                        println!("Quote {} Base {}", quoteId, baseId);
                    });
                }
                let result = LoadMarketsResult{
                    markets: HashMap::new(),
                    // id: String::new(),
                    // baseId: 0,
                    // quoteId: 0,
                    // active: true,
                    // symbol: CCXTSymbol::Undefined,
                    // precision: 0,
                    // limits: (0.0,0.0),
                    // info: Value::Null
                };
                ok(result)
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
