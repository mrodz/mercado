use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_error_de() {
        const JSON: &str = "{\"id\":\"b1b4d335-35e8-4bce-81af-c22e60122086\",\"status\":\"400\",\"title\":\"Bad Request\",\"detail\":\"Search combination should have min of 1.\",\"source\":{\"pointer\":[\"/data/attributes/symbols\",\"/data/attributes/cusips\",\"/data/attributes/ssids\"]}}";
        let result = serde_json::from_str::<ApiError>(JSON);
        dbg!(&result);
        assert!(result.is_ok())
    }

    #[test]
    fn full_error_de() {
        const JSON: &str = "{\"errors\":[{\"id\":\"a208a739-7d58-469a-b6c4-ec1395edb48f\",\"status\":\"400\",\"title\":\"Bad Request\",\"detail\":\"Search combination should have min of 1.\",\"source\":{\"pointer\":[\"/data/attributes/symbols\",\"/data/attributes/cusips\",\"/data/attributes/ssids\"]}}]}";
        let result = serde_json::from_str::<QuoteResponse>(JSON);
        dbg!(&result);
        assert!(result.is_ok())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchwabAccount {
    #[serde(rename(serialize = "accountNumber", deserialize = "accountNumber"))]
    account_number: String,
    #[serde(rename(serialize = "primaryAccount", deserialize = "primaryAccount"))]
    primary_account: bool,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    account_type: String,
    #[serde(rename(serialize = "nickName", deserialize = "nickName"))]
    nick_name: String,
    #[serde(rename(serialize = "displayAcctId", deserialize = "displayAcctId"))]
    display_id: String,
    #[serde(rename(serialize = "autoPositionEffect", deserialize = "autoPositionEffect"))]
    auto_position_effect: bool,
    #[serde(rename(serialize = "accountColor", deserialize = "accountColor"))]
    account_color: String,
    #[serde(rename(serialize = "lotSelectionMethod", deserialize = "lotSelectionMethod"))]
    lot_selection_method: String,
    #[serde(rename(serialize = "hasFuturesAccount", deserialize = "hasFuturesAccount"))]
    has_futures_account: bool,
    #[serde(rename(serialize = "hasForexAccount", deserialize = "hasForexAccount"))]
    has_forex_account: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteResponse {
    #[serde(flatten)]
    pub quotes: HashMap<String, QuoteResponseObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum QuoteResponseObject {
    Equity(EquityResponse),
    Option(OptionResponse),
    Forex(ForexResponse),
    Future(FutureResponse),
    FutureOption(FutureOptionResponse),
    Index(IndexResponse),
    MutualFund(MutualFundResponse),
    Error(QuoteError),
    ApiError(ApiErrorResponse),
}

// Asset Types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssetMainType {
    Bond,
    Equity,
    Forex,
    Future,
    FutureOption,
    Index,
    MutualFund,
    Option,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EquityAssetSubType {
    Coe,
    Prf,
    Adr,
    Gdr,
    Cef,
    Etf,
    Etn,
    Uit,
    War,
    Rgt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MutualFundAssetSubType {
    Oef,
    Cef,
    Mmf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuoteType {
    Nbbo,
    Nfl,
}

// Equity Response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquityResponse {
    pub asset_main_type: AssetMainType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_sub_type: Option<EquityAssetSubType>,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_type: Option<QuoteType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended: Option<ExtendedMarket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fundamental: Option<Fundamental>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<QuoteEquity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceEquity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular: Option<RegularMarket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedMarket {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fundamental {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg10_days_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg1_year_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declaration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub div_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub div_ex_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub div_freq: Option<DivFreq>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub div_pay_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub div_pay_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub div_yield: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_leverage_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fund_strategy: Option<FundStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_div_ex_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_div_pay_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pe_ratio: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DivFreq {
    #[serde(rename = "1")]
    Annually,
    #[serde(rename = "2")]
    SemiAnnually,
    #[serde(rename = "3")]
    Triannually,
    #[serde(rename = "4")]
    Quarterly,
    #[serde(rename = "6")]
    Bimonthly,
    #[serde(rename = "11")]
    ElevenTimes,
    #[serde(rename = "12")]
    Monthly,
    #[serde(rename = "0")]
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FundStrategy {
    A, // Active
    L, // Leveraged
    P, // Passive
    Q, // Quantitative
    S, // Short
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteEquity {
    #[serde(rename = "52WeekHigh", skip_serializing_if = "Option::is_none")]
    pub week_52_high: Option<f64>,
    #[serde(rename = "52WeekLow", skip_serializing_if = "Option::is_none")]
    pub week_52_low: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_mic_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_mic_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_mic_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_percent_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_percent_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volatility: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceEquity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsi_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub htb_quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub htb_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hard_to_borrow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_shortable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otc_market_tier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegularMarket {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_market_last_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_market_last_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_market_net_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_market_percent_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_market_trade_time: Option<i64>,
}

// Option Response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionResponse {
    pub asset_main_type: AssetMainType,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<QuoteOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteOption {
    #[serde(rename = "52WeekHigh", skip_serializing_if = "Option::is_none")]
    pub week_52_high: Option<f64>,
    #[serde(rename = "52WeekLow", skip_serializing_if = "Option::is_none")]
    pub week_52_low: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ind_ask_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ind_bid_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ind_quote_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implied_yield: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_percent_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money_intrinsic_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_percent_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rho: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theoretical_option_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vega: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volatility: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractType {
    P, // Put
    C, // Call
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExerciseType {
    A, // American
    E, // European
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpirationType {
    M, // End of Month
    Q, // Quarterly
    W, // Weekly
    S, // Standard (3rd Friday)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettlementType {
    A, // AM
    P, // PM
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<ContractType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_to_expiration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverables: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exercise_type: Option<ExerciseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_day: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_type: Option<ExpirationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_penny_pilot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_trading_day: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_type: Option<SettlementType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
}

// Forex Response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForexResponse {
    pub asset_main_type: AssetMainType,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<QuoteForex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceForex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteForex {
    #[serde(rename = "52WeekHigh", skip_serializing_if = "Option::is_none")]
    pub week_52_high: Option<f64>,
    #[serde(rename = "52WeekLow", skip_serializing_if = "Option::is_none")]
    pub week_52_low: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_percent_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceForex {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tradable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_maker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_hours: Option<String>,
}

// Future Response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureResponse {
    pub asset_main_type: AssetMainType,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<QuoteFuture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceFuture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteFuture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_mic_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_mic_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_percent_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_mic_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quoted_in_session: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceFuture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_active_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_expiration_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_price_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_settlement_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_trading_hours: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}

// Future Option Response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FutureOptionResponse {
    pub asset_main_type: AssetMainType,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<QuoteFutureOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceFutureOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteFutureOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_mic_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_mic_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_mic_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_percent_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlemet_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceFutureOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<ContractType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
}

// Index Response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexResponse {
    pub asset_main_type: AssetMainType,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<QuoteIndex>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceIndex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteIndex {
    #[serde(rename = "52WeekHigh", skip_serializing_if = "Option::is_none")]
    pub week_52_high: Option<f64>,
    #[serde(rename = "52WeekLow", skip_serializing_if = "Option::is_none")]
    pub week_52_low: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_percent_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceIndex {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
}

// Mutual Fund Response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutualFundResponse {
    pub asset_main_type: AssetMainType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_sub_type: Option<MutualFundAssetSubType>,
    pub ssid: i64,
    pub symbol: String,
    pub realtime: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fundamental: Option<Fundamental>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<QuoteMutualFund>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<ReferenceMutualFund>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteMutualFund {
    #[serde(rename = "52WeekHigh", skip_serializing_if = "Option::is_none")]
    pub week_52_high: Option<f64>,
    #[serde(rename = "52WeekLow", skip_serializing_if = "Option::is_none")]
    pub week_52_low: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    #[serde(rename = "nAV", skip_serializing_if = "Option::is_none")]
    pub nav: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_percent_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceMutualFund {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
}

// Error Response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteError {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invalid_cusips: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invalid_ssids: Vec<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invalid_symbols: Vec<String>,
}

// API Error Response (for HTTP error responses)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse(pub Vec<ApiError>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ErrorSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<Vec<String>>,
}
