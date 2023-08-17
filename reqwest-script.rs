#!/usr/bin/env -S cargo +nightly -Zscript

//! ```cargo
//! [dependencies]
//! reqwest = { version = "0.11", features = ["json"] }
//! tokio = { version = "1", features = ["full"] }
//! ```

use reqwest;


#[tokio::main]
async fn main() {
    let resp = match reqwest::get("https://httpbin.org/ip").await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => panic!("Error: {}", err)
    };
    println!("{}", resp)
}