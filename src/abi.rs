//! Append-only ABI: selectors, decoded call shapes, and calldata decoding.
//!
//! Every selector ever shipped stays decodable forever. Fork gating does NOT
//! happen here — it happens later, during version resolution, so that a
//! pre-activation call decodes fine and then hits the frozen version's
//! "unsupported" default.

use crate::primitives::{Address, Error, Selector, U256};

pub mod selector {
    use crate::primitives::Selector;

    pub const TRANSFER: Selector = [0, 0, 0, 1];
    pub const BALANCE_OF: Selector = [0, 0, 0, 2];
    pub const MINT: Selector = [0, 0, 0, 3]; // added in v2
    pub const SET_FROZEN: Selector = [0, 0, 0, 4]; // added in v3
}

#[derive(Clone, Copy, Debug)]
pub enum FooCall {
    Transfer { from: Address, to: Address, value: U256 },
    BalanceOf { account: Address },
    Mint { to: Address, value: U256 },
    SetFrozen { account: Address, frozen: bool },
}

pub struct Abi;

impl Abi {
    /// Decode a selector + argument words into a typed call. Args are simplified
    /// to a slice of words; a real ABI would parse packed calldata and validate
    /// lengths.
    pub fn decode(sel: Selector, args: &[U256]) -> Result<FooCall, Error> {
        match sel {
            selector::TRANSFER => Ok(FooCall::Transfer {
                from: Address(args[0] as u64),
                to: Address(args[1] as u64),
                value: args[2],
            }),
            selector::BALANCE_OF => Ok(FooCall::BalanceOf {
                account: Address(args[0] as u64),
            }),
            selector::MINT => Ok(FooCall::Mint {
                to: Address(args[0] as u64),
                value: args[1],
            }),
            selector::SET_FROZEN => Ok(FooCall::SetFrozen {
                account: Address(args[0] as u64),
                frozen: args[1] != 0,
            }),
            _ => Err(Error::UnknownFunctionSelector(sel)),
        }
    }
}
