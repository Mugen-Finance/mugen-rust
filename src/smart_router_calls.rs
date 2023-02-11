use crate::{
    aggregator::stargate_helper::{_get_bridge_token_path},
    constants::{
        STARGATE_ARBITRUM, STARGATE_AVAX, STARGATE_BINANCE, STARGATE_ETHEREUM, STARGATE_FANTOM,
        STARGATE_OPTIMISM, STARGATE_POLYGON,
    },
};
use ethers::{
    abi::{AbiEncode, Address},
    types::{Bytes, U256},
};
use reqwest::*;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, fmt::format};

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
)]
struct SushiParams {
    amount_in: U256,
    amount_out_min: U256,
    path: Vec<Address>,
    send_tokens: bool,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
)]
struct UniswapV2Params {
    amount_in: U256,
    amount_out_min: U256,
    path: Vec<Address>,
    deadline: U256,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
)]
struct UniswapV3Single {
    amount_in: U256,
    amount_out_min: U256,
    token_1: Address,
    token_2: Address,
    pool_fee: u32,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
)]
struct VeloParams {
    amount_in: U256,
    amount_out_min: U256,
    routes: Vec<Route>,
    deadline: U256
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
)]
struct Route {
    from: Address,
    to: Address,
    stable: bool
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
)]
struct UniswapV3Multi {
    amount_in: U256,
    amount_out_min: U256,
    token_1: Address,
    token_2: Address,
    token_3: Address,
    fee_1: u32,
    fee_2: u32,
}

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

pub async fn get_url(
    chain_id: u64,
    buy_token: String,
    sell_token: String,
    amount: U256,
) -> String {
    let chain = match chain_id {
        STARGATE_ARBITRUM => "arbitrum.",
        STARGATE_AVAX => "avalanche.",
        STARGATE_BINANCE => "bsc.",
        STARGATE_ETHEREUM => "",
        STARGATE_POLYGON => "polygon.",
        STARGATE_FANTOM => "fantom.",
        STARGATE_OPTIMISM => "optimism.",
        _ => "error",
    };

    let excluded_sources = match chain_id {
        STARGATE_ARBITRUM => "Aave_V3,Balancer_V2,Curve_V2,GMX,MultiHop,Saddle,Synapse,WOOFi",
        STARGATE_AVAX => "Aave_V2,Aave_V3,Curve,Curve_V2,GMX,KyberDMM,MultiHop,Pangolin,Platypus,Synapse,WOOFi",
        STARGATE_BINANCE => "ACryptoS,ApeSwap,BakerySwap,Belt,BiSwap,DODO,DODO_V2,Ellipsis,FirebirdOneSwap,KnightSwap,KyberDMM,MDex,Mooniswap,MultiHop,Nerve,PancakeSwap,Synapse,WaultSwap,WOOFi",
        STARGATE_ETHEREUM => "0x,Aave_V2,Balancer,Balancer_V2,Bancor,BancorV3,Component,Compound,CryptoCom,Curve,Curve_V2,DODO,DODO_V2,KyberDMM,Lido,MakerPsm,mStable,MultiHop,Saddle,Shell,ShibaSwap,Synapse,Synthetix,Uniswap_V2",
        STARGATE_POLYGON => "0x,Aave_V2,Aave_V3,ApeSwap,Balancer_V2,Curve,Curve_V2,Dfyn,DODO,DODO_V2,FirebirdOneSwap,IronSwap,KyberDMM,MeshSwap,mStable,MultiHop,QuickSwap,Synapse,WaultSwap,WOOFi",
        STARGATE_FANTOM => "Beethovenx,Curve,Curve_V2,MorpheusSwap,MultiHop,SpiritSwap,SushiSwap,Synapse,WOOFi,Yoshi",
        STARGATE_OPTIMISM => "Aave_V3,Beethovenx,Curve,Curve_V2,MultiHop,Saddle,Synapse,Synthetix,WOOFi",
        _ => "error"
    };
    let url = format!("https://{chain}api.0x.org/swap/v1/quote?buyToken={buy_token}&sellToken={sell_token}&sellAmount={amount}&excludedSources={excluded_sources}");
    return url;
}

pub async fn router_path(
    chain: u64,
    buy_token: String,
    sell_token: String,
    amount: U256,
) -> Responses {
    let url = get_url(chain, buy_token, sell_token, amount).await;
    println!("{url}");
    let res: Responses = get(url).await.unwrap().json().await.expect("invalid response");
    println!("{res:#?}");
    res
}

