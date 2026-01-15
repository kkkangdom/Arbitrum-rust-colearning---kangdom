//! Example of creating an HTTP provider using the `connect_http` method on the `ProviderBuilder`.
 
use alloy::providers::{Provider, ProviderBuilder}; 
use std::error::Error;
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "https://api.zan.top/arb-sepolia".parse()?;
        
    // Create a provider with the HTTP transport using the `reqwest` crate.
    let provider = ProviderBuilder::new().connect_http(rpc_url); 
 
    let latest_block = provider.get_block_number().await?;

    println!("Latest block number: {}", latest_block);
    // 4. 打印结果 (Hello Web3)
    println!("Hello Web3! Arbitrum 链连接成功！");
    println!("当前区块高度: {}", latest_block);
    Ok(())
}
