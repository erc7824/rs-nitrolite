use ethereum_types::{Address, U256};
use serde::{Deserialize, Serialize};

/// Represents a fund allocation in a state channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Allocation {
    /// Recipient of the funds
    pub destination: Address,
    /// ERC-20 token address (zero address for ETH)
    pub token: Address,
    /// Quantity of tokens allocated
    pub amount: U256,
}
