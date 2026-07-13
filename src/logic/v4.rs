//! FOO v4 — activated at ForkD. Current tip.
//!
//! Introduces a NEW method (`burn`). As a self-contained copy, v4 restates all
//! four existing methods verbatim from v3 and adds `burn`. Beyond this file, a
//! new method also touches the ABI, the `Foo` trait, the dispatcher, and the
//! `ActiveFoo` router.

use crate::logic::Foo;
use crate::primitives::{Address, Error, Storage, U256};
use crate::storage::FooStorage;

pub struct FooV4;

impl Foo for FooV4 {
    /// Goal 3 — new method: burn reduces an account's balance.
    fn burn(&self, storage: &mut dyn Storage, from: Address, value: U256) -> Result<(), Error> {
        let balance = FooStorage::balance(storage, from)?;
        if balance < value {
            return Err(Error::InsufficientBalance);
        }
        FooStorage::set_balance(storage, from, balance - value)
    }

    // --- everything below copied verbatim from v3 ---

    fn transfer(&self, storage: &mut dyn Storage, from: Address, to: Address, value: U256) -> Result<(), Error> {
        if FooStorage::is_frozen(storage, from)? {
            return Err(Error::AccountFrozen(from));
        }
        if FooStorage::is_frozen(storage, to)? {
            return Err(Error::AccountFrozen(to));
        }
        if to == Address(0) {
            return Err(Error::TransferToZero);
        }
        let from_balance = FooStorage::balance(storage, from)?;
        if from_balance < value {
            return Err(Error::InsufficientBalance);
        }
        FooStorage::set_balance(storage, from, from_balance - value)?;
        let to_balance = FooStorage::balance(storage, to)?;
        FooStorage::set_balance(storage, to, to_balance + value)?;
        Ok(())
    }

    fn set_frozen(&self, storage: &mut dyn Storage, account: Address, frozen: bool) -> Result<(), Error> {
        FooStorage::set_frozen(storage, account, frozen)
    }

    fn mint(&self, storage: &mut dyn Storage, to: Address, value: U256) -> Result<(), Error> {
        let balance = FooStorage::balance(storage, to)?;
        FooStorage::set_balance(storage, to, balance + value)
    }

    fn balance_of(&self, storage: &mut dyn Storage, account: Address) -> Result<U256, Error> {
        FooStorage::balance(storage, account)
    }
}
