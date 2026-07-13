//! Namespaced storage accessors. State evolves append-only (Goal 2): v3 adds a
//! brand-new `foo.frozen` namespace without touching or reinterpreting the
//! existing `foo.balances` namespace, so old logic reads exactly what it always
//! did.

use crate::primitives::{Address, Error, Storage, U256};

pub struct FooStorage;

impl FooStorage {
    fn balance_key(account: Address) -> Vec<u8> {
        let mut key = b"foo.balances/".to_vec();
        key.extend_from_slice(&account.0.to_be_bytes());
        key
    }

    // --- v3 addition: a new namespace, appended alongside the old one ---
    fn frozen_key(account: Address) -> Vec<u8> {
        let mut key = b"foo.frozen/".to_vec();
        key.extend_from_slice(&account.0.to_be_bytes());
        key
    }

    pub fn balance(storage: &mut dyn Storage, account: Address) -> Result<U256, Error> {
        storage.read(&Self::balance_key(account))
    }

    pub fn set_balance(storage: &mut dyn Storage, account: Address, value: U256) -> Result<(), Error> {
        storage.write(&Self::balance_key(account), value)
    }

    pub fn is_frozen(storage: &mut dyn Storage, account: Address) -> Result<bool, Error> {
        Ok(storage.read(&Self::frozen_key(account))? != 0)
    }

    pub fn set_frozen(storage: &mut dyn Storage, account: Address, frozen: bool) -> Result<(), Error> {
        storage.write(&Self::frozen_key(account), U256::from(frozen))
    }
}
