use barter_instrument::{
    asset::name::AssetNameInternal,
    exchange::ExchangeId,
    instrument::market_data::{MarketDataInstrument, kind::MarketDataInstrumentKind},
};
use barter_integration::{
    error::SocketError,
    protocol::websocket::{WebSocketSerdeParser, WsMessage},
};
use barter_macro::{DeExchange, SerExchange};
use derive_more::Display;
use futures::Stream;
use itertools::Itertools;
use url::Url;

use crate::{
    ExchangeWsStream,
    error::DataError,
    event::DataKind,
    exchange::{
        Connector, PingInterval, StreamSelector,
        schwab::{
            channel::SchwabChannel, l1::SchwabOrderBookMessage, market::SchwabMarket,
            subscription::SchwabSubResponse,
        },
    },
    instrument::InstrumentData,
    streams::{
        builder::dynamic::DynamicStreams,
        consumer::{MarketStreamEvent, MarketStreamResult},
        reconnect::stream::ReconnectingStream,
    },
    subscriber::{WebSocketSubscriber, validator::WebSocketSubValidator},
    subscription::{SubKind, Subscription, book::OrderBooksL1},
    transformer::stateless::StatelessTransformer,
};

pub mod channel;

pub mod l1;

pub mod market;

pub mod message;

pub mod subscription;

pub mod trade;

/// [`Schwab`] server base url.
///
/// See docs: <https://www.okx.com/docs-v5/en/#overview-api-resources-and-support>
pub const BASE_URL_SCHWAB: &str = "wss://127.0.0.1:8000/u/quotes/stream";

pub type SchwabWsStream<Transformer> = ExchangeWsStream<WebSocketSerdeParser, Transformer>;

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

impl Schwab {
    pub async fn stream<S>(
        base: S,
        quote: S,
    ) -> Result<
        impl Stream<Item = MarketStreamEvent<MarketDataInstrument, DataKind>> + use<S>,
        DataError,
    >
    where
        S: Into<AssetNameInternal>,
    {
        let subscription = Self::subscription(base, quote);

        /*

        the trait bound `Event<ExchangeId, Result<MarketEvent<...>, ...>>: From<...>` is not satisfied
        the trait `From<reconnect::Event<ExchangeId, Result<MarketEvent<MarketDataInstrument, PublicTrade>, DataError>>>` is not implemented for `Event<ExchangeId, Result<MarketEvent<InstrumentIndex>, DataError>>`
        the following other types implement trait `From<T>`:

         */

        let stream = DynamicStreams::init([[subscription]])
            .await?
            .select_all::<MarketStreamResult<MarketDataInstrument, DataKind>>()
            .with_error_handler(|error| tracing::warn!(?error, "MarketStream generated error"));

        Ok(stream)
    }

    pub fn subscription<S>(base: S, quote: S) -> Subscription
    where
        S: Into<AssetNameInternal>,
    {
        /*
         * pub struct MarketDataInstrument {
         *   pub base: AssetNameInternal,
         *   pub quote: AssetNameInternal,
         *   #[serde(rename = "instrument_kind")]
         *   pub kind: MarketDataInstrumentKind,
         * }
         */
        let instrument = MarketDataInstrument::new(base, quote, MarketDataInstrumentKind::Spot);

        Subscription::new(Schwab::ID, instrument, SubKind::OrderBooksL1)
    }
}

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

    fn requests(
        exchange_subs: Vec<super::subscription::ExchangeSub<Self::Channel, Self::Market>>,
    ) -> Vec<barter_integration::protocol::websocket::WsMessage> {
        let symbols = exchange_subs
            .iter()
            .map(|exchange| exchange.market.as_ref())
            .collect_vec();

        vec![WsMessage::text(
            serde_json::json!({
                "type": "subscribe",
                "symbols": symbols,
            })
            .to_string(),
        )]
    }
}

impl<Instrument> StreamSelector<Instrument, OrderBooksL1> for Schwab
where
    Instrument: InstrumentData,
{
    type SnapFetcher = crate::NoInitialSnapshots;
    type Stream = SchwabWsStream<
        StatelessTransformer<Self, Instrument::Key, OrderBooksL1, SchwabOrderBookMessage>,
    >;
}
