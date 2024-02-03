use xapi;

use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::from(&json)?;

    let x = xapi::connect(&credentials).await?;

    let start = 1701126000000; // 2023-11-28 00:00:00
    let end = 1703718000000; // 2023-12-28 00:00:00

    let response = x.socket.get_ibs_history(start, end).await?;
    println!("{:?}", response.return_data);

    Ok(())
}
