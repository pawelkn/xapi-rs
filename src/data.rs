use crate::enums::*;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub status: bool,
    pub return_data: T,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub status: bool,
    pub stream_session_id: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogoutResponse {
    pub status: bool,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub status: bool,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub status: bool,
    pub error_code: String,
    pub error_descr: String,
}

#[derive(Debug, Clone)]
pub enum Record {
    Balance(Balance),
    Candle(Candle),
    KeepAlive(KeepAlive),
    News(News),
    Profit(Profit),
    Tick(Tick),
    Trade(Trade),
    TradeStatus(TradeStatus),
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub balance: f64,
    pub credit: f64,
    pub equity: f64,
    pub margin: f64,
    pub margin_free: f64,
    pub margin_level: f64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub close: f64,
    pub ctm: i64,
    pub ctm_string: String,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub quote_id: i64,
    pub symbol: String,
    pub vol: f64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeepAlive {
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct News {
    pub body: String,
    pub key: String,
    pub time: i64,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Profit {
    pub order: i64,
    pub order2: i64,
    pub position: i64,
    pub profit: f64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Tick {
    pub ask: f64,
    pub ask_volume: i64,
    pub bid: f64,
    pub bid_volume: i64,
    pub high: f64,
    pub level: i64,
    pub low: f64,
    pub quote_id: Option<i64>,
    pub spread_raw: f64,
    pub spread_table: f64,
    pub symbol: String,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Trade {
    pub close_price: f64,
    pub close_time: Option<i64>,
    pub close_time_string: Option<String>,
    pub closed: bool,
    pub cmd: TradeCmd,
    pub comment: String,
    pub commission: Option<f64>,
    #[serde(rename = "customComment")]
    pub custom_comment: String,
    pub digits: i64,
    pub expiration: Option<i64>,
    #[serde(rename = "expirationString")]
    pub expiration_string: Option<String>,
    pub margin_rate: f64,
    pub offset: i64,
    pub open_price: f64,
    pub open_time: i64,
    #[serde(rename = "open_timeString")]
    pub open_time_string: Option<String>,
    pub order: i64,
    pub order2: i64,
    pub position: i64,
    pub profit: Option<f64>,
    pub sl: f64,
    pub state: Option<String>,
    pub storage: f64,
    pub symbol: Option<String>,
    pub timestamp: Option<i64>,
    pub tp: f64,
    pub type_: Option<TradeType>,
    pub volume: f64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TradeStatus {
    pub ask: Option<f64>,
    pub bid: Option<f64>,
    pub custom_comment: String,
    pub message: Option<String>,
    pub order: i64,
    pub price: Option<f64>,
    pub request_status: RequestStatus,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub ask: f64,
    pub bid: f64,
    pub category_name: String,
    pub contract_size: i64,
    pub currency: String,
    pub currency_pair: bool,
    pub currency_profit: String,
    pub description: String,
    pub expiration: Option<i64>,
    pub group_name: String,
    pub high: f64,
    pub initial_margin: i64,
    pub instant_max_volume: i64,
    pub leverage: f64,
    pub long_only: bool,
    pub lot_max: f64,
    pub lot_min: f64,
    pub lot_step: f64,
    pub low: f64,
    pub margin_hedged: i64,
    pub margin_hedged_strong: bool,
    pub margin_maintenance: Option<i64>,
    pub margin_mode: i64,
    pub percentage: f64,
    pub pips_precision: Option<i64>,
    pub precision: i64,
    pub profit_mode: i64,
    pub quote_id: i64,
    pub short_selling: bool,
    pub spread_raw: f64,
    pub spread_table: f64,
    pub starting: Option<i64>,
    pub step_rule_id: i64,
    pub stops_level: i64,
    #[serde(rename = "swap_rollover3days")]
    pub swap_rollover3days: i64,
    pub swap_enable: bool,
    pub swap_long: f64,
    pub swap_short: f64,
    pub swap_type: i64,
    pub symbol: String,
    pub tick_size: f64,
    pub tick_value: f64,
    pub time: i64,
    pub time_string: String,
    pub trailing_enabled: bool,
    pub type_: i64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub country: String,
    pub current: String,
    pub forecast: String,
    pub impact: String,
    pub period: String,
    pub previous: String,
    pub time: i64,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChartRateInfo {
    pub digits: i64,
    pub rate_infos: Vec<RateInfo>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RateInfo {
    pub close: f64,
    pub ctm: i64,
    pub ctm_string: String,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub vol: f64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommissionDef {
    pub commission: Option<f64>,
    pub rate_of_exchange: Option<f64>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUserData {
    pub company_unit: i64,
    pub currency: String,
    pub group: String,
    pub ib_account: bool,
    pub leverage: i64,
    pub leverage_multiplier: f64,
    pub spread_type: Option<String>,
    pub trailing_stop: bool,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct IBData {
    pub close_price: Option<f64>,
    pub login: Option<String>,
    pub nominal: Option<f64>,
    pub open_price: Option<f64>,
    pub side: Option<i64>,
    pub surname: Option<String>,
    pub symbol: Option<String>,
    pub timestamp: Option<i64>,
    pub volume: Option<f64>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarginLevel {
    pub balance: f64,
    pub credit: f64,
    pub currency: String,
    pub equity: f64,
    pub margin: f64,
    #[serde(rename = "margin_free")]
    pub margin_free: f64,
    #[serde(rename = "margin_level")]
    pub margin_level: f64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarginTrade {
    pub margin: f64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProfitCalculation {
    pub profit: f64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub time: i64,
    pub time_string: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StepRule {
    pub id: i64,
    pub name: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    pub from_value: f64,
    pub step: f64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TickPrices {
    pub quotations: Vec<Tick>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TradingHours {
    pub quotes: Vec<Quote>,
    pub symbol: String,
    pub trading: Vec<Trading>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub day: i64,
    pub from_t: i64,
    pub to_t: i64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Trading {
    pub day: i64,
    pub from_t: i64,
    pub to_t: i64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub version: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub cmd: TradeCmd,
    pub custom_comment: String,
    pub expiration: i64,
    pub offset: i64,
    pub order: i64,
    pub price: f64,
    pub sl: f64,
    pub symbol: String,
    pub tp: f64,
    pub type_: TradeType,
    pub volume: f64,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order: i64,
}
