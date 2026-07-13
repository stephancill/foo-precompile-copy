//! Fork -> version routing.
//!
//! A single gate (`VersionManager::active`) maps the active hardfork to the
//! version that implements the precompile. Each version is a zero-sized unit
//! struct, so the returned `&'static dyn Foo` is just a pointer to a vtable —
//! no allocation. The dispatcher then calls trait methods on it directly.
//!
//! This trades a vtable lookup (dynamic dispatch) for a big simplification:
//! there is no per-version enum and no per-method match arms. Adding a version
//! is a one-line arm here.

use crate::gas::{GasParams, GAS_PARAMS_V1, GAS_PARAMS_V2};
use crate::logic::{Foo, FooV1, FooV2, FooV3, FooV4};
use crate::primitives::Hardfork;

pub struct VersionManager;

impl VersionManager {
    /// The gate: the FOO version active at `fork`.
    pub fn active(fork: Hardfork) -> &'static dyn Foo {
        match fork {
            Hardfork::Genesis | Hardfork::ForkA => &FooV1,
            Hardfork::ForkB => &FooV2,
            Hardfork::ForkC => &FooV3,
            Hardfork::ForkD => &FooV4,
        }
    }

    /// Gas schedule active at a fork (Goal 4). Frozen schedules never change.
    pub fn gas_params_for(fork: Hardfork) -> GasParams {
        match fork {
            Hardfork::Genesis | Hardfork::ForkA | Hardfork::ForkB => GAS_PARAMS_V1,
            Hardfork::ForkC | Hardfork::ForkD => GAS_PARAMS_V2,
        }
    }
}
