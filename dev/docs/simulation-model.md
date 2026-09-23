# Simulation model format

`simulation-model.hxm` is a template for a simulation model and
`simulation-component.hxc` is a template for a component. Validate model files
against `simulation-model.xsd` and component files against
`simulation-component.xsd`. The component schema includes the model schema, so
shared types are defined once.

Hrafnix model files use the `.hxm` extension and component files use `.hxc`.
The XML files in this directory are format examples; their names, values, and
wiring are illustrative only.

## Root elements

The root `model` element contains the following sections in order:

| Section | Required | Purpose |
| --- | --- | --- |
| `metadata` | No | Descriptive information such as `author` and `description`. |
| `model-settings` | Yes | Simulation-wide typed settings, such as time bounds and time step. |
| `parameters` | Yes | Frozen parameter values for the model. |
| `variables` | Yes | Frozen variable values for the model. |
| `background-decorations` | No | Decorations drawn behind components. |
| `foreground-decorations` | No | Decorations drawn in front of components. |
| `components` | Yes | Ordered built-in and file-backed instantiated components. |
| `connections` | Yes | Directed port-to-port connections between components. |
| `translations` | No | Translations of user-facing strings. |

The root `component` element contains the following sections in order:

| Section | Required | Purpose |
| --- | --- | --- |
| `metadata` | No | Descriptive information such as `author` and `description`. |
| `parameters` | Yes | Parameter definitions. |
| `variables` | Yes | Variable definitions. |
| `ports` | No | The component's ports. |
| `components` | Yes | Child components. Empty for a leaf component. |
| `connections` | Yes | Connections between child components. Empty for a leaf component. |
| `translations` | No | Translations of user-facing strings. |

Both roots share these attributes:

| Attribute | Type | Purpose |
| --- | --- | --- |
| `file-version` | `u16` | Format revision of the file. |
| `minimum-version` | semantic version | Minimum engine version able to load the file. |
| `recommended-version` | semantic version | Engine version the file was written for. |
| `version` | `u16` | Version of the model or component itself. |
| `name` | string | Display name. |
| `language` | language tag | Language of the file's user-facing strings, such as `en`. |

The component root also has required `width` and `height` `u16` attributes
that define the component's intrinsic size, and an optional `icon` attribute. A
relative icon value resolves through the project's conventional `data/icons`
directory (for example, `icon="heater.svg"` resolves to
`data/icons/heater.svg`).

## Settings

Each `setting` has a `key`, a `type`, and a `value`. Setting keys use the `g_`
prefix (for example, `g_time_step`), and `type` uses the same item types as
parameters and variables. Setting values are strings to preserve the datastore
representation.

## Parameters and variables

Parameter keys use the datastore `p_` prefix and variable keys use the
datastore `v_` prefix. Each item has a key that is unique within its object, a
`description`, and a type corresponding to an `ItemFrozen` variant.

- **Model and component definitions.** The root `parameters` and `variables`
  collections require an object `description`. In a component file they are
  datastore `ParameterObjectDefinition` and `VariableObjectDefinition` values;
  each item requires a `description` and may have a `default`.
- **Instantiated components.** A component instance in a `components` list
  stores `ParameterObjectFrozen` and `VariableObjectFrozen` values. The object
  `description` is optional and `variables` may be omitted. Scalar items store
  their current `value` and the `default` recorded when the instance was
  created.

`tab` and `separator` are structural item types and have neither a value nor a
default. File items also record `extension-filter` and `is-input`; folder items
record `is-input`; unit items record `unit-family`; and number-with-units items
record `preferred-unit` and current `units`. Not every item has a unit.

The XML currently represents boolean, file, folder, integer, number,
number-with-units, string, unit, tab, and separator datastore items. It rejects
choice, map, table, and table-with-units items explicitly because their
required datastore schemas cannot be represented faithfully. Integer and number
constraints are likewise rejected until the format gains constraint metadata.

## Components

The `components` collection is a single ordered list of file-backed
`component-file` and built-in `component` entries. Both share a `location`, an
optional `icon`, frozen `parameters` and `variables`, and `ports`. When
present, the icon is an `<icon path="..."/>` child immediately after the
component's `location`, so cached render data is self-contained.

A `component-file` has these attributes:

| Attribute | Required | Purpose |
| --- | --- | --- |
| `id` | Yes | Unique component ID. |
| `version` | Yes | Interface version of the source component. |
| `source-hash` | Yes | Hash of the source component, as `algorithm:digest`. |
| `path` | Yes | Path of the source `.hxc` file. |
| `package` | No | Package containing the source file. |

A built-in `component` has an `id`, an engine-provided `type`, and a behavioral
`version`.

Component IDs, decoration IDs, port IDs, and item keys must start with a
lowercase ASCII letter and otherwise contain only lowercase ASCII letters,
digits, or underscores (`[a-z][0-9a-z_]*`). Component IDs are globally unique,
decoration IDs are unique within the file, port IDs are unique within their
component, and item keys are unique within their object. The schema validates
these identities; a loader must additionally verify that each connection names
an existing port on its source and destination components.

