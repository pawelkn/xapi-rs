use xapi;

use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::from(&json)?;

    let x = xapi::connect(&credentials).await?;

    let orders = vec![7489839, 7489841];

    let response = x.socket.get_trade_records(orders).await?;
    println!("{:?}", response.return_data);

    Ok(())
}
