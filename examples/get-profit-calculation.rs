use xapi;
use xapi::TradeCmd;

use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::loads(&json)?;

    let x = xapi::connect(&credentials).await?;

    let response = x
        .socket
        .get_profit_calculation("EURPLN", TradeCmd::Buy, 1.2233, 1.3000, 1.0)
        .await?;
    println!("{:?}", response.return_data);

    Ok(())
}