Each component has a `location` with `x`, `y`, `width`, and `height` as `u16`
values (0 through 65,535). This is distinct from the component root's `width`
and `height`: the root defines the component's intrinsic size, and `location`
defines one instance's position and layout size. Each port declares its `type`
as `input`, `output`, or `bidirectional`, and a `location` with normalized
decimal `x` and `y` coordinates in the inclusive range `0.0` through `1.0`,
relative to the component.

Component files contain child components using the same `component-file` and
`component` entries as a model, so they can be opened independently. A
component instance uses only the interface of its source component; it does
not copy the source's own child components.

### Component versions

Component versions are single `u16` counters, not semantic versions; they
have no minor or patch parts. They have different compatibility roles. A
built-in component version increments for every behavior change so saved
models reproduce the same results. A file-based component version increments
only when its interface changes, including parameter or port additions,
removals, or other incompatible changes. Internal implementation changes that
preserve the interface are detected through `source-hash` and do not require a
component version increment.

### Source hash

`source-hash` has the form `algorithm:digest`. The only supported algorithm is
`blake3`, with a 64-character lowercase hexadecimal digest, for example
`blake3:1b6e…`. A loader compares this digest with the referenced source file
to determine whether the cached data is stale.

The digest is calculated from the canonical serialized bytes of the source
component file. Canonical serialization must define element order, attribute
order, whitespace, character encoding, and line endings, so formatting-only
changes do not invalidate a cached component.

## Cached component data

Each component instance stores a copy of its source component's interface:
parameters, variables, ports, layout, and icon. This lets a model load and
render when a source file is missing, and gives merges the data they need.

When the source file exists and its hash differs from `source-hash`, the loader
reconciles the copy with the source:

| Change in the source | Result |
| --- | --- |
| Item or port added | Added with the consumer's default. |
| Item or port removed or renamed | Dropped, along with connections to a dropped port. |
| Item type changed | The stored value is dropped and the new default is used. |
| Default changed | No effect. Defaults are recorded when an instance is created; the user re-enters the value to adopt a new default. |

When the source file is missing, the copy is kept unchanged, including when the
file is saved.

While a model is being edited, the editor periodically scans source files for
changes; otherwise sources are checked on load. Tolerance depends on the mode:

- **Editing:** missing or stale source files are allowed and the cached copy is
  used.
- **Simulating:** every source file must exist and match its cached copy.

## Packages

A project package contains `model`, `components`, and `data` folders. A TOML
manifest in the project declares its library dependencies. Downloaded
libraries are stored in the `.hrafnix` folder, one folder per version, named
`name@version`. Several versions of a library can be installed at the same
time.

`package` is either a package name (`hrafnix.sensors`) or a package name pinned
to a version (`hrafnix.sensors@1.0`). Without a version, the manifest decides
which version is used. A pinned version is used while migrating instances
between library versions. When `package` is omitted, `path` refers to a
component in the project's own package.

Library versions are semantic versions, separate from component versions, and
follow these rules:

| Change | Library version bump |
| --- | --- |
| Component added | Minor |
| Component interface changed (its component `version` is incremented) | Major |
| Component implementation changed, interface unchanged (its component `version` is unchanged; its `source-hash` changes) | Patch |

## Decorations

`background-decorations` are drawn behind components and
`foreground-decorations` in front of them. Each `decoration` has an `id`, a
`type`, a `location` in the same layout space as components, and an optional
`style`.

| Type | Content |
| --- | --- |
| `rectangle` | None. |
| `rounded-rectangle` | None. Uses `corner-radius`. |
| `frame` | None. Draws only the outline. |
| `text` | A `<text>` child with the text to display. |
| `image` | An `<image path="..." fit="..."/>` child. `fit` is `contain`, `cover`, `fill`, or `none`. |

`style` attributes are all optional: `fill-color`, `stroke-color`, and
`text-color` are `#RRGGBB` or `#RRGGBBAA` colors; `stroke-width`,
`corner-radius`, and `font-size` are non-negative decimals; and `font-weight`
is `normal` or `bold`. XSD 1.0 cannot require that a decoration's content
matches its type, so a loader must check it.

## Connections

A connection names its source and destination components and ports using
`from-component`, `from-port`, `to-component`, and `to-port`. The schema
requires connection endpoints to resolve to a component ID in the file. Its
required `style` is `solid`, `dashed`, or `dotted`.

Each connection has a `line` containing one or more `midpoint` elements.
Midpoints preserve the routed path between the port endpoints. Their `x` and
`y` values are absolute `u16` coordinates in the same layout space as component
locations; the line endpoints are derived from the connected ports.

## Translations

Translations are a lookup keyed by the source string. Each `translation` has
the source string as `key` and the source `language`, and contains one `item`
per target language with a `language` and a `value`. Keys are unique within a
file, and each target language appears at most once per translation.
