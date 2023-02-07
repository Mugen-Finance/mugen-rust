use ethers::prelude::*;

pub struct SrcTransferParams {
    token: Address,
    receiver: Address,
    amount: U256
}

pub async fn src_transfer(tokens: Vec<Address>, receiver: Address) -> (u8, Bytes) {
    let mut params: Vec<SrcTransferParams> = Vec::new();
    for token in tokens {
        let round = SrcTransferParams{token: token, receiver: receiver, amount: U256::zero()};
        params.push(round);
    }
    let step = 14;
    let encoded_data = AbiEncode::encode(params);
    let data = Bytes::from(encoded_data);
    returns (step, data);
}  