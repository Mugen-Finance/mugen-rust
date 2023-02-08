use eyre::Result;
use mugen_rust;

#[tokio::main]
async fn main() -> Result<()> {
    let src_tokens = Vec::new();
    let src_amounts = Vec::new();
    let dst_tokens = Vec::new();
    let src_chain = 1;
    let dst_chain = 112;
    let exchanges = Vec::new();
    let aggregate = true;
    let complex = 4;
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
