//! Business-logic interface for the FOO precompile plus its versioned impls.
//!
//! Newer methods carry a default that returns the pre-activation
//! "unsupported selector" error, so frozen versions (which do not override
//! them) reproduce exactly the behavior callers saw before the method existed
//! — same error type, same (zero state) gas.

use crate::abi::selector;
use crate::primitives::{Address, Error, Storage, U256};

pub trait Foo {
    // Present since v1.
    fn transfer(&self, storage: &mut dyn Storage, from: Address, to: Address, value: U256) -> Result<(), Error>;
    fn balance_of(&self, storage: &mut dyn Storage, account: Address) -> Result<U256, Error>;

    // Added in v2. Default = pre-activation behavior (inherited by v1).
    fn mint(&self, _storage: &mut dyn Storage, _to: Address, _value: U256) -> Result<(), Error> {
        Err(Error::UnknownFunctionSelector(selector::MINT))
    }

    // Added in v3. Default = pre-activation behavior (inherited by v1 and v2).
    fn set_frozen(&self, _storage: &mut dyn Storage, _account: Address, _frozen: bool) -> Result<(), Error> {
        Err(Error::UnknownFunctionSelector(selector::SET_FROZEN))
    }
}

mod v1;
pub use v1::FooV1;

mod v2;
pub use v2::FooV2;

mod v3;
pub use v3::FooV3;

mod v4;
pub use v4::FooV4;
