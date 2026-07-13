//! Versioned gas schedules (Goal 4) and a metered storage wrapper.
//!
//! Each schedule is frozen once activated; a gas change ships a new `GasParams`
//! constant and the version manager selects it at the activation fork.

use crate::primitives::{Error, Storage, U256};

#[derive(Clone, Copy, Debug)]
pub struct GasParams {
    pub read: u64,
    pub write: u64,
}

/// Frozen at Genesis.
pub const GAS_PARAMS_V1: GasParams = GasParams { read: 100, write: 2_000 };

/// Activated at ForkC: storage writes got more expensive.
pub const GAS_PARAMS_V2: GasParams = GasParams { read: 100, write: 5_000 };

/// Charges gas for every storage op using the fork-selected schedule, then
/// forwards to the underlying store.
pub struct MeteredStorage<'a> {
    inner: &'a mut dyn Storage,
    params: GasParams,
    gas_limit: u64,
    pub gas_used: u64,
}

impl<'a> MeteredStorage<'a> {
    pub fn new(inner: &'a mut dyn Storage, params: GasParams, gas_limit: u64) -> Self {
        Self { inner, params, gas_limit, gas_used: 0 }
    }

    fn charge(&mut self, amount: u64) -> Result<(), Error> {
        self.gas_used = self.gas_used.saturating_add(amount);
        if self.gas_used > self.gas_limit {
            Err(Error::OutOfGas)
        } else {
            Ok(())
        }
    }
}

impl Storage for MeteredStorage<'_> {
    fn read(&mut self, key: &[u8]) -> Result<U256, Error> {
        self.charge(self.params.read)?;
        self.inner.read(key)
    }

    fn write(&mut self, key: &[u8], value: U256) -> Result<(), Error> {
        self.charge(self.params.write)?;
        self.inner.write(key, value)
    }
}
