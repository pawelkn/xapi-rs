use crate::data::ErrorResponse;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection closed")]
    ConnectionClosed,
    #[error("Ping timeout")]
    PingTimeout,
    #[error("No data received")]
    NoDataReceived,
    #[error("Trading is disabled when safe=true")]
    TradingIsDisabled,
    #[error("Error received: {response:?}")]
    ErrorResponse { response: ErrorResponse },
    #[error("Unknown record: {record:?}")]
    UnknownRecord { record: String },
    #[error("JsonParseError: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Websocket error: {0}")]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
}
