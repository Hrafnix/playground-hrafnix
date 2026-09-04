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

## License

Dual-licensed under MIT or Apache-2.0, whichever you prefer.