// ==============================================================================================================================
// ==============================================================================================================================
// ==============================================================================================================================

// Update to handle the other exchanges, and then add in the excluded sources aspect for other chains

pub async fn single_to_single_aggregate(
    chain_id: u64,
    buy_token: String,
    sell_token: String,
    _amount: U256,
) -> (Vec<u8>, Vec<Bytes>) {
    let mut steps: Vec<u8> = Vec::new();
    let mut data: Vec<Bytes> = Vec::new();
    let response = router_path(chain_id, buy_token, sell_token, _amount).await;
    let orders = response.orders;
    for order in orders {
        let amount: String = order.maker_amount.to_string();
        if order.source == "Uniswap_V3" {
            let tokens = Some(order.fill_data.token_address_path).unwrap().unwrap();
            let token_1 = &tokens[0];
            let token_2 = &tokens[1];
            let token_3 = Some(&tokens[2]);
            match token_3 {
                None => {
                    let params = AbiEncode::encode(UniswapV3Single {
                        amount_in: U256::from_str(&amount).unwrap(),
                        amount_out_min: U256::from(0),
                        token_1: token_1.parse::<Address>().unwrap(),
                        token_2: token_2.parse::<Address>().unwrap(),
                        pool_fee: 500,
                    });
                    let encoded_data = Bytes::from(params);
                    steps.push(3);
                    data.push(encoded_data);
                }
                Some(_) => {
                    let params = AbiEncode::encode(UniswapV3Multi {
                        amount_in: U256::from_str(&amount).unwrap(),
                        amount_out_min: U256::from(0),
                        token_1: token_1.parse::<Address>().unwrap(),
                        token_2: token_2.parse::<Address>().unwrap(),
                        token_3: token_3.unwrap().parse::<Address>().unwrap(),
                        fee_1: 500,
                        fee_2: 500,
                    });
                    steps.push(4);
                    data.push(Bytes::from(params));
                }
            }
        } else if order.source == "SushiSwap" {
            let mut path: Vec<Address> = Vec::new();
            for token in order.fill_data.token_address_path.unwrap() {
                let address = token.parse::<Address>().unwrap();
                path.push(address);
            }
            let params = AbiEncode::encode(SushiParams {
                amount_in: U256::from_str(&amount).unwrap(),
                amount_out_min: U256::from(0),
                path: path,
                send_tokens: true,
            });
            let encoded_data = Bytes::from(params);
            steps.push(5);
            data.push(encoded_data);
        } else if order.source == "PancakeSwap_V2" {
            let mut path: Vec<Address> = Vec::new();
            for token in order.fill_data.token_address_path.unwrap() {
                let address = token.parse::<Address>().unwrap();
                path.push(address);
            }
            let params = AbiEncode::encode(UniswapV2Params {
                amount_in: U256::from_str(&amount).unwrap(),
                amount_out_min: U256::from(0),
                path: path,
                deadline: U256::from(0),
            });
            let encoded_data = Bytes::from(params);
            steps.push(8);
            data.push(encoded_data);
        } else if order.source == "TraderJoe" {
            let mut path: Vec<Address> = Vec::new();
            for token in order.fill_data.token_address_path.unwrap() {
                let address = token.parse::<Address>().unwrap();
                path.push(address);
            }
            let params = AbiEncode::encode(UniswapV2Params {
                amount_in: U256::from_str(&amount).unwrap(),
                amount_out_min: U256::from(0),
                path: path,
                deadline: U256::from(0),
            });
            let encoded_data = Bytes::from(params);
            steps.push(11);
            data.push(encoded_data);
        } else if order.source == "SpookySwap" {
            let mut path: Vec<Address> = Vec::new();
            for token in order.fill_data.token_address_path.unwrap() {
                let address = token.parse::<Address>().unwrap();
                path.push(address);
            }
            let params = AbiEncode::encode(UniswapV2Params {
                amount_in: U256::from_str(&amount).unwrap(),
                amount_out_min: U256::from(0),
                path: path,
                deadline: U256::from(0),
            });
            let encoded_data = Bytes::from(params);
            steps.push(9);
            data.push(encoded_data);
        } else if order.source == "Velodrome" {
            let mut path: Vec<Address> = Vec::new();
            let mut routes: Vec<Route> = Vec::new();
           let token_1 = path[0];
           let token_2 = path[1];
           let token_3 = Some(path[3]);
           routes.push(Route { from: token_1, to: token_2, stable: false });
           match token_3 {
               Some(_) => routes.push(Route { from: token_2, to: token_3.unwrap(), stable: false }),
               None => ()
           }
            for token in order.fill_data.token_address_path.unwrap() {
                let address = token.parse::<Address>().unwrap();
                path.push(address);
            }
            let params = AbiEncode::encode(VeloParams{ amount_in: U256::from_str(&amount).unwrap(), amount_out_min: U256::from(0), routes: routes, deadline: U256::from(0) });
            let encoded_data = Bytes::from(params);
            steps.push(10);
            data.push(encoded_data);
        }
    }

    return (steps, data);
}

