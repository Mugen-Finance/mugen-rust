use ethers::{abi::Address, types::U256};
use reqwest::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Sources {
    name: String,
    proportion: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderData {
    #[serde(rename = "tokenAddressPath")]
    pub token_address_path: Option<Vec<String>>,
    pub path: Option<Vec<String>>,
    pub router: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Fill {
    input: String,
    output: String,
    #[serde(rename = "adjustedOutput")]
    adjusted_output: String,
    gas: U256,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Orders {
    #[serde(rename = "type")]
    _type: u32,
    pub source: String,
    #[serde(rename = "makerToken")]
    maker_token: String,
    #[serde(rename = "takerToken")]
    taker_token: String,
    #[serde(rename = "makerAmount")]
    maker_amount: String,
    #[serde(rename = "takerAmount")]
    pub taker_amount: Option<String>,
    #[serde(rename = "fillData")]
    pub fill_data: OrderData,
    //  #[serde(rename = "sourcePathId")]
    //  source_path_id: String,
    //fill: Fill,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Responses {
    #[serde(rename = "chainId")]
    chain_id: u64,
    price: String,
    #[serde(rename = "guaranteedPrice")]
    guaranteed_price: String,
    #[serde(rename = "estimatedPriceImpact")]
    estimated_price_impact: String,
    to: String,
    data: String,
    value: String,
    gas: String,
    #[serde(rename = "estimatedGas")]
    estimated_gas: String,
    #[serde(rename = "gasPrice")]
    gas_price: String,
    #[serde(rename = "protocolFee")]
    protocol_fee: String,
    #[serde(rename = "minimumProtocolFee")]
    minimum_protocol_fee: String,
    #[serde(rename = "buyTokenAddress")]
    buy_token_address: String,
    #[serde(rename = "sellTokenAddress")]
    sell_token_address: String,
    #[serde(rename = "buyAmount")]
    buy_amount: String,
    #[serde(rename = "sellAmount")]
    sell_amount: String,
    sources: Vec<Sources>,
    pub orders: Vec<Orders>,
    #[serde(rename = "allowanceTarget")]
    allowance_target: String,
    #[serde(rename = "decodedUniqueId")]
    decoded_unique_id: String,
    #[serde(rename = "sellTokenToEthRate")]
    sell_token_to_eth_rate: String,
    #[serde(rename = "buyTokenToEthRate")]
    buy_token_to_eth_rate: String,
    //  #[serde(rename = "expectedSlippage")]
    //expected_slippage: u32,
}

pub async fn router_path<'a>(
    _buy_token: Address,
    _sell_token: Address,
    _amount: U256,
) -> Responses {
    let res: Responses =  get("https://api.0x.org/swap/v1/quote?buyToken=0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2&sellToken=0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984&sellAmount=10000000000000000000000000000&excludedSources=0x,Aave_V2,Balancer,Balancer_V2,Bancor,BancorV3,Component,Compound,CryptoCom,Curve,Curve_V2,DODO,DODO_V2,KyberDMM,Lido,MakerPsm,mStable,Saddle,Shell,ShibaSwap,Synapse,Synthetix,Uniswap,Uniswap_V2").await.unwrap().json().await.unwrap();
    println!("{res:#?}");
    res
}
