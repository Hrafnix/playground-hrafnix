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

## Component appearance

Custom-component documents use Hopsan-style appearance metadata. SVG paths are resolved relative
to the custom-component JSON file. The editor prefers the selected visual convention, falls back
to another available convention, and finally uses its missing-component icon.

```json
"appearance": {
  "icons": [
    {
      "icon_type": "user",
      "path": "icons/my-component.svg",
      "scale": 1.0,
      "rotate_with_component": true
    }
  ],
  "port_locations": {
    "in": { "x": 0.0, "y": 0.5, "angle": 180.0 },
    "out": { "x": 1.0, "y": 0.5, "angle": 0.0 }
  }
}
```

Both `user` and `iso` icon entries may be provided. Legacy embedded `icon_svg` values remain
readable and are used when no referenced SVG can be loaded.

Canvas ports follow Hopsan's built-in port-icon selection pattern. Scalar signals use read/write
circle icons, table signals use read/write 2D square icons, and each icon rotates by its port pose
angle. Domain-specific C/Q overlays and multiport overlays will apply once those port contracts are
represented by the simulation model.

## License

Dual-licensed under MIT or Apache-2.0, whichever you prefer.