// ==============================================================================================================================
// ==============================================================================================================================
// ==============================================================================================================================

// The amount will be based on the desired spilts sent into the run function.
// Get the api's for the function; run each one through the single
pub async fn _single_to_multi_aggregate(
    chain_id: u64,
    buy_tokens: Vec<String>,
    sell_token: String,
    amounts: Vec<U256>,
) -> (Vec<u8>, Vec<Bytes>) {
    let mut steps = Vec::new();
    let mut data = Vec::new();
    let mut index = 0;
    for token in buy_tokens {
        let (steps, data) =
            single_to_single_aggregate(chain_id, token, sell_token.clone(), amounts[index]).await;
        index += 1;
    }

    return (steps, data);
}

pub async fn _multi_to_single(
    chain_id: u64,
    buy_token: String,
    sell_tokens: Vec<String>,
    amounts: Vec<U256>,
) -> (Vec<u8>, Vec<Bytes>) {
    let mut steps = Vec::new();
    let mut data = Vec::new();
    let mut index = 0;
    for token in sell_tokens {
        let (steps, data) =
            single_to_single_aggregate(chain_id, buy_token.clone(), token, amounts[index]).await;
        index += 1;
    }

    return (steps, data);
}

pub async fn _multi_to_multi() {}

// For these need to figure out what to do about making sure the bridge tokens are used as intermediaries;
pub async fn _cross_single_to_single(
    src_chain_id: u64,
    dst_chain_id: u64,
    buy_token: String,
    sell_token: String,
    amount: U256,
) {
    let bridge_tokens = _get_bridge_token_path(src_chain_id, dst_chain_id);
    let (src_data_steps, src_data) = single_to_single_aggregate(
        src_chain_id,
        bridge_tokens[0].to_owned(),
        sell_token,
        amount,
    )
    .await;
    let (dst_steps, dst_data) = single_to_single_aggregate(
        src_chain_id,
        buy_token,
        bridge_tokens[1].to_owned(),
        amount,
    )
    .await;
}

pub async fn _cross_single_to_multi(
    src_chain_id: u64,
    dst_chain_id: u64,
    buy_tokens: Vec<String>,
    sell_token: String,
    sell_amount: U256,
    buy_amounts: Vec<U256>,
) {
    let bridge_tokens = _get_bridge_token_path(src_chain_id, dst_chain_id);
    let (src_data_steps, src_data) = single_to_single_aggregate(
        src_chain_id,
        bridge_tokens[0].to_owned(),
        sell_token,
        sell_amount,
    )
    .await;
    let (dst_steps, dst_data) = _single_to_multi_aggregate(
        dst_chain_id,
        buy_tokens,
        bridge_tokens[1].to_owned(),
        buy_amounts,
    )
    .await;
}

pub async fn _cross_multi_to_single(
    src_chain_id: u64,
    dst_chain_id: u64,
    buy_token: String,
    sell_tokens: Vec<String>,
    sell_amounts: Vec<U256>,
    buy_amount: U256,
) {
    let bridge_tokens = _get_bridge_token_path(src_chain_id, dst_chain_id);
    let (src_steps, src_data) = _multi_to_single(
        src_chain_id,
        bridge_tokens[0].to_owned(),
        sell_tokens,
        sell_amounts,
    )
    .await;
    let (dst_steps, dst_data) = single_to_single_aggregate(
        dst_chain_id,
        buy_token,
        bridge_tokens[1].to_owned(),
        buy_amount,
    )
    .await;
}

pub async fn _cross_multi_to_multi() {} // this may be easier as we have to get all the assets to just one asset then aggregate that to the other side
