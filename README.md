# FOO precompile — versioned evolution demo (copy-fork)

A self-contained, runnable illustration of versioning a precompile so that logic,
state, methods and gas can change over hardforks **without** ever mutating
previously activated behavior — historical blocks replay identically.

In this variant each version is a **standalone, self-contained implementation**:
when behavior changes, the previous version's file is copied forward and edited
in place. Unchanged logic is duplicated rather than referenced, so every
version's file is the complete source of truth for that version's behavior and
no version depends on another at runtime.

`FOO` is a fake ERC20-ish precompile that evolves over three versions.

## Fork / version timeline

| Fork      | Active version | What changed at this fork |
|-----------|----------------|---------------------------|
| Genesis   | `FooV1`        | Base: `transfer`, `balance_of`. Transfer to the zero address is a burn. |
| ForkA     | `FooV1`        | (a version may span forks) |
| ForkB     | `FooV2`        | **Goal 1** `transfer` now rejects the zero address. **Goal 3** new `mint` method. |
| ForkC     | `FooV3`        | **Goal 1** `transfer` blocks frozen accounts. **Goal 2** new `foo.frozen` state. **Goal 3** new `set_frozen` method. **Goal 4** new gas schedule. |

## File map

```text
src/
  primitives.rs   platform stand-ins (Address, U256, Storage, Error, Hardfork)
  abi.rs          append-only selectors + calldata decoding
  storage.rs      namespaced state accessors (append-only)
  gas.rs          versioned GasParams + metered storage       (Goal 4)
  logic/
    mod.rs        Foo business-logic trait (new methods default to "unsupported")
    v1.rs         FooV1  (frozen, full impl)
    v2.rs         FooV2  (frozen, full copy of v1 + changes)
    v3.rs         FooV3  (tip, full copy of v2 + changes)
  versions.rs     VersionManager (fork -> version + gas) and ActiveFoo router
  dispatcher.rs   decode -> resolve version -> meter gas -> route
tests/
  replay.rs       golden / replay tests for every goal
```

## The core mechanic: a full copy per version

Each version is a plain struct that implements every method it supports with the
whole body written out. Adding a version means copying the previous file and
editing the parts that change:

```rust,ignore
pub struct FooV2;

impl Foo for FooV2 {
    fn transfer(..) {
        if to == Address(0) { return Err(Error::TransferToZero); } // new in v2
        // ...balance move copied verbatim from v1...
    }
    fn balance_of(..) { /* copied verbatim from v1 */ }
    fn mint(..) { /* brand new */ }
}
```

Because nothing references an earlier version, each frozen file fully captures
its own behavior (a per-version source hash is exact) and can never be affected
by a later edit. The cost is duplication: unchanged logic is repeated in every
version, and adding a version produces a large diff (a whole new file). Method
fork gating still falls out for free: a selector added in v2 is not implemented
by v1, so a pre-ForkB block resolves to v1 and the trait default returns the
original "unknown selector" error.

## Run it

```sh
cargo test
```

## Notes

Types (`Address`, `U256`, `Storage`) are simplified stand-ins for alloy / the EVM
state layer so the crate is dependency-free.
