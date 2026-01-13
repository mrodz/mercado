use barter_instrument::exchange::ExchangeId;
use barter_integration::{error::SocketError, protocol::websocket::WsMessage};
use barter_macro::{DeExchange, SerExchange};
use derive_more::Display;
use itertools::Itertools;
use url::Url;

use crate::{exchange::{Connector, PingInterval, schwab::{channel::SchwabChannel, market::SchwabMarket, subscription::SchwabSubResponse}}, subscriber::{WebSocketSubscriber, validator::WebSocketSubValidator}};

pub mod channel;

pub mod market;

pub mod subscription;

pub mod trade;

/// [`Schwab`] server base url.
///
/// See docs: <https://www.okx.com/docs-v5/en/#overview-api-resources-and-support>
pub const BASE_URL_SCHWAB: &str = "https://127.0.0.1:8000/u/quotes/stream";

// /// [`Okx`] server [`PingInterval`] duration.
// ///
// /// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-connect>
// pub const PING_INTERVAL_OKX: Duration = Duration::from_secs(29);

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    Default,
    Display,
    DeExchange,
    SerExchange,
)]
pub struct Schwab;

impl Connector for Schwab {
    const ID: ExchangeId = ExchangeId::Schwab;
    type Channel = SchwabChannel;
    type Market = SchwabMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = SchwabSubResponse;

    fn url() -> Result<url::Url, barter_integration::error::SocketError> {
        Url::parse(BASE_URL_SCHWAB).map_err(SocketError::UrlParse)
    }

    fn ping_interval() -> Option<PingInterval> {
        None
    }

    fn requests(exchange_subs: Vec<super::subscription::ExchangeSub<Self::Channel, Self::Market>>) -> Vec<barter_integration::protocol::websocket::WsMessage> {
        let symbols = exchange_subs.iter().map(|exchange| exchange.market.as_ref()).collect_vec();

        vec![WsMessage::text(serde_json::json!({
            "type": "subscribe",
            "symbols": symbols,
        }).to_string())]
    }
}