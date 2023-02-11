use ethers::abi::AbiEncode;
use ethers::types::{Address, Bytes, U256};
use eyre::Result;
mod aggregator;
mod constants;
mod exchanges;
mod helpers;
mod smart_router_calls;
mod weth;

use aggregator::arbitrum_swaps::*;

use crate::constants::SWAPS_ADDRESS;
use crate::helpers::get_token_by_chain;
use crate::smart_router_calls::*;

//Change U256 to strings to see if that changes the amount that is being input 

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
struct TokensAndAmounts(Vec<Address>, Vec<U256>);

pub async fn run(
    src_tokens: Vec<String>,
    src_amounts: Vec<U256>,
    _dst_tokens: Vec<String>,
    _src_chain: u64,
    _dst_chain: u64,
    exchanges: Vec<&str>,
    aggregate: bool,
    complex: u8, // There are 4 type corresponding the type of aggregation that happens. 1: Single to single 2: single to multi 3: multi to single and 4: Multi to multi.
                 // Depending on which type is supplied is how the aggregation will be applied on both the src and dst chain.
) -> (Result<()>, Vec<u8>, Vec<Bytes>) {
    //Steps to take when running the program:

    //Checks the tokens coming in for eth and others, Gets data and makes it use able (may be easier to just seperate eth as a different function)

    // It should check if it aggregates and is cross chain.

    // Third check which exchanges are being used, if there are any and if aggregate == true
    // Fourth check the chains to see what chains are being used if they are not the same, prepare stargate parameters and estimate gas fee off chain.
    // Fifth run the aggregator for the dst chain to get the necessary data and encode it
    //
    // Finally test all of this to make sure it is working properly.

    let mut steps: Vec<u8> = Vec::new();
    let mut data: Vec<Bytes> = Vec::new();

    // Change this to check to see if eth is in there and if it is not, change it to just encode the src tokens and amount
    // Turn this into its own function

    // =============================================================================================================================================================
    // =============================================================================================================================================================
    // ============================================                           Step 1 Clean Source Data                   ===========================================
    // =============================================================================================================================================================
    // =============================================================================================================================================================

    // let mut tokens: Vec<Address> = Vec::new();
    // let mut amounts: Vec<U256> = Vec::new();

    // for token in src_tokens.clone() {
    //     let mut index = 0;
    //     if token == constants::ETHEREUM.parse::<Address>().unwrap() {
    //         steps.push(2);
    //         let amount = AbiEncode::encode(src_amounts[index]);
    //         data.push(Bytes::from(amount));
    //     } else if token != constants::ETHEREUM.parse::<Address>().unwrap() {
    //         tokens.push(token);
    //         amounts.push(src_amounts[index]);
    //     }

    //     index += 1;
    // }
    // println!("{tokens:#?}, {amounts:#?}");
    // let bytes_tokens = Bytes::from(AbiEncode::encode(tokens.clone()));
    // let bytes_amounts = Bytes::from(AbiEncode::encode(amounts.clone()));

    // let d = [bytes_tokens, bytes_amounts].concat();
    // steps.push(1);
    // data.push(Bytes::from(d));

    // =============================================================================================================================================================
    // =============================================================================================================================================================
    // ===============================================                      Step 2 Verify Exchanges                    =============================================
    // =============================================================================================================================================================
    // =============================================================================================================================================================
    for exchange in exchanges {
            match exchange {
            "Uniswap" => exchanges::uniswap_swap(),
            "Camelot" => exchanges::camelot_swap(),
            "Sushi" => exchanges::sushi_swap(),
            "Xcal" => exchanges::xcal_swap(),
            _ => ()
        };
    }

    // =============================================================================================================================================================
    // =============================================================================================================================================================
    // ===============================================                      Step 3 Aggregate if True                    ============================================
    // =============================================================================================================================================================
    // =============================================================================================================================================================

    //Turn the repeat into its own function
     if aggregate == true {
        if complex == 1 && _src_chain == _dst_chain {
            let (_add_steps, _add_bytes) = single_to_single_aggregate(
                _src_chain,
                _dst_tokens[0].to_owned(),
                src_tokens[0].to_owned(),
                src_amounts[0],
            ).await;
            steps.extend(_add_steps);
            data.extend(_add_bytes);
        } else if complex == 2 && _src_chain == _dst_chain {
            let (_add_steps, _add_bytes) =
                _single_to_multi_aggregate(_src_chain, _dst_tokens, src_tokens[0].clone(), src_amounts)
                    .await;
        } else if complex == 3 && _src_chain == _dst_chain {
            
            let (_add_steps, _add_bytes) =
                _multi_to_single(_src_chain, _dst_tokens[0].clone(), src_tokens, src_amounts).await;
        } else if complex == 4 && _src_chain == _dst_chain {
        } else if complex == 1 && _src_chain != _dst_chain {
        } else if complex == 2 && _src_chain != _dst_chain {
        } else if complex == 3 && _src_chain != _dst_chain {
        } else if complex == 4 && _src_chain != _dst_chain {
        }

    }

    println!("{:#?}, {:#?}", steps, data);

    // =============================================================================================================================================================
    // =============================================================================================================================================================
    // ==============================================                     Step * Stargate Preperation                   ============================================
    // =============================================================================================================================================================
    // =============================================================================================================================================================

    // Need to also handle dst chain issues

    // let stargate_params = AbiEncode::encode(arbitrum_swaps::StargateParams {
    //     dst_chain_id: _dst_chain as u16,
    //     token: get_token_by_chain(_src_chain),
    //     src_pool_id: U256::from(1),
    //     dst_pool_id: U256::from(1),
    //     amount: U256::from(0),
    //     amount_min: U256::from(0),
    //     dust_amount: U256::from(0),
    //     receiver: SWAPS_ADDRESS.parse::<Address>().unwrap(),
    //     to: SWAPS_ADDRESS.parse::<Address>().unwrap(),
    //     gas: U256::from(0),
    //     src_context: [
    //         1, 2, 5, 3, 5, 7, 5, 3, 2, 3, 4, 6, 8, 5, 3, 5, 5, 7, 7, 4, 3, 4, 6, 6, 5, 4, 3, 4, 6,
    //         7, 4, 3,
    //     ],
    // });

    // steps.push(15);
    // data.push(Bytes::from(stargate_params));

    (Ok(()), steps, data)
}
