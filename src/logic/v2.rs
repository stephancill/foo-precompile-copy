//! FOO v2 — activated at ForkB. FROZEN as of ForkC; do not modify.
//!
//! Self-contained copy of v1 with the zero-address rule added. There is no link
//! back to v1: every method v2 supports is spelled out in full here, so this
//! file alone is the complete source of truth for v2's behavior.

use crate::logic::Foo;
use crate::primitives::{Address, Error, Storage, U256};
use crate::storage::FooStorage;

pub struct FooV2;

impl Foo for FooV2 {
    /// Goal 1 — changed logic: reject transfers to the zero address, then run
    /// the balance move (copied verbatim from v1).
    fn transfer(&self, storage: &mut dyn Storage, from: Address, to: Address, value: U256) -> Result<(), Error> {
        if to == Address(0) {
            return Err(Error::TransferToZero);
        }
        // --- copied from v1 ---
        let from_balance = FooStorage::balance(storage, from)?;
        if from_balance < value {
            return Err(Error::InsufficientBalance);
        }
        FooStorage::set_balance(storage, from, from_balance - value)?;
        let to_balance = FooStorage::balance(storage, to)?;
        FooStorage::set_balance(storage, to, to_balance + value)?;
        Ok(())
    }

    /// Goal 3 — new method introduced in v2.
    fn mint(&self, storage: &mut dyn Storage, to: Address, value: U256) -> Result<(), Error> {
        let balance = FooStorage::balance(storage, to)?;
        FooStorage::set_balance(storage, to, balance + value)
    }

    // Copied verbatim from v1.
    fn balance_of(&self, storage: &mut dyn Storage, account: Address) -> Result<U256, Error> {
        FooStorage::balance(storage, account)
    }

    // set_frozen inherits the trait default (unsupported) until v3.
}
