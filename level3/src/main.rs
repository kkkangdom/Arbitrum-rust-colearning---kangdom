use alloy::providers::{Provider, ProviderBuilder};
use alloy_primitives::U256;
use std::error::Error;

/// 标准 ERC20 转账的 Gas 限额（行业通用值）
const STANDARD_TRANSFER_GAS_LIMIT: u64 = 21_000;

/// 计算预估转账 Gas 费
/// Gas 费 = Gas 价格 × Gas 限额
fn calculate_transfer_gas_fee(gas_price: U256, gas_limit: u64) -> U256 {
    let gas_limit_u256 = U256::from(gas_limit);
    gas_price * gas_limit_u256
}

/// 将 Wei 转换为 ETH
fn wei_to_eth(wei: U256) -> f64 {
    // 将 U256 转换为字符串，然后处理
    let wei_str = wei.to_string();
    let wei_value: f64 = wei_str.parse().unwrap_or(0.0);
    wei_value / 1e18
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Arbitrum Sepolia Gas 费用计算 ===\n");

    // 连接到 Arbitrum Sepolia RPC
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url);

    println!("✓ 已连接到 Arbitrum Sepolia 测试网\n");

    // 获取当前区块信息
    let block_number = provider.get_block_number().await?;
    println!("当前区块高度: {}\n", block_number);

    // 动态获取实时 Gas 价格
    let gas_price_u128 = provider.get_gas_price().await?;
    let gas_price = U256::from(gas_price_u128);
    println!("当前 Gas 价格: {} Wei/unit", gas_price);

    // 计算预估转账 Gas 费
    let estimated_gas_fee = calculate_transfer_gas_fee(gas_price, STANDARD_TRANSFER_GAS_LIMIT);

    println!("\n=== Gas 费用计算结果 ===");
    println!("Gas 限额（标准转账）: {} units", STANDARD_TRANSFER_GAS_LIMIT);
    println!("Gas 价格: {} Wei/unit", gas_price);
    println!("预估 Gas 费: {} Wei", estimated_gas_fee);
    println!("预估 Gas 费: {} ETH", wei_to_eth(estimated_gas_fee));

    
    Ok(())
}
