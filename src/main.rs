extern crate web3;
extern crate diesel;
extern crate eth_filter;
extern crate redis;

use web3::futures::Future;
use web3::types::{TransactionRequest, U256, U64, H256, SyncState};
use std::{thread, time};

use self::eth_filter::*;
// use self::models::*;
// use self::diesel::prelude::*;
use tokio::time::{sleep, Duration};

mod w3;
mod db;

async fn get_last_block() -> i32 {
    let mut con = db::connection().await;
    let last: i32 = redis::cmd("get").arg("eth.latest.block").query_async(&mut con).await.unwrap_or(0);

    if last > 0 {
        return last;
    }

    let last_block = option_env!("LATEST_BLOCK");
    let last_block = last_block.unwrap_or("9671241");
    return last_block.parse::<i32>().unwrap();
}

async fn get_latest_block() -> U64 {
    return match w3::WEB3.eth().block_number().await {
        Ok(block_number) => block_number,
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    };
}

async fn set_block(n: i32) {
    // Move into loop
    let mut con = db::connection().await;
    redis::cmd("set").arg("eth.latest.block").arg(n.to_string()).query_async(&mut con).await.unwrap_or(0);
}

async fn get_block(block: i32) -> web3::types::Block<H256> {
    let block = block as u64;
    let res = w3::WEB3.eth().block(U64([block]).into()).await;
    // return res;

    match res {
        Ok(_x) => {
            return _x.unwrap();
        }
        Err(e) => {
            panic!("this is a terrible mistake! {:?}", e);
        }
    }
}

async fn process() {
    let latest_block = get_latest_block().await;
    let latest_block = latest_block.low_u64();
    let latest_block = latest_block as i32;

    println!("Latest Block Number: {:?}", latest_block);

    let last_block = get_last_block().await;

    println!("{}, {}", last_block, latest_block);
    for n in last_block..latest_block {
        set_block(n).await;
        let data = get_block(n).await;
        println!("xxxxxxxx {:?}", data);
    }
}

async fn run() {
    loop {
        println!("Running checks");
        process().await;
        sleep(Duration::from_millis(10000)).await;
    }
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    w3::init();
    db::init();

    let res = w3::WEB3.eth().syncing().await;
    match res {
        Ok(_x) => {}
        Err(e) => {
            panic!("this is a terrible mistake! {:?}", e);
        }
    }

    match w3::WEB3.eth().syncing().await? {
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
