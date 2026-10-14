# Hrafnix

[![Rust tests](https://github.com/Hrafnix/playground-hrafnix/actions/workflows/rust-test.yml/badge.svg?branch=main)](https://github.com/Hrafnix/playground-hrafnix/actions/workflows/rust-test.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/Hrafnix/playground-hrafnix#license)

Pronounced RAF-niks.

Hrafnix is an experimental Rust simulation engine.

## What's in here

It's a Cargo workspace split into eleven crates:

- `hrafnix` – the command-line application.
- `hrafnix_common` – shared utilities, such as floating-point helpers, used across the workspace.
- `hrafnix_shareable_string` – interned, thread-safe strings and translation maps.
- `hrafnix_keys` – strongly typed identifiers for stores, components, globals, parameters, variables, ports, and units.
- `hrafnix_message` – messages, object paths, and source spans used for diagnostics.
- `hrafnix_units` – unit definitions, conversions, and arithmetic.
- `hrafnix_datastore` – models hierarchical data as `CompileTime`, `Definition`, `Frozen`, and `Editable` values.
- `hrafnix_expression_engine` – converts datastore objects into inputs, evaluates their expressions, and returns computed data.
- `hrafnix_translation` – the application's built-in English translation catalog.
- `hrafnix_files` – project, model, and component files.
- `hrafnix_built_in_registry` – registry of the application's built-in components.

## Status

This is a personal playground project.

Don't expect much. This may not be stable anytime soon.

## Running the tests

```
cargo test --workspace
```

## Floating-point determinism

The target is identical finite `f64` result bits for identical inputs on the
x86-64 and AArch64 Linux, macOS, and Windows targets configured in
`.cargo/config.toml`. This is a tested compatibility goal, not a guarantee for
arbitrary architectures, compiler flags, or custom functions.

- Use the toolchain pinned in `rust-toolchain.toml` and `Cargo.lock` (`--locked`
  for reproducible builds). The minimum supported Rust version is checked
  separately; the beta workflow evaluates upcoming compiler changes. Review
  reference-test results before upgrading the pinned toolchain or `libm`.
- Keep the configured baseline CPU flags; do not replace them with
  `target-cpu=native`, fast-math options, or flags that permit reassociation.
  `RUSTFLAGS` and `CARGO_ENCODED_RUSTFLAGS` override Cargo's configured flags.
  The test workflow adds warning denial with a matching Cargo `cfg(all())`
  target configuration, which is combined with the target-triple flags.
- Expression transcendental functions, powers, and remainders use the pinned
  `libm` implementation. Ordinary expression multiplication and addition remain
  separate operations. Temperature conversions intentionally use `f64::mul_add`
  where written: the product and sum round once, whether implemented in hardware
  or software. Disabling hardware FMA does not change that API's fused semantics.
- Evaluation rejects non-finite results and normalizes negative zero to positive
  zero. Subnormals remain supported; they must not be flushed to zero.
- Each evaluation thread must use round-to-nearest, ties-to-even, with gradual
  underflow (no flush-to-zero or denormals-are-zero modes). Native libraries,
  plugins, and registered custom functions must preserve or restore that
  environment and follow the same math policy. The engine does not reset
  thread-local floating-point control registers.
- Preserve expression grouping and deterministic traversal/update order.
  Future reductions and parallel simulation updates must use a fixed ordering
  and reduction tree, not hash-map iteration or task-completion order. Numerical
  accuracy improvements alone do not establish deterministic results.

Fixed-bit reference tests exercise the evaluation pipeline and unit conversions,
including fused arithmetic, rounding boundaries, and subnormals. The same tests
run in debug and release across the six-platform test matrix; tolerance-based
accuracy tests are retained alongside them.

## License

Dual-licensed under MIT or Apache-2.0, whichever you prefer.
