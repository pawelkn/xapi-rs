use xapi;

use std::error::Error;
use std::fs;

/*
    User should send requests in 200 ms intervals. This rule can be broken,
    but if it happens 6 times in a row the connection is dropped.

    The xapi takes care of the necessary delays by itself.
    But this restriction can be omitted by using the skip_delay() method.
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::from(&json)?;

    let x = xapi::connect(&credentials).await?;

    let symbols = vec![
        "BITCOIN", "ETHEREUM", "LITECOIN", "RIPPLE", "EURUSD", "EURPLN", "GOLD", "SILVER",
    ];
    for symbol in symbols {
        let response = x.socket.get_symbol(symbol).await?;
        println!("{symbol}: {:?}", response.return_data);

        x.socket.skip_delay().await;
    }

    Ok(())
}
