use ethers::prelude::{k256::ecdsa::SigningKey, *};
use std::sync::Arc;
mod arbitrum_swaps;

use crate::aggregator::arbitrum_swaps::arbitrum_swaps::*;

pub struct Aggregator {
    address: Address,
    aggregator: ArbitrumSwaps<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl Aggregator {
    pub fn new(
        middleware: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        address: Address,
    ) -> Self {
        let aggregator = ArbitrumSwaps::new(address, Arc::clone(&middleware));
        Self {
            address,
            aggregator,
        }
    }
}
