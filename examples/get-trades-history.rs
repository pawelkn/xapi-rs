use xapi;

use chrono::prelude::*;
use chrono::Utc;

use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::loads(&json)?;

    let x = xapi::connect(&credentials).await?;

    let start = Utc
        .with_ymd_and_hms(2023, 11, 28, 0, 0, 0)
        .unwrap()
        .timestamp_millis();
    let end = Utc
        .with_ymd_and_hms(2024, 1, 28, 0, 0, 0)
        .unwrap()
        .timestamp_millis();

    let response = x.socket.get_trades_history(start, end).await?;
    println!("{:?}", response.return_data);

    Ok(())
}
