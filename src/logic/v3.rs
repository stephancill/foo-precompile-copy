//! FOO v3 — activated at ForkC. FROZEN as of ForkD; do not modify.
//!
//! Self-contained copy of v2 with frozen-account checks and the new
//! `set_frozen` method added. All previously shipped logic is copied forward,
//! so this file fully describes v3's behavior on its own.

use crate::logic::Foo;
use crate::primitives::{Address, Error, Storage, U256};
use crate::storage::FooStorage;

pub struct FooV3;

impl Foo for FooV3 {
    /// Goal 1 — changed logic: block transfers involving a frozen account, then
    /// the v2 zero-address rule, then the v1 balance move — all copied here.
    fn transfer(&self, storage: &mut dyn Storage, from: Address, to: Address, value: U256) -> Result<(), Error> {
        // new in v3
        if FooStorage::is_frozen(storage, from)? {
            return Err(Error::AccountFrozen(from));
        }
        if FooStorage::is_frozen(storage, to)? {
            return Err(Error::AccountFrozen(to));
        }
        // --- copied from v2 ---
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

    /// Goal 2 + 3 — new state (`foo.frozen`) exposed through a new method.
    fn set_frozen(&self, storage: &mut dyn Storage, account: Address, frozen: bool) -> Result<(), Error> {
        FooStorage::set_frozen(storage, account, frozen)
    }

    // Copied verbatim from v2.
    fn mint(&self, storage: &mut dyn Storage, to: Address, value: U256) -> Result<(), Error> {
        let balance = FooStorage::balance(storage, to)?;
        FooStorage::set_balance(storage, to, balance + value)
    }

    // Copied verbatim from v1.
    fn balance_of(&self, storage: &mut dyn Storage, account: Address) -> Result<U256, Error> {
        FooStorage::balance(storage, account)
    }
}
