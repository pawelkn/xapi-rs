use xapi;

use futures::future::join_all;
use std::error::Error;
use std::fs;
use tokio::time::{sleep, Duration};

async fn get_tick_prices(x: xapi::XApi) -> Result<(), xapi::Error> {
    x.stream.get_tick_prices("BITCOIN", 0, 0).await?;
    x.stream.get_tick_prices("ETHEREUM", 0, 0).await
}

async fn listen(x: xapi::XApi) -> Result<(), xapi::Error> {
    loop {
        let record = x.stream.listen().await?;
        println!("{:?}", record);
    }
}

async fn listen_tick_prices(credentials: &xapi::Credentials) -> Result<(), xapi::Error> {
    let x = xapi::connect(&credentials).await?;

    let handles = vec![
        tokio::spawn(get_tick_prices(x.clone())),
        tokio::spawn(listen(x.clone())),
    ];

    let results = join_all(handles).await;
    for result in results {
        if let Err(err) = result.unwrap() {
            return Err(err);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::from(&json)?;

    while let Err(err) = listen_tick_prices(&credentials).await {
        println!("{}, Reconnecting in 5 seconds ...", err);
        sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}
