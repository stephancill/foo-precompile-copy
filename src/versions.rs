//! Centralized version resolution (Goal: single source of truth for routing).
//!
//! `VersionManager` maps a hardfork to the active FOO version and gas schedule.
//! `ActiveFoo` is an enum wrapper that dispatches statically (no `dyn`), honoring
//! the design constraint to keep the hot execution path free of dynamic dispatch.

use crate::gas::{GasParams, GAS_PARAMS_V1, GAS_PARAMS_V2};
use crate::logic::{Foo, FooV1, FooV2, FooV3, FooV4};
use crate::primitives::{Address, Error, Hardfork, Storage, U256};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Version {
    V1,
    V2,
    V3,
    V4,
}

pub struct VersionManager;

impl VersionManager {
    /// The single source of truth mapping a hardfork to the active FOO version.
    pub fn version_for(fork: Hardfork) -> Version {
        match fork {
            Hardfork::Genesis | Hardfork::ForkA => Version::V1,
            Hardfork::ForkB => Version::V2,
            Hardfork::ForkC => Version::V3,
            Hardfork::ForkD => Version::V4,
        }
    }

    /// Gas schedule active at a fork (Goal 4). Frozen schedules never change.
    pub fn gas_params_for(fork: Hardfork) -> GasParams {
        match fork {
            Hardfork::Genesis | Hardfork::ForkA | Hardfork::ForkB => GAS_PARAMS_V1,
            // v4 spans ForkD without a gas change: same frozen schedule as ForkC.
            Hardfork::ForkC | Hardfork::ForkD => GAS_PARAMS_V2,
        }
    }
}

/// Statically-dispatched handle to whichever version is active for a fork.
pub enum ActiveFoo {
    V1(FooV1),
    V2(FooV2),
    V3(FooV3),
    V4(FooV4),
}

impl ActiveFoo {
    pub fn resolve(fork: Hardfork) -> Self {
        match VersionManager::version_for(fork) {
            Version::V1 => ActiveFoo::V1(FooV1),
            Version::V2 => ActiveFoo::V2(FooV2),
            Version::V3 => ActiveFoo::V3(FooV3),
            Version::V4 => ActiveFoo::V4(FooV4),
        }
    }
}

impl Foo for ActiveFoo {
    fn transfer(&self, storage: &mut dyn Storage, from: Address, to: Address, value: U256) -> Result<(), Error> {
        match self {
            ActiveFoo::V1(f) => f.transfer(storage, from, to, value),
            ActiveFoo::V2(f) => f.transfer(storage, from, to, value),
            ActiveFoo::V3(f) => f.transfer(storage, from, to, value),
            ActiveFoo::V4(f) => f.transfer(storage, from, to, value),
        }
    }

    fn balance_of(&self, storage: &mut dyn Storage, account: Address) -> Result<U256, Error> {
        match self {
            ActiveFoo::V1(f) => f.balance_of(storage, account),
            ActiveFoo::V2(f) => f.balance_of(storage, account),
            ActiveFoo::V3(f) => f.balance_of(storage, account),
            ActiveFoo::V4(f) => f.balance_of(storage, account),
        }
    }

    fn mint(&self, storage: &mut dyn Storage, to: Address, value: U256) -> Result<(), Error> {
        match self {
            ActiveFoo::V1(f) => f.mint(storage, to, value),
            ActiveFoo::V2(f) => f.mint(storage, to, value),
            ActiveFoo::V3(f) => f.mint(storage, to, value),
            ActiveFoo::V4(f) => f.mint(storage, to, value),
        }
    }

    fn set_frozen(&self, storage: &mut dyn Storage, account: Address, frozen: bool) -> Result<(), Error> {
        match self {
            ActiveFoo::V1(f) => f.set_frozen(storage, account, frozen),
            ActiveFoo::V2(f) => f.set_frozen(storage, account, frozen),
            ActiveFoo::V3(f) => f.set_frozen(storage, account, frozen),
            ActiveFoo::V4(f) => f.set_frozen(storage, account, frozen),
        }
    }
}
