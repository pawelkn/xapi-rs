use xapi;

use std::error::Error;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::loads(&json)?;

    let x = xapi::connect(&credentials).await?;

    let symbols = vec!["BITCOIN", "ETHEREUM"];
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64 - 60000;

    let response = x.socket.get_tick_prices(symbols, timestamp, 0).await?;
    println!("{:?}", response.return_data);

    Ok(())
}
