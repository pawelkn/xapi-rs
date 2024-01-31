use xapi;

use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::loads(&json)?;

    let x = xapi::connect(&credentials).await?;

    let symbols = vec!["BITCOIN", "ETHEREUM"];

    let response = x.socket.get_trading_hours(symbols).await?;
    println!("{:?}", response.return_data);

    Ok(())
}
