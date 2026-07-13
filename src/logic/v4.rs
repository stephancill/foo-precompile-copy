//! FOO v4 — activated at ForkD. Current tip.
//!
//! Self-contained copy of v3 with a zero-value guard added. Every check and the
//! balance move from v3/v2/v1 are copied here verbatim, so this file is the
//! complete, standalone description of v4's behavior.

use crate::logic::Foo;
use crate::primitives::{Address, Error, Storage, U256};
use crate::storage::FooStorage;

pub struct FooV4;

impl Foo for FooV4 {
    /// Goal 1 — reject zero-value transfers, then the v3 frozen checks, the v2
    /// zero-address rule, and the v1 balance move — all copied here.
    fn transfer(&self, storage: &mut dyn Storage, from: Address, to: Address, value: U256) -> Result<(), Error> {
        // new in v4
        if value == 0 {
            return Err(Error::ZeroValueTransfer);
        }
        // --- copied from v3 ---
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

    // Copied verbatim from v3.
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
