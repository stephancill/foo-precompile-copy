//! FOO v1 — activated at Genesis. FROZEN as of ForkB; do not modify.

use crate::logic::Foo;
use crate::primitives::{Address, Error, Storage, U256};
use crate::storage::FooStorage;

pub struct FooV1;

impl Foo for FooV1 {
    /// v1 semantics: a plain balance move. Sending to the zero address is
    /// permitted and acts as a burn.
    fn transfer(&self, storage: &mut dyn Storage, from: Address, to: Address, value: U256) -> Result<(), Error> {
        let from_balance = FooStorage::balance(storage, from)?;
        if from_balance < value {
            return Err(Error::InsufficientBalance);
        }
        FooStorage::set_balance(storage, from, from_balance - value)?;
        let to_balance = FooStorage::balance(storage, to)?;
        FooStorage::set_balance(storage, to, to_balance + value)?;
        Ok(())
    }

    fn balance_of(&self, storage: &mut dyn Storage, account: Address) -> Result<U256, Error> {
        FooStorage::balance(storage, account)
    }

    // mint + set_frozen inherit the trait defaults (unsupported).
}
