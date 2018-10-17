#![feature(trivial_bounds)]
#![feature(custom_attribute)]
#![feature(associated_type_defaults)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate failure;
extern crate futures;
extern crate hyper;

mod base;

mod test_plateform;

#[cfg(test)]
mod tests {
    use super::base::exchange::*;
    use super::base::http_connector::*;

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
        exchange.call_api("public", ExchangeApiMethod::Get, "Exchanges/{pair}/lasttrades");
    }
}
