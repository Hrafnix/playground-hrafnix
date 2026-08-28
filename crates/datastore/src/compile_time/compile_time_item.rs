use crate::compile_time::{
    BooleanCompileTime, ChoiceCompileTime, FileCompileTime, FolderCompileTime, IntegerCompileTime,
    MapCompileTime, NumberCompileTime, NumberWithUnitsCompileTime, SeparatorCompileTime,
    StringCompileTime, TabCompileTime, TableCompileTime, TableWithUnitsCompileTime,
    UnitCompileTime,
};
use crate::definition::ItemDefinitionType;

/// Compile-time representation of a heterogeneous item. Use the `item_compile_time!` macro
/// to construct values; Rust enum variants remain public for matching.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemCompileTimeType {
    /// Boolean item variant.
    Boolean(BooleanCompileTime),
    /// Choice item variant.
    Choice(ChoiceCompileTime),
    /// File item variant.
    File(FileCompileTime),
    /// Folder item variant.
    Folder(FolderCompileTime),
    /// Integer item variant.
    Integer(IntegerCompileTime),
    /// Map item variant.
    Map(MapCompileTime),
    /// Number item variant.
    Number(NumberCompileTime),
    /// Number-with-units item variant.
    NumberWithUnits(NumberWithUnitsCompileTime),
    /// String item variant.
    String(StringCompileTime),
    /// Table item variant.
    Table(TableCompileTime),
    /// Table-with-units item variant.
    TableWithUnits(TableWithUnitsCompileTime),
    /// Unit item variant.
    Unit(UnitCompileTime),
    /// Tab item variant.
    Tab(TabCompileTime),
    /// Separator item variant.
    Separator(SeparatorCompileTime),
}

/// Helper macro for converting compile-time item types into `ItemCompileTimeType`.
macro_rules! item_from {
    ($type:ty, $variant:ident) => {
        impl From<$type> for ItemCompileTimeType {
            fn from(value: $type) -> Self {
                Self::$variant(value)
            }
        }
    };
}
item_from!(BooleanCompileTime, Boolean);
item_from!(ChoiceCompileTime, Choice);
item_from!(FileCompileTime, File);
item_from!(FolderCompileTime, Folder);
item_from!(IntegerCompileTime, Integer);
item_from!(MapCompileTime, Map);
item_from!(NumberCompileTime, Number);
item_from!(NumberWithUnitsCompileTime, NumberWithUnits);
item_from!(StringCompileTime, String);
item_from!(TableCompileTime, Table);
item_from!(TableWithUnitsCompileTime, TableWithUnits);
item_from!(UnitCompileTime, Unit);
item_from!(TabCompileTime, Tab);
item_from!(SeparatorCompileTime, Separator);

impl ItemCompileTimeType {
    /// Hidden wrapper for the `item_compile_time!(boolean = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`BooleanCompileTime`] (a `true`/`false` toggle) as an `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __boolean(value: BooleanCompileTime) -> Self {
        Self::Boolean(value)
    }
    /// Hidden wrapper for the `item_compile_time!(choice = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`ChoiceCompileTime`] (a single-select value chosen from a fixed list) as an
    /// `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __choice(value: ChoiceCompileTime) -> Self {
        Self::Choice(value)
    }
    /// Hidden wrapper for the `item_compile_time!(file = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`FileCompileTime`] (a file-picker parameter) as an `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __file(value: FileCompileTime) -> Self {
        Self::File(value)
    }
    /// Hidden wrapper for the `item_compile_time!(folder = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`FolderCompileTime`] (a folder-picker parameter) as an `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __folder(value: FolderCompileTime) -> Self {
        Self::Folder(value)
    }
    /// Hidden wrapper for the `item_compile_time!(integer = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps an [`IntegerCompileTime`] (an integer value with an optional constraint) as an
    /// `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __integer(value: IntegerCompileTime) -> Self {
        Self::Integer(value)
    }
    /// Hidden wrapper for the `item_compile_time!(map = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`MapCompileTime`] (a nested, dynamically keyed collection of map items) as
    /// an `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __map(value: MapCompileTime) -> Self {
        Self::Map(value)
    }
    /// Hidden wrapper for the `item_compile_time!(number = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`NumberCompileTime`] (an `f64` value with an optional constraint) as an
    /// `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __number(value: NumberCompileTime) -> Self {
        Self::Number(value)
    }
    /// Hidden wrapper for the `item_compile_time!(number_with_units = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`NumberWithUnitsCompileTime`] (an `f64` value with a preferred unit and an
    /// optional constraint) as an `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __number_with_units(value: NumberWithUnitsCompileTime) -> Self {
        Self::NumberWithUnits(value)
    }
    /// Hidden wrapper for the `item_compile_time!(string = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`StringCompileTime`] (a free-form text value) as an `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __string(value: StringCompileTime) -> Self {
        Self::String(value)
    }
    /// Hidden wrapper for the `item_compile_time!(table = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`TableCompileTime`] (a table of unit-less numeric columns) as an
    /// `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __table(value: TableCompileTime) -> Self {
        Self::Table(value)
    }
    /// Hidden wrapper for the `item_compile_time!(table_with_units = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`TableWithUnitsCompileTime`] (a table of numeric columns, each with its own
    /// preferred unit) as an `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __table_with_units(value: TableWithUnitsCompileTime) -> Self {
        Self::TableWithUnits(value)
    }
    /// Hidden wrapper for the `item_compile_time!(unit = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`UnitCompileTime`] (a value chosen from the units of a unit family) as an
    /// `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __unit(value: UnitCompileTime) -> Self {
        Self::Unit(value)
    }
    /// Hidden wrapper for the `item_compile_time!(tab = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`TabCompileTime`] (a layout-only tab heading; stores no value) as an
    /// `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __tab(value: TabCompileTime) -> Self {
        Self::Tab(value)
    }
    /// Hidden wrapper for the `item_compile_time!(separator = value)` arm.
    ///
    /// This is an implementation detail; call `item_compile_time!` instead.
    /// Wraps a [`SeparatorCompileTime`] (a layout-only visual divider; stores no value) as
    /// an `ItemCompileTimeType`.
    #[doc(hidden)]
    #[must_use]
    pub const fn __separator(value: SeparatorCompileTime) -> Self {
        Self::Separator(value)
    }

    /// Converts this compile-time item into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> ItemDefinitionType {
        match self {
            ItemCompileTimeType::Boolean(value) => {
                ItemDefinitionType::Boolean(value.into_definition())
            }
            ItemCompileTimeType::Choice(value) => {
                ItemDefinitionType::Choice(value.into_definition())
            }
            ItemCompileTimeType::File(value) => ItemDefinitionType::File(value.into_definition()),
            ItemCompileTimeType::Folder(value) => {
                ItemDefinitionType::Folder(value.into_definition())
            }
            ItemCompileTimeType::Integer(value) => {
                ItemDefinitionType::Integer(value.into_definition())
            }
            ItemCompileTimeType::Map(value) => ItemDefinitionType::Map(value.into_definition()),
            ItemCompileTimeType::Number(value) => {
                ItemDefinitionType::Number(value.into_definition())
            }
            ItemCompileTimeType::NumberWithUnits(value) => {
                ItemDefinitionType::NumberWithUnits(value.into_definition())
            }
            ItemCompileTimeType::String(value) => {
                ItemDefinitionType::String(value.into_definition())
            }
            ItemCompileTimeType::Table(value) => ItemDefinitionType::Table(value.into_definition()),
            ItemCompileTimeType::TableWithUnits(value) => {
                ItemDefinitionType::TableWithUnits(value.into_definition())
            }
            ItemCompileTimeType::Unit(value) => ItemDefinitionType::Unit(value.into_definition()),
            ItemCompileTimeType::Tab(value) => ItemDefinitionType::Tab(value.into_definition()),
            ItemCompileTimeType::Separator(value) => {
                ItemDefinitionType::Separator(value.into_definition())
            }
        }
    }
}

