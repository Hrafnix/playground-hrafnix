# Simulation model format

`simulation-model.hxm` is a template for a simulation model.
Validate model files against `simulation-model.xsd`.

Hrafnix model files use the `.hxm` extension and component files use `.hrc`.
The XML files in this directory are format examples.

The root `model` element contains the following sections in order:

| Section | Purpose |
| --- | --- |
| `model-settings` | Simulation-wide key/value settings, such as time bounds and time step. |
| `parameters` | Frozen parameter values for the model. |
| `variables` | Frozen variable values for the model. |
| `components` | Ordered built-in and file-backed instantiated components. |
| `connections` | Directed port-to-port connections between components. |

Each component has a unique `id`. A file-backed `component-file` includes its
path, source hash, interface version, and optional package; a built-in
`component` includes its engine-provided `type` and behavioral version. Both
share frozen data, layout, an optional icon path, and ports. When present, the
icon is an `<icon path="..."/>` child immediately after the component's
`location`, so cached render data is self-contained. Models own `parameters`
and `variables`; instantiated components own frozen `parameters` and
`variables`.
Component IDs, port IDs, and item keys must start with a lowercase ASCII
letter and otherwise contain only lowercase ASCII letters, digits, or
underscores (`[a-z][0-9a-z_]*`). Component IDs are globally unique; port IDs
are unique within their component, and item keys are unique within their
object. The schema validates component, port, and item identities; a model
loader must additionally verify that each connection names an existing port
on its referenced source and destination components.

The `components` collection is a single ordered list of file-backed
`component-file` and built-in `component` entries. Each entry contains its full
frozen parameters, variables, ports, layout, and optional icon data required
to load and render the model without resolving its source component file.
Every `component-file` stores the source file's `source-blake3`: a
64-character lowercase hexadecimal BLAKE3 digest. A loader compares this
digest with the referenced source file to determine whether its cached data is
stale.

Component versions have different compatibility roles. A built-in component
version increments for every behavior change so saved models reproduce the
same results. A file-based simulation component version increments only when
its interface changes, including parameter or port additions, removals, or
other incompatible changes. Internal implementation changes that preserve the
interface are detected through `source-blake3` and do not require a component
version increment.

`file-version`, cached component version, and source component interface
version are `u16` counters. Engine compatibility fields such as
`minimum-version`, `recommended-version`, and `built-version` use documented
semantic-version strings.

`source-blake3` is calculated from the canonical serialized bytes of the
source component file. Canonical serialization must define element order,
attribute order, whitespace, character encoding, and line endings, so
formatting-only changes do not invalidate a cached component.

Each `parameters` and `variables` collection is a frozen datastore object and
must have a `description` attribute. Parameter keys use the datastore
`p_` prefix and variable keys use the datastore `v_` prefix. Each item has a
unique key within its object, a `description`, and a type corresponding to an
`ItemFrozen` variant. Scalar frozen items include their current `value` and
their definition's optional `default`; `tab` and `separator` are structural
item types and have neither. File items also record `extension-filter` and
`is-input`; folder items record `is-input`; unit items record `unit-family`;
and number-with-units items record `preferred-unit` and current `units`.
Each component has a
`location` with `x`, `y`, `width`, and `height` as `u16` values (0 through
65,535). Each port declares its `type` as `input`, `output`, or
`bidirectional`, and a `location` with normalized decimal `x` and `y`
coordinates in the inclusive range `0.0` through `1.0`.

Standalone component files declare datastore `ParameterObjectDefinition` and
`VariableObjectDefinition` values. Their `parameters` and `variables`
elements require an object `description`, and each definition item requires an
item `description` plus an optional `default` attribute. Their root
`<component>` has required `length` and `width` `u16`
attributes that define the source component's intrinsic size. These attributes
are distinct from an instantiated child component's required `<location
x="..." y="..." width="..." height="..."/>`, which defines that instance's
position and layout size. The source component's optional icon is an
`icon="..."` attribute on its root `<component>` metadata. A relative icon
value resolves through the project's conventional `data/icons` directory (for
example, `icon="heater.svg"` resolves to `data/icons/heater.svg`). In contrast,
each matching component in a model uses `ParameterObjectFrozen` and
`VariableObjectFrozen` values and serializes the current value with `value`.
Component files also contain child components using the same
`component-file` and `component` entries as a model, so they can be opened
independently without resolving child files. Component definitions and child
component lists each include `connections` for their direct child graph. A leaf
component has empty component and connection sections.

A connection names its source and destination components and ports using
`from-component`, `from-port`, `to-component`, and `to-port`. The schema
requires file references and connection endpoints to resolve to a unique
component ID in the component tree. Its required `style` is `solid`, `dashed`,
or `dotted`.

Each connection has a `line` containing one or more `midpoint` elements.
Midpoints preserve the routed path between the port endpoints. Their `x` and
`y` values are absolute `u16` coordinates in the same layout space as
component locations; the line endpoints are derived from the connected ports.

The XML currently represents boolean, file, folder, integer, number,
number-with-units, string, unit, tab, and separator datastore items. It
rejects choice, map, table, and table-with-units items explicitly because
their required datastore schemas cannot be represented faithfully. Integer and
number constraints are likewise rejected until the format gains constraint
metadata. Model setting values and scalar frozen-item values are strings to
preserve the datastore representation. The root identifies its format revision with
`file-version` (`u16`), minimum compatible engine version with
`minimum-version`, and the engine version that produced the model with
`built-version`; engine versions are semantic-version strings.
