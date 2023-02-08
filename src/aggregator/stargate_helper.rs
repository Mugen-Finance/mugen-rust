use crate::aggregator::arbitrum_swaps;
use crate::constants::*;
use ethers::abi::AbiEncode;
use ethers::prelude::*;

pub fn _generate_stargate(
    dst_chain_id: u16,
    token: ::ethers::core::types::Address,
    src_pool_id: ::ethers::core::types::U256,
    dst_pool_id: ::ethers::core::types::U256,
    amount: ::ethers::core::types::U256,
    amount_min: ::ethers::core::types::U256,
    dust_amount: ::ethers::core::types::U256,
    receiver: ::ethers::core::types::Address,
    to: ::ethers::core::types::Address,
    gas: ::ethers::core::types::U256,
    src_context: [u8; 32],
) -> (u8, Bytes) {
    let params = AbiEncode::encode(arbitrum_swaps::StargateParams {
        dst_chain_id,
        token,
        src_pool_id,
        dst_pool_id,
        amount,
        amount_min,
        dust_amount,
        receiver,
        to,
        gas,
        src_context,
    });
    let data = Bytes::from(params);
    let step = 15;
    return (step, data);
}

pub fn _get_pools(dst_chain_id: u16) -> (U256, U256) {
    let pool_2 = match dst_chain_id {
        110 => 1,
        _ => 0,
    };
    return (U256::from(pool_2), U256::from(dst_chain_id));
}

pub fn _get_bridge_token_path(src_chain_id: u64, dst_chain_id: u64) -> Vec<String> {
    let mut path: Vec<String> = Vec::new();
    let chain_1 = match src_chain_id {
        STARGATE_BINANCE => BUSD,
        STARGATE_ARBITRUM => USDC,
        STARGATE_ETHEREUM => USDC,
        STARGATE_AVAX => USDC,
        STARGATE_FANTOM => USDC,
        STARGATE_OPTIMISM => USDC,
        STARGATE_POLYGON => USDC,
        _ => "Error",
    };

    let chain_2 = match dst_chain_id {
        STARGATE_BINANCE => BUSD,
        STARGATE_ARBITRUM => USDC,
        STARGATE_ETHEREUM => USDC,
        STARGATE_AVAX => USDC,
        STARGATE_FANTOM => USDC,
        STARGATE_OPTIMISM => USDC,
        STARGATE_POLYGON => USDC,
        _ => "Error",
    };
    path.push(chain_1.to_string());
    path.push(chain_2.to_string());
    path
}
