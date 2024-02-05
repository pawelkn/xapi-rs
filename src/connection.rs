use crate::data::ErrorResponse;
use crate::error::Error;

use futures::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::time::Instant;

use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::{sleep, timeout, Duration};

use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

const PING_INTERVAL: Duration = Duration::from_secs(5);
const WRITE_DELAY: Duration = Duration::from_millis(200);
const WRITE_AT_ONCE: u64 = 6;

#[derive(Debug)]
pub struct Reader {
    stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

#[derive(Debug)]
pub struct Writer {
    sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    last: Option<Instant>,
    counter: u64,
}

#[derive(Debug, Clone)]
pub struct Connection {
    writer: Arc<Mutex<Writer>>,
    reader: Arc<Mutex<Reader>>,
    transaction: Arc<Mutex<()>>,
}

impl Connection {
    pub async fn connect(url: &str) -> Result<Connection, Error> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
        let (sink, stream) = ws_stream.split();

        let conn = Connection {
            writer: Arc::new(Mutex::new(Writer { sink, last: None, counter: 0 })),
            reader: Arc::new(Mutex::new(Reader { stream })),
            transaction: Arc::new(Mutex::new(())),
        };

        conn.spawn_pinging_task();

        Ok(conn)
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
        let mut writer = self.writer.lock().await;
        writer.rate_limit().await;

        writer.sink.send(Message::Text(String::from(command))).await?;
        writer.last = Some(Instant::now());

        Ok(())
    }

    pub async fn receive(&self) -> Result<String, Error> {
        let mut reader = self.reader.lock().await;
        loop {
            let result = match timeout(PING_INTERVAL * 3, reader.stream.next()).await {
                Ok(result) => result,
                Err(_) => return Err(Error::ConnectionTimeout),
            };

            let message = match result {
                Some(data) => data?,
                None => return Err(Error::NoDataReceived),
            };

            return match message {
                Message::Text(string) => Ok(string),
                Message::Binary(_) | Message::Ping(_) | Message::Pong(_) => continue,
                Message::Close(_) => Err(Error::ConnectionClosed),
            };
        }
    }

    fn spawn_pinging_task(&self) {
        let writer = Arc::downgrade(&self.writer);
        tokio::spawn(async move {
            while let Some(writer) = writer.upgrade() {
                let mut writer = writer.lock().await;
                writer.sink.send(Message::Ping(Vec::new())).await.ok();
                drop(writer); // unlock writer object, before sleep

                sleep(PING_INTERVAL).await;
            }
        });
    }
}

impl Writer {
    async fn rate_limit(&mut self) {
        if let Some(last) = self.last {
            let elapsed = last.elapsed();
            if elapsed < WRITE_DELAY {
                if self.counter < WRITE_AT_ONCE {
                    self.counter += 1;
                } else {
                    self.counter = 0;
                    sleep(WRITE_DELAY - elapsed).await;
                }
            } else {
                self.counter = 0;
            }
        }
    }
}
