use ethers::prelude::*;
use eyre::Result;

//Create this file to encode the data

pub fn generate_binding(name: &str, path: &str, output: &str) -> Result<()> {
    Abigen::new(format!("{name}"), format!("{path}"))?
        .generate()?
        .write_to_file(format!("{output}"))?;
    Ok(())
}
