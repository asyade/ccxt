#![feature(trivial_bounds)]
#![feature(custom_attribute)]
#![feature(associated_type_defaults)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate tokio;
extern crate tokio_core;
extern crate tokio_reactor;

extern crate failure;
extern crate futures;

extern crate hyper;
use futures::future::{ok, err};
use hyper::rt;

use tokio::prelude::*;

mod base;

mod test_plateform;

#[cfg(test)]
mod tests {
    use super::base::exchange::*;
    use super::base::http_connector::*;
    use hyper::rt;
    use futures::Future;
    use futures::future;

    #[test]
    fn test_plateform() {

        let mut exchange : Exchange<HttpConnector> = Exchange::<HttpConnector>::from_json(r#"
            {
                "id": "bitfinex",
                "name": "Bitfinex",
                "urls": {
                    "logo": "https://user-images.githubusercontent.com/1294454/27766244-e328a50c-5ed2-11e7-947b-041416579bb3.jpg",
                    "www": "https://www.bitfinex.com"
                },
                "api-urls": {
                    "public": "https://api.bitfinex.com",
                    "private": "https://api.bitfinex.com"
                },
                "api": {
                    "public": {
                        "get": [
                            "Exchanges/{pair}/Ticker",
                            "Exchanges/{pair}/orderbook",
                            "Exchanges/{pair}/trades",
                            "Exchanges/{pair}/lasttrades"
                        ]
                    },
                    "private": {
                        "post": [
                            "Merchant/CreateCheckout",
                            "Order/AddCoinFundsRequest",
                            "Order/AddFund",
                            "Order/AddOrder",
                            "Order/AddOrderMarketPriceBuy"
                        ],
                        "get": [
                            "Order/AccountHistory",
                            "Order/OrderHistory"
                        ]
                    }
                }
            }
            "#).unwrap();
        exchange.set_connector(Box::new(HttpConnector::new()));
        println!("{:?}", exchange);
        let mut fut = exchange.call_api("private", ExchangeApiMethod::Post, "Order/AddFund", &["Pair01"]);
        rt::run(future::lazy(move|| {
                                        fut.map_err(|e|{ 
                                            println!("Error : {}", e);
                                        })
                                        .map(|value|{
                                            println!("result {:?}", value);
                                        })
                                    }));
    }
}
