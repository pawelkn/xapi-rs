use xapi;

use std::error::Error;
use std::fs;
use tokio::time::{sleep, Duration};

async fn listen_tick_prices(credentials: &xapi::Credentials) -> Result<(), xapi::Error> {
    let x = xapi::connect(&credentials).await?;

    x.stream.get_tick_prices("BITCOIN", 0, 0).await?;
    x.stream.get_tick_prices("ETHEREUM", 0, 0).await?;

    loop {
        let record = x.stream.listen().await?;
        println!("{:?}", record);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::loads(&json)?;

    while let Err(err) = listen_tick_prices(&credentials).await {
        println!("{}, Reconnecting in 5 seconds ...", err);
        sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}
