use barter_instrument::{Keyed, asset::name::AssetNameInternal, instrument::market_data::MarketDataInstrument};
use smol_str::{SmolStr, StrExt, format_smolstr};

use crate::{Identifier, exchange::schwab::Schwab, instrument::MarketInstrumentData, subscription::Subscription};

#[derive(Debug)]
pub struct SchwabMarket(pub SmolStr);

impl<Kind> Identifier<SchwabMarket> for Subscription<Schwab, MarketDataInstrument, Kind> {
    fn id(&self) -> SchwabMarket {
        schwab_market(&self.instrument.base, &self.instrument.quote)
    }
}

impl<InstrumentKey, Kind> Identifier<SchwabMarket>
    for Subscription<Schwab, Keyed<InstrumentKey, MarketDataInstrument>, Kind>
{
    fn id(&self) -> SchwabMarket {
        schwab_market(&self.instrument.value.base, &self.instrument.value.quote)
    }
}

impl<InstrumentKey, Kind> Identifier<SchwabMarket>
    for Subscription<Schwab, MarketInstrumentData<InstrumentKey>, Kind>
{
    fn id(&self) -> SchwabMarket {
        SchwabMarket(self.instrument.name_exchange.name().clone())
    }
}

impl AsRef<str> for SchwabMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn schwab_market(base: &AssetNameInternal, quote: &AssetNameInternal) -> SchwabMarket {
    SchwabMarket(format_smolstr!("{base}-{quote}").to_uppercase_smolstr())
}