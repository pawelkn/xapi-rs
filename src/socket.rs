use crate::connection::Connection;
use crate::data::*;
use crate::enums::*;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Socket {
    conn: Connection,
    safe: bool,
}

impl Socket {
    pub async fn connect(url: &str, safe: bool) -> Result<Socket, Error> {
        Ok(Socket { conn: Connection::connect(url).await?, safe })
    }

    pub async fn skip_delay(&self) {
        self.conn.skip_delay().await;
    }

    pub async fn login(&self, account_id: &str, password: &str) -> Result<LoginResponse, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"login\",\"arguments\":{{\"userId\":\"{}\",\"password\":\"{}\"}}}}",
                account_id, password
            ))
            .await
    }

    pub async fn logout(&self) -> Result<LogoutResponse, Error> {
        self.conn.transaction("{\"command\":\"logout\"}").await
    }

    pub async fn get_all_symbols(&self) -> Result<Response<Vec<Symbol>>, Error> {
        self.conn.transaction("{\"command\":\"getAllSymbols\"}").await
    }

    pub async fn get_calendar(&self) -> Result<Response<Vec<Calendar>>, Error> {
        self.conn.transaction("{\"command\":\"getCalendar\"}").await
    }

    pub async fn get_chart_last_request(
        &self,
        symbol: &str,
        start: i64,
        period: Period,
    ) -> Result<Response<ChartRateInfo>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getChartLastRequest\",\"arguments\":{{\"info\":{{\"period\":{},\"start\":{},\"symbol\":\"{}\"}}}}}}",
                period as i64, start, symbol
            ))
            .await
    }

    pub async fn get_chart_range_request(
        &self,
        symbol: &str,
        start: i64,
        end: i64,
        period: Period,
        ticks: i64,
    ) -> Result<Response<ChartRateInfo>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getChartRangeRequest\",\"arguments\":{{\"info\":{{\"end\":{},\"period\":{},\"start\":{},\"symbol\":\"{}\",\"ticks\":{}}}}}}}",
                end, period as i64, start, symbol, ticks
            ))
            .await
    }

    pub async fn get_commission_def(&self, symbol: &str, volume: f64) -> Result<Response<CommissionDef>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getCommissionDef\",\"arguments\":{{\"symbol\":\"{}\",\"volume\":{:.6}}}}}",
                symbol, volume
            ))
            .await
    }

    pub async fn get_current_user_data(&self) -> Result<Response<CurrentUserData>, Error> {
        self.conn.transaction("{\"command\":\"getCurrentUserData\"}").await
    }

    pub async fn get_ibs_history(&self, start: i64, end: i64) -> Result<Response<Vec<IBData>>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getIbsHistory\",\"arguments\":{{\"end\":{},\"start\":{}}}}}",
                end, start
            ))
            .await
    }

    pub async fn get_margin_level(&self) -> Result<Response<MarginLevel>, Error> {
        self.conn.transaction("{\"command\":\"getMarginLevel\"}").await
    }

    pub async fn get_margin_trade(&self, symbol: &str, volume: f64) -> Result<Response<MarginTrade>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getMarginTrade\",\"arguments\":{{\"symbol\":\"{}\",\"volume\":{:.6}}}}}",
                symbol, volume
            ))
            .await
    }

    pub async fn get_news(&self, start: i64, end: i64) -> Result<Response<Vec<News>>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getNews\",\"arguments\":{{\"end\":{},\"start\":{}}}}}",
                end, start
            ))
            .await
    }

    pub async fn get_profit_calculation(
        &self,
        symbol: &str,
        cmd: TradeCmd,
        open_price: f64,
        close_price: f64,
        volume: f64,
    ) -> Result<Response<ProfitCalculation>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getProfitCalculation\",\"arguments\":{{\"closePrice\":{:.6},\"cmd\":{},\"openPrice\":{:.6},\"symbol\":\"{}\",\"volume\":{:.6}}}}}",
                close_price, cmd as i64, open_price, symbol, volume
            ))
            .await
    }

    pub async fn get_server_time(&self) -> Result<Response<ServerTime>, Error> {
        self.conn.transaction("{\"command\":\"getServerTime\"}").await
    }

    pub async fn get_step_rules(&self) -> Result<Response<Vec<StepRule>>, Error> {
        self.conn.transaction("{\"command\":\"getStepRules\"}").await
    }

    pub async fn get_symbol(&self, symbol: &str) -> Result<Response<Symbol>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getSymbol\",\"arguments\":{{\"symbol\":\"{}\"}}}}",
                symbol
            ))
            .await
    }

    pub async fn get_tick_prices(
        &self,
        symbols: Vec<&str>,
        timestamp: i64,
        level: i64,
    ) -> Result<Response<TickPrices>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getTickPrices\",\"arguments\":{{\"level\":{},\"symbols\":[{}],\"timestamp\":{}}}}}",
                level,
                symbols
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<String>>()
                    .join(","),
                timestamp
            ))
            .await
    }

    pub async fn get_trade_records(&self, orders: Vec<i64>) -> Result<Response<Vec<Trade>>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getTradeRecords\",\"arguments\":{{\"orders\":[{}]}}}}",
                orders.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
            ))
            .await
    }

    pub async fn get_trades(&self, opened_only: bool) -> Result<Response<Vec<Trade>>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getTrades\",\"arguments\":{{\"openedOnly\":{}}}}}",
                opened_only
            ))
            .await
    }

    pub async fn get_trades_history(&self, start: i64, end: i64) -> Result<Response<Vec<Trade>>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getTradesHistory\",\"arguments\":{{\"end\":{},\"start\":{}}}}}",
                end, start
            ))
            .await
    }

    pub async fn get_trading_hours(&self, symbols: Vec<&str>) -> Result<Response<Vec<TradingHours>>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"getTradingHours\",\"arguments\":{{\"symbols\":[{}]}}}}",
                symbols
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<String>>()
                    .join(",")
            ))
            .await
    }

    pub async fn get_version(&self) -> Result<Response<Version>, Error> {
        self.conn.transaction("{\"command\":\"getVersion\"}").await
    }

    pub async fn ping(&self) -> Result<PingResponse, Error> {
        self.conn.transaction("{\"command\":\"ping\"}").await
    }

    pub async fn trade_transaction(&self, transaction: Transaction) -> Result<Response<Order>, Error> {
        if self.safe == true {
            return Err(Error::TradingIsDisabled);
        }

        self.conn
            .transaction(&format!(
                "{{\"command\":\"tradeTransaction\",\"arguments\":{{\"tradeTransInfo\":{{\"cmd\":{},\"customComment\":\"{}\",\"expiration\":{},\"offset\":{},\"order\":{},\"price\":{:.6},\"sl\":{:.6},\"symbol\":\"{}\",\"tp\":{:.6},\"type\":{},\"volume\":{:.6}}}}}}}",
                transaction.cmd as i64, transaction.custom_comment, transaction.expiration, transaction.offset, transaction.order, transaction.price, transaction.sl, transaction.symbol, transaction.tp, transaction.type_ as i64, transaction.volume
            ))
            .await
    }

    pub async fn trade_transaction_status(&self, order: i64) -> Result<Response<TradeStatus>, Error> {
        self.conn
            .transaction(&format!(
                "{{\"command\":\"tradeTransactionStatus\",\"arguments\":{{\"order\":{}}}}}",
                order
            ))
            .await
    }
}
