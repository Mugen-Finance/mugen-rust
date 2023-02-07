use ethers::abi::AbiEncode;
use ethers::types::{Address, Bytes, U256};
use eyre::Result;
use std::str::FromStr;
mod smart_router_calls;
mod weth;

use crate::smart_router_calls::*;

//TODO:
// Add batch deposits
// Add xcal
// Add Stargate
// Refactor and make a lib for inputs

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
struct CamelotParams {
    amount_in: U256,
    path: Vec<Address>,
    referrer: Address,
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
struct UniswapV3Multi {
    amount_in: U256,
    amount_out_min: U256,
    token_1: Address,
    token_2: Address,
    token_3: Address,
    fee_1: u32,
    fee_2: u32,
}


pub async fn run(_tokens: Vec<Address>, _amounts: Vec<U256>, _chains: Vec<u64>, _exchanges: Vec<String>) -> Result<()> {
    let token_path: Vec<String> = Vec::new();
    let exchange = String::from("Camelot");
    let aggregate = true;
    let addr_1 = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
        .parse::<Address>()
        .unwrap();
    let addr_2 = "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984"
        .parse::<Address>()
        .unwrap();
    let amount = U256::from(1) * U256::exp10(18);
    let mut steps: Vec<u8> = Vec::new();
    let mut data: Vec<Bytes> = Vec::new();
    if aggregate == true {
        let response = router_path(addr_1, addr_2, amount).await;

        //let orders: Vec<Orders> = response.orders;
        let order = response.orders;

        //How to get the taker amount to stick?

        for order in order {
            let amount: String = order.taker_amount.unwrap().to_string();
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
                        let encoded_data = Bytes::from(params);
                        steps.push(4);
                        data.push(encoded_data);
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
            }
        }
    }

    if exchange == "Camelot" {
        let mut path: Vec<Address> = Vec::new();
        for token in token_path {
            let i = token.parse::<Address>().unwrap();
            path.push(i);
        }
        let swap_params = AbiEncode::encode(CamelotParams {
            amount_in: U256::from(0),
            path: path,
            referrer: "0x6Cb6D9Fb673CfbF31b3A432F6316fE3196efd4aA"
                .parse::<Address>()
                .unwrap(),
            deadline: U256::from(0),
        });
        let encoded_data = Bytes::from(swap_params);
        steps.push(7);
        data.push(encoded_data);
    } else if exchange == "3xcal" {
    }
    println!("{:#?}, {:#?}", steps, data);
    Ok(())
}
