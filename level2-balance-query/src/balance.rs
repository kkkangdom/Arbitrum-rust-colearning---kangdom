use ethers::prelude::*;
use ethers::utils::format_units; // 用于单位转换
use std::str::FromStr; // 用于解析地址字符串

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 连接 Arbitrum Sepolia 节点
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let provider = Provider::<Http>::try_from(rpc_url)?;

   
    let wallet_address = "0x870E94Fdd39D3F958CdDd3859761d0Edb845A93A"; // 示例地址
    
    // 将字符串转换为 Address 类型
    let address = Address::from_str(wallet_address)?;

    println!("正在查询地址: {}", wallet_address);

    // 3. 查询余额 (返回的是 Wei，这是一个巨大的整数)
    let balance_wei = provider.get_balance(address, None).await?;

    // 4. 格式化余额 (将 Wei 转换为 Ether)
    // 1 ETH = 10^18 Wei
    // format_units 会帮我们要么除以 10^18 并处理小数点
    let balance_eth = format_units(balance_wei, "ether")?;

    // 5. 打印结果
    println!("--------------------------------");
    println!("原始余额 (Wei): {}", balance_wei);
    println!("可读余额 (ETH): {} ETH", balance_eth);
    println!("--------------------------------");

    Ok(())
}