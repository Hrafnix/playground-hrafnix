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

## License

Dual-licensed under MIT or Apache-2.0, whichever you prefer.
