use crate::connection::Connection;
use crate::data::*;
use crate::error::Error;

use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Stream {
    conn: Connection,
    stream_session_id: String,
}

impl Stream {
    pub async fn connect(url: &str, stream_session_id: String) -> Result<Stream, Error> {
        Ok(Stream { conn: Connection::connect(url).await?, stream_session_id })
    }

    pub async fn skip_delay(&self) {
        self.conn.skip_delay().await;
    }

    pub async fn get_balance(&self) -> Result<(), Error> {
        self.conn
            .request(&format!(
                "{{\"command\":\"getBalance\",\"streamSessionId\":\"{}\"}}",
                self.stream_session_id
            ))
            .await
    }

    pub async fn stop_balance(&self) -> Result<(), Error> {
        self.conn.request("{\"command\":\"stopBalance\"}").await
    }

    pub async fn get_candles(&self, symbol: &str) -> Result<(), Error> {
        self.conn
            .request(&format!(
                "{{\"command\":\"getCandles\",\"streamSessionId\":\"{}\",\"symbol\":\"{}\"}}",
                self.stream_session_id, symbol
            ))
            .await
    }

    pub async fn stop_candles(&self, symbol: &str) -> Result<(), Error> {
        self.conn
            .request(&format!("{{\"command\":\"stopCandles\",\"symbol\":\"{}\"}}", symbol))
            .await
    }

    pub async fn get_keep_alive(&self) -> Result<(), Error> {
        self.conn
            .request(&format!(
                "{{\"command\":\"getKeepAlive\",\"streamSessionId\":\"{}\"}}",
                self.stream_session_id
            ))
            .await
    }

    pub async fn stop_keep_alive(&self) -> Result<(), Error> {
        self.conn.request("{\"command\":\"stopKeepAlive\"}").await
    }

    pub async fn get_news(&self) -> Result<(), Error> {
        self.conn
            .request(&format!(
                "{{\"command\":\"getNews\",\"streamSessionId\":\"{}\"}}",
                self.stream_session_id
            ))
            .await
    }

    pub async fn stop_news(&self) -> Result<(), Error> {
        self.conn.request("{\"command\":\"stopNews\"}").await
    }

    pub async fn get_profits(&self) -> Result<(), Error> {
        self.conn
            .request(&format!(
                "{{\"command\":\"getProfits\",\"streamSessionId\":\"{}\"}}",
                self.stream_session_id
            ))
            .await
    }

    pub async fn stop_profits(&self) -> Result<(), Error> {
        self.conn.request("{\"command\":\"stopProfits\"}").await
    }

    pub async fn get_tick_prices(&self, symbol: &str, min_arrival_time: i64, max_level: i64) -> Result<(), Error> {
        self.conn
            .request(&format!(
                "{{\"command\":\"getTickPrices\",\"streamSessionId\":\"{}\",\"symbol\":\"{}\",\"minArrivalTime\":{},\"maxLevel\":{}}}",
                self.stream_session_id, symbol, min_arrival_time, max_level
            ))
            .await
    }

    pub async fn stop_tick_prices(&self, symbol: &str) -> Result<(), Error> {
        self.conn
            .request(&format!("{{\"command\":\"stopTickPrices\",\"symbol\":\"{}\"}}", symbol))
            .await
    }

    pub async fn get_trades(&self) -> Result<(), Error> {
        self.conn
            .request(&format!(
                "{{\"command\":\"getTrades\",\"streamSessionId\":\"{}\"}}",
                self.stream_session_id
            ))
            .await
    }

    pub async fn stop_trades(&self) -> Result<(), Error> {
        self.conn.request("{\"command\":\"stopTrades\"}").await
    }

    pub async fn get_trade_status(&self) -> Result<(), Error> {
        self.conn
            .request(&format!(
                "{{\"command\":\"getTradeStatus\",\"streamSessionId\":\"{}\"}}",
                self.stream_session_id
            ))
            .await
    }

    pub async fn stop_trade_status(&self) -> Result<(), Error> {
        self.conn.request("{\"command\":\"stopTradeStatus\"}").await
    }

    pub async fn ping(&self) -> Result<(), Error> {
        self.conn
            .request(&format!(
                "{{\"command\":\"ping\",\"streamSessionId\":\"{}\"}}",
                self.stream_session_id
            ))
            .await
    }

    pub async fn listen(&self) -> Result<Record, Error> {
        let record = self.conn.receive().await?;

        #[derive(Deserialize)]
        struct Command {
            command: String,
        }
        #[derive(Deserialize)]
        struct Data<T> {
            data: T,
        }

        fn from<T: DeserializeOwned>(record: &str) -> Result<T, Error> {
            let d = serde_json::from_str::<Data<T>>(record)?;
            Ok(d.data)
        }

        let c = serde_json::from_str::<Command>(&record)?;
        match c.command.as_str() {
            "balance" => Ok(Record::Balance(from::<Balance>(&record)?)),
            "candle" => Ok(Record::Candle(from::<Candle>(&record)?)),
            "keepAlive" => Ok(Record::KeepAlive(from::<KeepAlive>(&record)?)),
            "news" => Ok(Record::News(from::<News>(&record)?)),
            "profit" => Ok(Record::Profit(from::<Profit>(&record)?)),
            "tickPrices" => Ok(Record::Tick(from::<Tick>(&record)?)),
            "trade" => Ok(Record::Trade(from::<Trade>(&record)?)),
            "tradeStatus" => Ok(Record::TradeStatus(from::<TradeStatus>(&record)?)),
            _ => Err(Error::UnknownRecord { record }),
        }
    }
}
