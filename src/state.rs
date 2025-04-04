use serde::{Deserialize, Serialize};
use crate::{Allocation, Signature};

/// Represents the current state of a channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    /// Arbitrary channel state information
    pub data: Vec<u8>,
    /// Fund distribution between participants
    pub allocations: [Allocation; 2],
    /// Signatures from channel participants
    pub sigs: Vec<Signature>,
}
