use alloy_chains::Chain;
use foundry_block_explorers::Client;

use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Http, Provider},
};
use eyre::Result;
use std::sync::Arc;

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    struct MyIpAddress {
        origin: String,
    }

    extern "Rust" {
        //async fn get_contract_name_from_rust(contractAddr: String) -> MyIpAddress;
        async fn uniswapv2_pair() -> MyIpAddress;

    }
}
// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    IUniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
    ]"#,
);

async fn uniswapv2_pair() -> ffi::MyIpAddress {
    let client = Provider::<Http>::try_from("https://eth.llamarpc.com").unwrap();
    let client = Arc::new(client);

    // ETH/USDT pair on Uniswap V2
    let address = "0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852"
        .parse::<Address>()
        .unwrap();
    let pair = IUniswapV2Pair::new(address, Arc::clone(&client));

    // getReserves -> get_reserves
    let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await.unwrap();
    println!("Reserves (ETH, USDT): ({reserve0}, {reserve1})");

    let mid_price = f64::powi(10.0, 18 - 6) * reserve1 as f64 / reserve0 as f64;
    println!("ETH/USDT price: {mid_price:.2}");
    ffi::MyIpAddress {
        origin: "OK".to_string(),
    }
}


//  Once we support returning Result from an async function.
async fn get_contract_name_from_rust(contractAddr: String) -> ffi::MyIpAddress {
    //println!("Using foundry_block_explorers::Client::contract_source_code from the Rust side...");
    println!("{}",contractAddr);

    let client = Client::new(Chain::mainnet(), "IXH6AE5PJPG28H3U7UFP64YV3FG19W3FVD");
    let address: Address = contractAddr.parse().unwrap();

    let metadata = client.expect("REASON")
    .contract_source_code(address)
    .await
    .unwrap();


    let origin = &metadata.items[0].contract_name;

    println!("Foundry_block_explorers complete. Returning the value to Swift...");

    ffi::MyIpAddress { origin:origin.to_string() }
}

