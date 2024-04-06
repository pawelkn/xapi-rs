use crate::data::ErrorResponse;
use crate::error::Error;

use futures::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use log::*;
use serde::de::DeserializeOwned;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Instant;

use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{sleep, timeout, Duration};

use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

const PING_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub struct Connection {
    write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    read: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    last_request_time: Arc<Mutex<Option<Instant>>>,
    transaction: Arc<Mutex<()>>,
}

impl Connection {
    pub async fn connect(url: &str) -> Result<Connection, Error> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
        let (write, read) = ws_stream.split();

        let conn = Connection {
            write: Arc::new(Mutex::new(write)),
            read: Arc::new(Mutex::new(read)),
            last_request_time: Arc::new(Mutex::new(None)),
            transaction: Arc::new(Mutex::new(())),
        };

        conn.spawn_pinging_task();

        Ok(conn)
    }

    pub async fn skip_delay(&self) {
        let mut last_request_time = self.last_request_time.lock().await;
        *last_request_time = None;
    }

    pub async fn transaction<T: DeserializeOwned>(&self, command: &str) -> Result<T, Error> {
        let _transaction = self.transaction.lock().await;
        self.request(command).await?;
        let response = self.receive().await?;

        if let Ok(response) = serde_json::from_str::<ErrorResponse>(&response) {
            return Err(Error::ErrorResponse { response });
        }

        let response = serde_json::from_str::<T>(&response)?;
        Ok(response)
    }

    pub async fn request(&self, command: &str) -> Result<(), Error> {
        let mut last_request_time = self.last_request_time.lock().await;
        if let Some(last_request_time) = last_request_time.deref() {
            let elapsed = last_request_time.elapsed();
            if elapsed < Duration::from_millis(200) {
                sleep(Duration::from_millis(200) - elapsed).await;
            }
        }

        let mut write = self.write.lock().await;
        write.send(Message::Text(String::from(command))).await?;
        *last_request_time = Some(Instant::now());
        debug!("Sent: {:?}", command);

        Ok(())
    }

    pub async fn receive(&self) -> Result<String, Error> {
        let mut read = self.read.lock().await;
        loop {
            let message = match timeout(PING_INTERVAL * 3, read.next()).await {
                Ok(message) => message,
                Err(_) => return Err(Error::ConnectionTimeout),
            };

            debug!("Received: {:?}", message);
            match message {
                Some(Ok(Message::Text(string))) => return Ok(string),
                Some(Ok(Message::Binary(_))) | Some(Ok(Message::Ping(_))) | Some(Ok(Message::Pong(_))) => continue,
                Some(Ok(Message::Close(_))) => return Err(Error::ConnectionClosed),
                Some(Err(err)) => return Err(Error::WebSocketError(err)),
                None => return Err(Error::NoDataReceived),
            };
        }
    }

    fn spawn_pinging_task(&self) {
        let write = Arc::downgrade(&self.write);
        tokio::spawn(async move {
            while let Some(write) = write.upgrade() {
                let mut write = write.lock().await;
                write.send(Message::Ping(Vec::new())).await.ok();
                debug!("Sent: Ping([])");

                drop(write); // unlock write object, before sleep
                sleep(PING_INTERVAL).await;
            }
        });
    }
}
