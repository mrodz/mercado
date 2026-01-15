use barter_instrument::exchange::ExchangeId;
use chrono::Utc;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    books::Level, event::{MarketEvent, MarketIter}, exchange::schwab::message::SchwabPayload, subscription::book::OrderBookL1
};


#[derive(Clone, Copy, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchwabLevel {
    /// price
    #[serde(rename = "p")]
    #[serde(with = "rust_decimal::serde::str")]
    pub price: Decimal,
    /// size/qty
    #[serde(rename = "q")]
    #[serde(with = "rust_decimal::serde::str")]
    pub size: Decimal,
}

impl From<SchwabLevel> for Level {
    fn from(level: SchwabLevel) -> Self {
        Self {
            price: level.price,
            amount: level.size,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SchwabOrderBookInner {
    #[serde(rename = "b")]
    pub bids: Vec<SchwabLevel>,

    #[serde(rename = "a")]
    pub asks: Vec<SchwabLevel>,

    #[serde(rename = "s")]
    pub symbol: String,
}

// pub type BybitOrderBookMessage = BybitPayload<BybitOrderBookInner>;
pub type SchwabOrderBookMessage = SchwabPayload<SchwabOrderBookInner>;

impl<InstrumentKey> From<(ExchangeId, InstrumentKey, SchwabOrderBookMessage)>
    for MarketIter<InstrumentKey, OrderBookL1>
where
    InstrumentKey: Clone,
{
    fn from(
        (exchange, instrument, book): (ExchangeId, InstrumentKey, SchwabOrderBookMessage),
    ) -> Self {
        let best_ask = book.data.asks.first().copied().map(Level::from);
        let best_bid = book.data.bids.first().copied().map(Level::from);

        Self(vec![Ok(MarketEvent {
            time_exchange: book.time,
            time_received: Utc::now(),
            exchange,
            instrument,
            kind: OrderBookL1 {
                last_update_time: book.time,
                best_bid,
                best_ask,
            },
        })])
    }
}
