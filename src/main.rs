use std::env;
extern crate web3;
use web3::futures::Future;
use web3::types::{TransactionRequest, U256, SyncState};

fn w3() -> web3::Web3<web3::transports::http::Http> {
    let web3_url = option_env!("WEB3_URL");
    println!("W3 URL is set to {:?}", web3_url);

    let web3_url = web3_url.unwrap_or("http://localhost:8545");

    let transport = web3::transports::Http::new(web3_url).unwrap();
    return web3::Web3::new(transport);
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    let w3 = w3();
    let accounts = w3.eth().accounts().await?;

    println!("Accounts: {:?}", accounts);

    let wei_conv: U256 = U256::exp10(18);
    for account in accounts {
        let balance = w3.eth().balance(account, None).await?;
        println!(
            "Eth balance of {:?}: {}",
            account,
            balance.checked_div(wei_conv).unwrap()
        );
    }

    let res = w3.eth().syncing().await;
    match res {
        Ok(_x) => {}
        Err(e) => {
            panic!("this is a terrible mistake! {:?}", e);
        }
    }

    let x = match w3.eth().syncing().await? {
        SyncState::Syncing(sync_info) => {
            let current: u64 = sync_info.current_block.as_u64();
            panic!("Blockchain syncing! {:?}", current);
        }
        SyncState::NotSyncing => {
            println!("Blocking in sync.");
        }
    };

    Ok(())
}
