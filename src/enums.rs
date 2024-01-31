use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(from = "i64")]
pub enum TradeCmd {
    /// Buy
    Buy = 0,
    /// Sell
    Sell = 1,
    /// Buy limit
    BuyLimit = 2,
    /// Sell limit
    SellLimit = 3,
    /// Suy stop
    BuyStop = 4,
    /// Sell stop
    SellStop = 5,
    /// Read only. Used in getTradesHistory for manager's deposit/withdrawal operations (profit>0 for deposit, profit<0 for withdrawal).
    Balance = 6,
    /// Read only
    Credit = 7,
    #[default]
    Invalid = -1,
}

impl From<i64> for TradeCmd {
    fn from(value: i64) -> Self {
        match value {
            0 => TradeCmd::Buy,
            1 => TradeCmd::Sell,
            2 => TradeCmd::BuyLimit,
            3 => TradeCmd::SellLimit,
            4 => TradeCmd::BuyStop,
            5 => TradeCmd::SellStop,
            6 => TradeCmd::Balance,
            7 => TradeCmd::Credit,
            _ => TradeCmd::Invalid,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(from = "i64")]
pub enum TradeType {
    /// Order open, used for opening orders
    Open = 0,
    /// Order pending, only used in the streaming getTrades command
    Pending = 1,
    /// Order close
    Close = 2,
    /// Order modify, only used in the tradeTransaction command
    Modify = 3,
    /// Order delete, only used in the tradeTransaction command
    Delete = 4,
    #[default]
    Invalid = -1,
}

impl From<i64> for TradeType {
    fn from(value: i64) -> Self {
        match value {
            0 => TradeType::Open,
            1 => TradeType::Pending,
            2 => TradeType::Close,
            3 => TradeType::Modify,
            4 => TradeType::Delete,
            _ => TradeType::Invalid,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(from = "i64")]
pub enum RequestStatus {
    /// Error occurred while executing the transaction
    Error = 0,
    /// The transaction is pending
    Pending = 1,
    /// The transaction has been executed successfully
    Accepted = 3,
    /// The transaction has been rejected
    Rejected = 4,
    #[default]
    Invalid = -1,
}

impl From<i64> for RequestStatus {
    fn from(value: i64) -> Self {
        match value {
            0 => RequestStatus::Error,
            1 => RequestStatus::Pending,
            3 => RequestStatus::Accepted,
            4 => RequestStatus::Rejected,
            _ => RequestStatus::Invalid,
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(from = "i64")]
pub enum Period {
    /// 1 minute
    M1 = 1,
    /// 5 minutes
    M5 = 5,
    /// 15 minutes
    M15 = 15,
    /// 30 minutes
    M30 = 30,
    /// 60 minutes (1 hour)
    H1 = 60,
    /// 240 minutes (4 hours)
    H4 = 240,
    /// 1440 minutes (1 day)
    D1 = 1440,
    /// 10080 minutes (1 week)
    W1 = 10080,
    /// 43200 minutes (30 days)
    MN1 = 43200,
    #[default]
    Invalid = -1,
}

impl From<i64> for Period {
    fn from(value: i64) -> Self {
        match value {
            1 => Period::M1,
            5 => Period::M5,
            15 => Period::M15,
            30 => Period::M30,
            60 => Period::H1,
            240 => Period::H4,
            1440 => Period::D1,
            10080 => Period::W1,
            43200 => Period::MN1,
            _ => Period::Invalid,
        }
    }
}
