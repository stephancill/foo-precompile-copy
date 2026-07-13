//! Entry point: decode calldata, resolve the fork-active version, meter gas,
//! then route to the selected implementation.

use crate::abi::{Abi, FooCall};
use crate::gas::MeteredStorage;
use crate::primitives::{Error, Hardfork, Selector, Storage, U256};
use crate::versions::VersionManager;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Output {
    Unit,
    Value(U256),
}

pub struct Dispatcher;

impl Dispatcher {
    /// Returns the call result plus gas consumed. Method-level fork gating is
    /// not special-cased here: it falls out of (fork -> version) resolution
    /// plus the frozen versions' "unsupported" trait defaults.
    pub fn dispatch(
        raw_storage: &mut dyn Storage,
        fork: Hardfork,
        gas_limit: u64,
        sel: Selector,
        args: &[U256],
    ) -> (Result<Output, Error>, u64) {
        // 1. Decode (the ABI recognizes every selector ever defined).
        let call = match Abi::decode(sel, args) {
            Ok(call) => call,
            Err(err) => return (Err(err), 0),
        };

        // 2. Resolve version + gas schedule from the fork (centralized).
        let active = VersionManager::active(fork);
        let params = VersionManager::gas_params_for(fork);
        let mut storage = MeteredStorage::new(raw_storage, params, gas_limit);

        // 3. Route.
        let result = match call {
            FooCall::Transfer { from, to, value } => {
                active.transfer(&mut storage, from, to, value).map(|()| Output::Unit)
            }
            FooCall::BalanceOf { account } => {
                active.balance_of(&mut storage, account).map(Output::Value)
            }
            FooCall::Mint { to, value } => {
                active.mint(&mut storage, to, value).map(|()| Output::Unit)
            }
            FooCall::SetFrozen { account, frozen } => {
                active.set_frozen(&mut storage, account, frozen).map(|()| Output::Unit)
            }
        };

        (result, storage.gas_used)
    }
}
