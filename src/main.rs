extern crate web3;
extern crate diesel;
extern crate eth_filter;

use web3::futures::Future;
use web3::types::{TransactionRequest, U256, U64, SyncState};
use std::{thread, time};

use self::eth_filter::*;
use self::models::*;
use self::diesel::prelude::*;
use lazy_static::lazy_static;

type Pool = web3::Web3<web3::transports::http::Http>;

lazy_static! {
    static ref WEB3: Pool = {
        let web3_url = option_env!("WEB3_URL");
        println!("W3 URL is set to {:?}", web3_url);

        let web3_url = web3_url.unwrap_or("http://localhost:8545");

        let transport = web3::transports::Http::new(web3_url).unwrap();
        return web3::Web3::new(transport);
    };
}

async fn get_last_block() -> i32 {
    // TODO not the last block
    let last_block = option_env!("LATEST_BLOCK");
    let last_block = last_block.unwrap_or("966591");
    return last_block.parse::<i32>().unwrap();
}

async fn get_latest_block() -> U64 {
    return match WEB3.eth().block_number().await {
        Ok(block_number) => block_number,
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    };
}

async fn process() {
    let latest_block = get_latest_block().await;
    let latest_block = latest_block.low_u64();
    let latest_block = latest_block as i32;

    println!("Latest Block Number: {:?}", latest_block);

    let last_block = get_last_block().await;
   
    println!("{}, {}", last_block, latest_block);
    for n in last_block..latest_block {
        println!("{}", n);
    }
}

async fn run() {
    process().await;

    // loop {
    //     println!("Running checks");

    //     let ten_millis = time::Duration::from_secs(10);
    //     let now = time::Instant::now();

    //     thread::sleep(ten_millis);

    //     assert!(now.elapsed() >= ten_millis);
    // }
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    lazy_static::initialize(&WEB3);

    let res = WEB3.eth().syncing().await;
    match res {
        Ok(_x) => {}
        Err(e) => {
            panic!("this is a terrible mistake! {:?}", e);
        }
    }

    match WEB3.eth().syncing().await? {
        SyncState::Syncing(sync_info) => {
            let current: u64 = sync_info.current_block.as_u64();
            panic!("Blockchain syncing! {:?}", current);
        }
        SyncState::NotSyncing => {
            println!("Blockchain insync.");
        }
    };

    run().await;

    Ok(())
}
