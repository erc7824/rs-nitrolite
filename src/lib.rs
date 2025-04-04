//! Nitrolite implementation in Rust
//! 
//! Provides state channel functionality compatible with Nitrolite protocol

mod allocation;
mod channel;
mod error;
mod signature;
mod state;

pub use allocation::Allocation;
pub use channel::Channel;
pub use error::NitroliteError;
pub use signature::Signature;
pub use state::State;

/// Result type for Nitrolite operations
pub type Result<T> = std::result::Result<T, NitroliteError>;

#[cfg(test)]
mod tests {
    use super::*;
    use ethereum_types::{Address, U256};

    #[test]
    fn test_create_channel() {
        let channel = Channel {
            participants: [Address::zero(), Address::random()],
            adjudicator: Address::random(),
            challenge: 3600,
            nonce: 1,
        };
        
        assert_eq!(channel.challenge, 3600);
        assert_eq!(channel.nonce, 1);
    }

    #[test]
    fn test_create_allocation() {
        let allocation = Allocation {
            destination: Address::random(),
            token: Address::zero(), // ETH
            amount: U256::from(1000000000000000000u64), // 1 ETH
        };
        
        assert_eq!(allocation.token, Address::zero());
        assert!(allocation.amount > U256::zero());
    }

    #[test]
    fn test_create_state() {
        let alice = Address::random();
        let bob = Address::random();
        
        let state = State {
            data: vec![1, 2, 3, 4],
            allocations: [
                Allocation {
                    destination: alice,
                    token: Address::zero(),
                    amount: U256::from(800000000000000000u64), // 0.8 ETH
                },
                Allocation {
                    destination: bob,
                    token: Address::zero(),
                    amount: U256::from(200000000000000000u64), // 0.2 ETH
                },
            ],
            sigs: vec![],
        };
        
        assert_eq!(state.data, vec![1, 2, 3, 4]);
        assert_eq!(state.allocations.len(), 2);
        assert_eq!(state.allocations[0].destination, alice);
        assert_eq!(state.allocations[1].destination, bob);
    }
}