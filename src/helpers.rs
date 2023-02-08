use ethers::prelude::*;
use eyre::Result;

use crate::constants::USDC;

//Create this file to encode the data

pub fn generate_binding(name: &str, path: &str, output: &str) -> Result<()> {
    Abigen::new(format!("{name}"), format!("{path}"))?
        .generate()?
        .write_to_file(format!("{output}"))?;
    Ok(())
}

pub fn get_token_by_chain(chain: u64) -> H160 {
    return USDC.parse::<Address>().unwrap();
}
