use ethers::prelude::*;
use weth::*;

pub struct DepositData {
    tokens: Address,
    amount: U256
}

pub async fn deposit(params: Vec<DepositData>) {
    // add in case for native token
   let step = 1;
   let encoded_data = AbiEncode::encode(params);
   let param = Bytes::from(encoded_data);
}