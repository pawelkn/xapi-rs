use xapi;
use xapi::{RequestStatus, TradeCmd, TradeType, Transaction};

use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::from(&json)?;

    let x = xapi::connect(&credentials).await?;

    let response = x
        .socket
        .trade_transaction(Transaction {
            symbol: String::from("BITCOIN"),
            cmd: TradeCmd::BuyLimit,
            type_: TradeType::Open,
            price: 10.00,
            volume: 1.0,
            ..Default::default()
        })
        .await?;

    if response.status != true {
        println!("Failed to trade a transaction {:?}", response);
        return Ok(());
    }

    let order = response.return_data.order;
    let response = x.socket.trade_transaction_status(order).await?;
    if response.status != true {
        println!("Failed to trade a transaction {:?}", response);
        return Ok(());
    }

    let status = response.return_data;
    match status.request_status {
        RequestStatus::Error => {
            println!("The transaction finished with error {:?}", status.message)
        }
        RequestStatus::Pending => {
            println!("The transaction is pending")
        }
        RequestStatus::Accepted => {
            println!("The transaction has been executed successfully")
        }
        RequestStatus::Rejected => {
            println!("The transaction has been rejected {:?}", status.message)
        }
        RequestStatus::Invalid => {
            println!("Invalid transaction status {:?}", status.message)
        }
    };

    Ok(())
}
