# Nitrolite Rust SDK

A Rust implementation of Nitrolite state channels.

## Roadmap

- [ ] Remove eip-191 for chain agnostic signatures
- [ ] Create helpers to convert hexadecimal string signature to type
- [ ] Create helpers for Channel to return ChannelId
- [ ] Create NitroRPC types and helpers

## Overview

This library provides Rust types and functionality for working with Nitrolite state channels

## Features

- State channel data structures
- Signature functionality for state authentication
- Ethereum-compatible types and serialization

## Usage

Add to your Cargo.toml:

```toml
[dependencies]
nitrolite = { git = "https://github.com/example/rs-nitrolite" }
```

Basic usage:

```rust
use ethereum_types::{Address, U256};
use nitrolite::{Allocation, Channel, State};

// Create a channel
let channel = Channel {
    participants: [Address::from_low_u64_be(1), Address::from_low_u64_be(2)],
    adjudicator: Address::random(),
    challenge: 3600,  // 1 hour challenge period
    nonce: 1,
};

// Create a state with allocations
let state = State {
    data: vec![1, 2, 3, 4],  // Application-specific data
    allocations: [
        Allocation {
            destination: channel.participants[0],
            token: Address::zero(),  // ETH
            amount: U256::from(800000000000000000u64),  // 0.8 ETH
        },
        Allocation {
            destination: channel.participants[1],
            token: Address::zero(),  // ETH
            amount: U256::from(200000000000000000u64),  // 0.2 ETH
        },
    ],
    sigs: vec![],  // No signatures yet
};
```

## License

MIT
