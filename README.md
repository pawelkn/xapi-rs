# xStation5 API Rust Library

[![Version](https://img.shields.io/crates/v/xapi.svg)](https://crates.io/crates/xapi)
[![Build](https://img.shields.io/github/actions/workflow/status/pawelkn/xapi/test.yml)](https://github.com/pawelkn/xapi-rs/actions/workflows/test.yml)
[![Docs](https://img.shields.io/docsrs/xapi)](https://docs.rs/xapi)
[![Downloads](https://img.shields.io/crates/d/xapi.svg)](https://crates.io/crates/xapi)
[![license](https://img.shields.io/badge/license-MIT-blue)](https://github.com/pawelkn/xapi-rs/LICENSE)

The xStation5 API Rust library provides a simple and easy-to-use API for interacting with the xStation5 trading platform. With this library, you can connect to the xStation5 platform, retrieve market data, and execute trades.

This library may be used for [BFB Capital](https://bfb.capital) and [XTB](https://www.xtb.com) xStation5 accounts.

API documentation: <http://developers.xstore.pro/documentation>

## Disclaimer

This xStation5 API Rust library is not affiliated with, endorsed by, or in any way officially connected to the xStation5 trading platform or its parent company. The library is provided as-is and is not guaranteed to be suitable for any particular purpose. The use of this library is at your own risk, and the author(s) of this library will not be liable for any damages arising from the use or misuse of this library. Please refer to the license file for more information.

## Installation

You can install xAPI using cargo. Simply run the following command:

```shell
cargo add xapi
```

## Usage

To use xAPI, you will need to have an active account with the xStation5 trading platform. Once you have an account, you can use the xAPI library to connect to the platform and begin trading.

Here is an example of how to use the xAPI library to connect to the xStation5 platform:

```rust
use xapi;

use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::loads(&json)?;

    let x = xapi::connect(&credentials).await?;
    Ok(())
}
```

Once you have connected to the platform, you can use the xAPI object to retrieve market data and execute trades.

Here is an example of how to subscribe to market data using the xAPI library:

```rust
use xapi;

use tokio::time::{Duration, sleep};
use std::error::Error;
use std::fs;

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
```

And here is an example of how to execute a trade using the xAPI library:

```rust
use xapi;
use xapi::{Transaction, RequestStatus, TradeCmd, TradeType};

use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = fs::read_to_string("credentials.json")?;
    let credentials = xapi::Credentials::loads(&json)?;

    let x = xapi::connect(&credentials).await?;

    let response = x.socket.trade_transaction(Transaction {
        symbol: String::from("BITCOIN"),
        cmd: TradeCmd::BuyLimit,
        type_: TradeType::Open,
        price: 10.00,
        volume: 1.0,
    ..Default::default() }).await?;

    if response.status == true {
        println!("Transaction sent to market")
    } else {
        println!("Failed to trade a transaction {:?}", response);
    }
    Ok(())
}
```

## Examples

To run the examples for the xAPI library, you will need to have an account with the xStation5 trading platform.

Before running the examples, you should create a file called _credentials.json_ in the project directory. This file should contain your account credentials, like this:

### credentials.json

```json
{
    "accountId": "<your_client_id>",
    "password": "<your_password>",
    "host": "ws.xtb.com",
    "type": "real",
    "safe": false
}
```

Once you have created the _credentials.json_ file, you can run an example using the following command:

```shell
cargo run --example listen-tick-prices
```

## Unit Tests

This will run all of the unit tests:

```shell
git clone https://github.com/pawelkn/xapi-rs.git
cd xapi-rs
cargo test
```

## Buy Me A Coffee! ☕

If you find the project beneficial and would like to support me, please consider showing your appreciation by buying me a coffee on [Buy Me A Coffee](https://buycoffee.to/pawelkn)
