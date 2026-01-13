use barter::{
    EngineEvent,
    engine::{
        Engine, Processor,
        clock::LiveClock,
        state::{
            EngineState,
            global::DefaultGlobalData,
            instrument::{
                data::{DefaultInstrumentMarketData, InstrumentDataState},
                filter::InstrumentFilter,
            },
            order::in_flight_recorder::InFlightRequestRecorder,
            position::PositionManager,
            trading::TradingState,
        },
    },
    logging::init_logging,
    risk::DefaultRiskManager,
    statistic::{summary::instrument::TearSheetGenerator, time::Daily},
    strategy::{
        DefaultStrategy,
        algo::AlgoStrategy,
        close_positions::{ClosePositionsStrategy, build_ioc_market_order_to_close_position},
        on_disconnect::OnDisconnectStrategy,
        on_trading_disabled::OnTradingDisabled,
    },
    system::{
        builder::{AuditMode, EngineFeedMode, SystemArgs, SystemBuilder},
        config::SystemConfig,
    },
};
use barter_data::{
    event::{DataKind, MarketEvent},
    exchange::{
        binance::{futures::BinanceFuturesUsd, spot::BinanceSpot},
        coinbase::Coinbase,
        gateio::spot::GateioSpot,
        okx::Okx,
        schwab::Schwab,
    },
    streams::{
        Streams, builder::dynamic::indexed::init_indexed_multi_exchange_market_stream,
        reconnect::stream::ReconnectingStream,
    },
    subscription::{SubKind, trade::PublicTrades},
};
use barter_execution::{
    AccountEvent, AccountEventKind,
    order::{
        id::{ClientOrderId, StrategyId},
        request::{OrderRequestCancel, OrderRequestOpen},
    },
};
use barter_instrument::{
    asset::AssetIndex,
    exchange::{ExchangeId, ExchangeIndex},
    index::IndexedInstruments,
    instrument::{InstrumentIndex, market_data::kind::MarketDataInstrumentKind},
};
use futures::StreamExt;
use tracing::warn;
#[tokio::main]
async fn main() {
    #[cfg(not)]
    {
        let streams = Streams::<PublicTrades>::builder()
            .subscribe([
                // (BinanceFuturesUsd::default(), "btc", "usdt", MarketDataInstrumentKind::Perpetual, PublicTrades)
                (
                    Schwab::default(),
                    "btc",
                    "usdt",
                    MarketDataInstrumentKind::Spot,
                    PublicTrades,
                ),
            ])
            .init()
            .await
            .unwrap();

        let mut joined_stream = streams
            .select_all()
            .with_error_handler(|error| warn!(?error, "MarketStream generated error"));

        while let Some(event) = joined_stream.next().await {
            println!("{event:?}");
        }
    }
}
