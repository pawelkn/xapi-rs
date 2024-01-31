use xapi;

use chrono::Utc;

use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::loads(&json)?;

    let x = xapi::connect(&credentials).await?;

    let symbols = vec!["BITCOIN", "ETHEREUM"];
    let timestamp = Utc::now().timestamp_millis() - 60000;

    let response = x.socket.get_tick_prices(symbols, timestamp, 0).await?;
    println!("{:?}", response.return_data);

    Ok(())
}