/// Wraps a compile-time value as an [`ItemCompileTimeType`] for use inside
/// `global_object_compile_time!`, `parameter_object_compile_time!`, and
/// `variable_object_compile_time!` item lists.
///
/// Expansion is wrapped in a `const` block, so `value` must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// item_compile_time!(boolean = value)
/// item_compile_time!(choice = value)
/// item_compile_time!(file = value)
/// item_compile_time!(folder = value)
/// item_compile_time!(integer = value)
/// item_compile_time!(map = value)
/// item_compile_time!(number = value)
/// item_compile_time!(number_with_units = value)
/// item_compile_time!(string = value)
/// item_compile_time!(table = value)
/// item_compile_time!(table_with_units = value)
/// item_compile_time!(unit = value)
/// item_compile_time!(tab = value)
/// item_compile_time!(separator = value)
/// ```
///
/// # Arguments
/// Each arm takes a single `value` of the matching compile-time type:
/// - `boolean`: [`BooleanCompileTime`] — a `true`/`false` toggle.
/// - `choice`: [`ChoiceCompileTime`] — a single-select value chosen from a fixed list.
/// - `file`: [`FileCompileTime`] — a file-picker parameter.
/// - `folder`: [`FolderCompileTime`] — a folder-picker parameter.
/// - `integer`: [`IntegerCompileTime`] — an integer value with an optional constraint.
/// - `map`: [`MapCompileTime`] — a nested, dynamically keyed collection of map items.
/// - `number`: [`NumberCompileTime`] — an `f64` value with an optional constraint.
/// - `number_with_units`: [`NumberWithUnitsCompileTime`] — an `f64` value with a preferred
///   unit and an optional constraint.
/// - `string`: [`StringCompileTime`] — a free-form text value.
/// - `table`: [`TableCompileTime`] — a table of unit-less numeric columns.
/// - `table_with_units`: [`TableWithUnitsCompileTime`] — a table of numeric columns, each
///   with its own preferred unit.
/// - `unit`: [`UnitCompileTime`] — a value chosen from the units of a unit family.
/// - `tab`: [`TabCompileTime`] — a layout-only tab heading; stores no value.
/// - `separator`: [`SeparatorCompileTime`] — a layout-only visual divider; stores no value.
///
/// # Examples
/// ```rust
/// use datastore::compile_time::ItemCompileTimeType;
/// use datastore::prelude::*;
///
/// const NAME: ItemCompileTimeType = item_compile_time!(string = string_compile_time!("Name"));
/// const READY: ItemCompileTimeType =
///     item_compile_time!(boolean = boolean_compile_time!("Ready", default = false));
/// let _definition = NAME.into_definition();
/// let _definition = READY.into_definition();
/// ```
#[macro_export]
macro_rules! item_compile_time {
    (boolean = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__boolean($value) }
    };
    (choice = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__choice($value) }
    };
    (file = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__file($value) }
    };
    (folder = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__folder($value) }
    };
    (integer = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__integer($value) }
    };
    (map = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__map($value) }
    };
    (number = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__number($value) }
    };
    (number_with_units = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__number_with_units($value) }
    };
    (string = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__string($value) }
    };
    (table = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__table($value) }
    };
    (table_with_units = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__table_with_units($value) }
    };
    (unit = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__unit($value) }
    };
    (tab = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__tab($value) }
    };
    (separator = $value:expr) => {
        const { $crate::compile_time::ItemCompileTimeType::__separator($value) }
    };
}
