//! Platform stand-in types. In the real node these come from alloy / the EVM
//! state layer; here they are minimal so the example is self-contained.

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Address(pub u64);

/// Stand-in for alloy `U256`.
pub type U256 = u128;

/// 4-byte function selector, as in a real ABI.
pub type Selector = [u8; 4];

/// Ordered chain of hardforks. Newer forks compare greater.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Hardfork {
    Genesis,
    ForkA,
    ForkB,
    ForkC,
    ForkD,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Error {
    UnknownFunctionSelector(Selector),
    InsufficientBalance,
    TransferToZero,
    ZeroValueTransfer,
    AccountFrozen(Address),
    OutOfGas,
}

/// Raw key/value view of precompile state. The real impl is the EVM state trie.
/// `read` takes `&mut self` because a metered implementation charges gas on it.
pub trait Storage {
    fn read(&mut self, key: &[u8]) -> Result<U256, Error>;
    fn write(&mut self, key: &[u8], value: U256) -> Result<(), Error>;
}

/// Simple in-memory backing used by tests. Charges no gas.
#[derive(Default)]
pub struct InMemoryStorage {
    map: HashMap<Vec<u8>, U256>,
}

impl Storage for InMemoryStorage {
    fn read(&mut self, key: &[u8]) -> Result<U256, Error> {
        Ok(*self.map.get(key).unwrap_or(&0))
    }

    fn write(&mut self, key: &[u8], value: U256) -> Result<(), Error> {
        self.map.insert(key.to_vec(), value);
        Ok(())
    }
}
