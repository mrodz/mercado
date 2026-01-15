use crate::{Identifier, exchange::schwab::Schwab, subscription::{Subscription, book::OrderBooksL1}};

#[derive(Debug)]
pub struct SchwabChannel(pub &'static str);

impl SchwabChannel {
    pub const TRADES: Self = Self("quotes/stream");
}

impl<Instrument> Identifier<SchwabChannel> for Subscription<Schwab, Instrument, OrderBooksL1> {
    fn id(&self) -> SchwabChannel {
        SchwabChannel::TRADES
    }
}

impl AsRef<str> for SchwabChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}