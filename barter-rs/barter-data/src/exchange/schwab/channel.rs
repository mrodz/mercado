use crate::{Identifier, exchange::schwab::Schwab, subscription::{Subscription, trade::PublicTrades}};

#[derive(Debug)]
pub struct SchwabChannel(pub &'static str);

impl SchwabChannel {
    pub const TRADES: Self = Self("quotes/stream");
}

impl<Instrument> Identifier<SchwabChannel> for Subscription<Schwab, Instrument, PublicTrades> {
    fn id(&self) -> SchwabChannel {
        SchwabChannel::TRADES
    }
}

impl AsRef<str> for SchwabChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}