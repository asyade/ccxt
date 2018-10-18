use std::sync::Once;
use super::prelude::*;

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
                        },
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

impl ExchangeTrait for Bitfinex {

    fn load_markets(&mut self) -> CCXTFut<LoadMarketsResult>{
        unimplemented!()
    }

}


#[cfg(test)]
mod tests {
    use hyper::rt;
    use super::Bitfinex;

    #[test]
    fn test_plateform() {
        let exchange: Bitfinex = Bitfinex::new();

        // let mut fut = exchange.call_api("public", ExchangeApiMethod::Get, "symbols_details", &[]);
        // rt::run(future::lazy(move|| {
                                        // fut.map_err(|e|{ 
                                            // println!("future : {}", e);
                                        // })
                                        // .map(|value|{
                                            // println!("result {:?}", value);
                                        // })
                                    // }));
    }
}
