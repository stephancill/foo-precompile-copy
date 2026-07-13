//! Golden / replay tests: prove each fork executes its own version and that
//! historical forks keep replaying identically after newer versions ship.

use foo_precompile_copy::{
    selector, Address, Dispatcher, Error, FooStorage, Hardfork, InMemoryStorage, Output,
};

const GAS: u64 = 1_000_000;

fn seeded(account: u64, balance: u128) -> InMemoryStorage {
    let mut storage = InMemoryStorage::default();
    FooStorage::set_balance(&mut storage, Address(account), balance).unwrap();
    storage
}

// Goal 1: changing existing logic. Transfer-to-zero is a burn under v1 but is
// rejected from ForkB onward. The two forks must disagree on the same input.
#[test]
fn goal1_transfer_to_zero_changes_at_forkb() {
    // ForkA (v1): allowed, balance is burned.
    let mut s = seeded(1, 100);
    let (r, _) = Dispatcher::dispatch(&mut s, Hardfork::ForkA, GAS, selector::TRANSFER, &[1, 0, 40]);
    assert_eq!(r, Ok(Output::Unit));
    assert_eq!(FooStorage::balance(&mut s, Address(1)).unwrap(), 60);

    // ForkB (v2): same call now reverts and state is untouched.
    let mut s = seeded(1, 100);
    let (r, _) = Dispatcher::dispatch(&mut s, Hardfork::ForkB, GAS, selector::TRANSFER, &[1, 0, 40]);
    assert_eq!(r, Err(Error::TransferToZero));
    assert_eq!(FooStorage::balance(&mut s, Address(1)).unwrap(), 100);
}

// Goal 3: new method. Pre-activation the selector must behave exactly as it did
// before it existed (unknown selector, no state gas); post-activation it works.
#[test]
fn goal3_mint_unsupported_before_forkb() {
    let mut s = InMemoryStorage::default();

    let (r, gas) = Dispatcher::dispatch(&mut s, Hardfork::ForkA, GAS, selector::MINT, &[7, 50]);
    assert_eq!(r, Err(Error::UnknownFunctionSelector(selector::MINT)));
    assert_eq!(gas, 0, "pre-activation call must not touch state");

    let (r, _) = Dispatcher::dispatch(&mut s, Hardfork::ForkB, GAS, selector::MINT, &[7, 50]);
    assert_eq!(r, Ok(Output::Unit));
    assert_eq!(FooStorage::balance(&mut s, Address(7)).unwrap(), 50);
}

// Goal 2 + 3: new append-only state + new method. Freezing exists only from
// ForkC; the new `foo.frozen` namespace is invisible to older logic (v2), which
// keeps transferring as before.
#[test]
fn goal2_new_state_and_freeze_at_forkc() {
    let mut s = seeded(1, 100);

    // set_frozen is unsupported before ForkC.
    let (r, _) = Dispatcher::dispatch(&mut s, Hardfork::ForkB, GAS, selector::SET_FROZEN, &[1, 1]);
    assert_eq!(r, Err(Error::UnknownFunctionSelector(selector::SET_FROZEN)));

    // ForkC: freeze account 1, then its transfer is blocked.
    let (r, _) = Dispatcher::dispatch(&mut s, Hardfork::ForkC, GAS, selector::SET_FROZEN, &[1, 1]);
    assert_eq!(r, Ok(Output::Unit));
    let (r, _) = Dispatcher::dispatch(&mut s, Hardfork::ForkC, GAS, selector::TRANSFER, &[1, 2, 10]);
    assert_eq!(r, Err(Error::AccountFrozen(Address(1))));

    // The same transfer at ForkB (v2) has no concept of frozen -> it succeeds,
    // proving the new namespace never altered old behavior.
    let (r, _) = Dispatcher::dispatch(&mut s, Hardfork::ForkB, GAS, selector::TRANSFER, &[1, 2, 10]);
    assert_eq!(r, Ok(Output::Unit));
    assert_eq!(FooStorage::balance(&mut s, Address(2)).unwrap(), 10);
}

// Goal 4: gas schedule is versioned. The same transfer costs more at ForkC.
#[test]
fn goal4_gas_schedule_versioned() {
    let mut s = seeded(1, 100);
    let (_, gas_b) = Dispatcher::dispatch(&mut s, Hardfork::ForkB, GAS, selector::TRANSFER, &[1, 2, 10]);

    let mut s = seeded(1, 100);
    let (_, gas_c) = Dispatcher::dispatch(&mut s, Hardfork::ForkC, GAS, selector::TRANSFER, &[1, 2, 10]);

    assert!(gas_c > gas_b, "ForkC writes are pricier + v3 reads frozen flags: gas_c={gas_c} gas_b={gas_b}");
}

// The whole point: a historical ForkA block replays under v1 semantics even
// though v2 and v3 now exist in the binary.
#[test]
fn replay_of_old_fork_is_deterministic() {
    let mut s = seeded(1, 100);
    // to == zero, which only v1 permits.
    let (r, _) = Dispatcher::dispatch(&mut s, Hardfork::ForkA, GAS, selector::TRANSFER, &[1, 0, 40]);
    assert_eq!(r, Ok(Output::Unit));
    assert_eq!(FooStorage::balance(&mut s, Address(1)).unwrap(), 60);
}
