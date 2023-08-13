#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std] // std support is experimental

use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::rt::TokioIo;
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::Digest;
use serde;
use serde::{Deserialize, Serialize};
// use tokio::net::TcpStream;

risc0_zkvm::guest::entry!(main);

// #[tokio::main]
pub fn main() {
    // TODO: Implement your guest code here
    // let binance_spot: Vec<BinanceSpot> =
   let data: Vec<BinanceSpot> = env::read();
    // let price = binance_spot[0].price.parse::<f64>().unwrap();
    let price = data[0].price.parse::<f64>().unwrap() as f64;
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
