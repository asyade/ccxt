use std::sync::{Once};//Arc, RwLock};
use super::prelude::*;
use std::collections::HashMap;
use chrono::naive::NaiveDateTime;
use futures::Future;
use futures::future::{ok, err};
use serde_json::Value;
//use hyper::rt;
//use std::cmp::{max, min};

static INIT: Once = Once::new();
static mut BITMEX_EXCHANGE: Option<Exchange<HttpConnector>> = None;

pub struct Bitmex {
    exchange: Exchange<HttpConnector>,
}

impl Bitmex {

    pub fn new() -> CCXTFut<Self> {
        INIT.call_once(||{
            unsafe {
                BITMEX_EXCHANGE = Some(Exchange::<HttpConnector>::from_json(r#"
                {
                    "id": "Bitmex",
                    "name": "Bitmex",
                    "api-urls": {
                        "public": "https://www.bitmex.com/api/v1",
                        "private": "https://www.bitmex.com/api/v1"
                    },
                    "api": {
                        "public": {
                            "get": [
                                "announcement",
                                "announcement/urgent",
                                "funding",
                                "instrument",
                                "instrument/active",
                                "instrument/activeAndIndices",
                                "instrument/activeIntervals",
                                "instrument/compositeIndex",
                                "instrument/indices",
                                "insurance",
                                "leaderboard",
                                "liquidation",
                                "orderBook",
                                "orderBook/L2",
                                "quote",
                                "quote/bucketed",
                                "schema",
                                "schema/websocketHelp",
                                "settlement",
                                "stats",
                                "stats/history",
                                "trade",
                                "trade/bucketed"
                            ]
                        }
                    },
                    "commonCurrencies": {
                    }
                }
            "#).unwrap())
            }
        });
        let connector = HttpConnector::new();
        let mut exchange = unsafe {BITMEX_EXCHANGE.as_ref().unwrap().clone()};
        exchange.set_connector(Box::new(connector));
        let mut exchange = Bitmex { exchange };
        Box::from(exchange.fetch_markets().and_then(|_| ok(exchange)))
    }

    fn time_frame(time: CandleTime) -> &'static str {
        match time {
            CandleTime::_5M => "5m",
            CandleTime::_1H => "1h",
            CandleTime::_1D => "1d",
            _ => "1m",
        }
    }

    fn parse_ohlcv(json: Value) -> FetchOhlcvResult {
        let mut ohlcv = Vec::<Ohlcv>::new();
        try_block!({
            for elem in as_array!(json, "ohlcv->timestamp")? {
                let time =as_str!(elem["timestamp"], "ohlcv->timestamp")?;
                let timestamp = NaiveDateTime::parse_from_str(time, "%Y-%m-%dT%H:%M:%S.000Z")?;
                let open = as_f64!(elem["open"], "ohlcv->open")?;
                let highest = as_f64!(elem["high"], "ohlcv->high")?;
                let lowest = as_f64!(elem["low"], "ohlcv->low")?;
                let losing = as_f64!(elem["close"], "ohlcv->close")?;
                let volume = as_f64!(elem["volume"], "ohlcv->volume")?;
                ohlcv.push(Ohlcv{
                    timestamp: timestamp.timestamp() * 1000,
                    open,
                    highest,
                    lowest,
                    losing,
                    volume,
                })
            }
        });
        Box::from(ok(ohlcv))
    }

    fn parse_markets(re: Value) -> Result<HashMap<String, Market>, Error> {
        let mut markets = HashMap::<String, Market>::new();
        for market in as_array!(re, "markets")?.into_iter() {
            try_block!({
                let id: String = as_str!(market["symbol"], "market->symbol")?.into();
                let base_id = as_str!(market["underlying"], "market->base_id")?;
                let quote_id = as_str!(market["quoteCurrency"], "market->quote_id")?;
                let basequote = format!("{}{}", base_id, quote_id);
                let symbol = if id == basequote { format!("{}/{}", base_id, quote_id) } else { id.clone() };
                markets.insert(symbol.clone(), Market {
                    id,
                    symbol,
                    base_id: base_id.into(),
                    quote_id: quote_id.into(),
                    active: as_str!(market["state"], "market->state")? != "Unlisted",
                    precision: (0.0, 0.0),
                    limits: MarketLimits::new((0.0, 0.0), (0.0, 0.0), (0.0, 0.0)),
                    info: None,
                });
            });
        }
        Ok(markets)
    }
}

impl ExchangeTrait for Bitmex {
    fn fetch_ohlcv(&self, symbol: &str, timeframe: CandleTime, since: u64, limit: u64) -> FetchOhlcvResult {
        let market = self.exchange.get_market_by_symbol(symbol).unwrap();//TODO not unwrap
        let bin_size = format!("binSize={}", Self::time_frame(timeframe));
        let symbol = format!("symbol={}", market.id);
        let count = format!("count={}", limit);
        let date = format!("startTime={}", NaiveDateTime::from_timestamp((since / 1000) as i64, 0).format("%m-%d-%y%%20%H:%M"));
        Box::from(
            get_api!(self.exchange, "public", "trade/bucketed", bin_size.as_str(), symbol.as_str(), count.as_str(), date.as_str())
                .and_then(move |json| Self::parse_ohlcv(json)
            )
        )
    }

    fn fetch_markets(&mut self) -> LoadMarketResult {
        let lock = self.exchange.market.clone();
        Box::from(get_api!(self.exchange, "public", "instrument/activeAndIndices")
            .and_then(move |re| {
                match Self::parse_markets(re) {
                    Ok(result) =>{ 
                        *lock.write().unwrap() = Some(result);
                        ok(lock)
                    },
                    Err(result) => {
                        println!("Can't parse {}", result);
                        err(result)
                    },
                }
            }))
    }
}


#[cfg(test)]
mod tests {
    use hyper::rt;
    use super::Bitmex;
    use futures::future;
    use futures::Future;
    //use futures::future::{ok, err};
    use crate::prelude::*;
    use crate::base::exchange::ExchangeTrait;
    #[test]
    fn test_plateform() {
        rt::run(future::lazy(move||{
            Bitmex::new().and_then(|exchange| {
                exchange.fetch_ohlcv("XBT/USD", CandleTime::_1M, 1240020225, 100)
                    .map(|ohlcv| {
                        println!("{:?}", ohlcv);
                    })
            })
            .map(|_|{})
            .map_err(|_|{})
        }));
    }
}