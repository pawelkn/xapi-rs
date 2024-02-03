use xapi;

use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::from(&json)?;

    let x = xapi::connect(&credentials).await?;

    let response = x.socket.get_margin_trade("EURPLN", 1.0).await?;
    println!("{:?}", response.return_data);

    Ok(())
}
