use ethers::types::U256;
use eyre::Result;
use mugen_rust;

#[tokio::main]
async fn main() -> Result<()> {
    let mut src_tokens:Vec<String> = Vec::new();
    src_tokens.push("0x6B3595068778DD592e39A122f4f5a5cF09C90fE2".to_string());


    let mut src_amounts = Vec::new();
    src_amounts.push(U256::from("256000000000000000000"));


    let mut dst_tokens:Vec<String> = Vec::new();
    dst_tokens.push("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string());


    let src_chain = 101;

    let dst_chain = 101;

    let exchanges = Vec::new();


    let aggregate = true;

    let complex = 1;


    let (result, _steps, _data) = mugen_rust::run(
        src_tokens,
        src_amounts,
        dst_tokens,
        src_chain,
        dst_chain,
        exchanges,
        aggregate,
        complex,
    )
    .await;
    result
   
}
