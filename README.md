# Playground Calculator

A personal project for messing around with how to build a data-driven expression engine in Rust.

Data is stored in a shallow tree (one to two levels deep) of objects, which can be frozen (immutable) or editable (mutable). 
The frozen objects can be evaluated into expressions, which can then be evaluated to produce a value.

## What's in here

It's a Cargo workspace, split into three crates:

- `shareable_string` – interned, thread-safe strings (plus translations), used throughout the other crates so the same string isn't copied around everywhere.
- `datastore` – a hierarchical, observable data store. 
  - Data is described with `Definition`s (basic values, structs, maps, tables).
  - Data is stored in `FrozenObject`s (immutable) and edited through `EditableObject`s (mutable, observable).
- `expression_engine` – converts the `FrozenObject`s into evaluable expressions, with support for tiers (globals, parameters, variables) and custom functions.

## Status

This is a personal playground project.

Don't expect much. This may not be stable anytime soon.

## Running the tests

```
cargo test --workspace
```

## License

Dual-licensed under MIT or Apache-2.0, whichever you prefer.
