#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::Digest;
use serde;
use serde::{Deserialize, Serialize};
use reqwest;
risc0_zkvm::guest::entry!(main);

#[tokio::main]
pub async fn main() {
    // TODO: Implement your guest code here
    let binance_spot: Vec<BinanceSpot> = reqwest::get("https://api.binance.us/api/v3/ticker/price")
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let price = binance_spot[0].price.parse::<f64>().unwrap();
    env::commit(&price)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceSpot {
    pub symbol: String,
    pub price: String,
}

// #[derive(Eq, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct Outputs{
//     pub data: u32,
// }
