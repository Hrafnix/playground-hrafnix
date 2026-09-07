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

## Simulation model format

The versioned model template and its validating XSD are in
[`docs/simulation-model.hxm`](docs/simulation-model.hxm) and
[`docs/simulation-model.xsd`](docs/simulation-model.xsd). See
[`docs/simulation-model.md`](docs/simulation-model.md) for the section
semantics. The standalone component template and schema are
[`docs/simulation-component.hrc`](docs/simulation-component.hrc) and
[`docs/simulation-component.xsd`](docs/simulation-component.xsd).

## Project format

A Hrafnix project has the following top-level layout:

```text
.hrafnix/       Project metadata
components/     Reusable simulation component files
data/           Project data files
models/         Simulation model files
hrafnix.toml    Project manifest
```

The manifest's `project.format-version` identifies the project layout revision.

## License

Dual-licensed under MIT or Apache-2.0, whichever you prefer.
