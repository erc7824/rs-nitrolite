use ethereum_types::Address;
use serde::{Deserialize, Serialize};

/// Represents the configuration of a state channel between two participants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Channel {
    /// Addresses of the two channel parties
    pub participants: [Address; 2],
    /// Address of the dispute resolution contract
    pub adjudicator: Address,
    /// Challenge period duration in seconds
    pub challenge: u64,
    /// Unique identifier for this channel
    pub nonce: u64,
}
