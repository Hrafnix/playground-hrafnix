# Hrafnix

Pronounced RAF-niks.

Hrafnix is an experimental Rust simulation engine.

## What's in here

It's a Cargo workspace split into seven crates:

- `shareable_string` – interned, thread-safe strings and translation maps.
- `keys` – strongly typed identifiers for stores, components, globals, parameters, variables, ports, and units.
- `message` – messages, object paths, and source spans used for diagnostics.
- `units` – unit definitions, conversions, and arithmetic.
- `datastore` – models hierarchical data as `CompileTime`, `Definition`, `Frozen`, and `Editable` values.
- `expression_engine` – converts datastore objects into inputs, evaluates their expressions, and returns computed data.
- `translation` – the application's built-in English translation catalog.

## Status

This is a personal playground project.

Don't expect much. This may not be stable anytime soon.

## Running the tests

```
cargo test --workspace
```

## Floating-point determinism guardrails

- Rust is pinned through `/home/runner/work/playground-hrafnix/playground-hrafnix/rust-toolchain.toml`, and CI installs that exact toolchain instead of updating to latest.
- `/home/runner/work/playground-hrafnix/playground-hrafnix/.cargo/config.toml` keeps supported targets on explicit baseline CPUs instead of host-tuned code generation.
- Expression evaluation and unit conversion normalize `-0.0` to `0.0` and reject non-finite values before they are stored as computed output.
- Determinism-sensitive accumulation should stay on ordered collections and stable iteration order.
- The release profile is the authoritative profile for determinism-sensitive behavior; debug builds remain useful for development and diagnostics.

## License

Dual-licensed under MIT or Apache-2.0, whichever you prefer.
