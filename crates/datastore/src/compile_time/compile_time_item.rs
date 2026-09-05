use crate::compile_time::{
    BooleanCompileTime, ChoiceCompileTime, FileCompileTime, FolderCompileTime, IntegerCompileTime,
    MapCompileTime, NumberCompileTime, NumberWithUnitsCompileTime, SeparatorCompileTime,
    StringCompileTime, TabCompileTime, TableCompileTime, TableWithUnitsCompileTime,
    UnitCompileTime,
};
use crate::definition::ItemDefinitionType;

/// Compile-time representation of a heterogeneous item. Use the `const_item!` macro
/// to construct values; Rust enum variants remain public for matching.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemCompileTime {
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

/// Helper macro for converting compile-time item types into [`ItemCompileTime`].
macro_rules! item_from {
    ($type:ty, $variant:ident) => {
        impl From<$type> for ItemCompileTime {
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

impl ItemCompileTime {
    /// Hidden wrapper for the `const_item!(boolean = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`BooleanCompileTime`] (a `true`/`false` toggle) as an [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __boolean(value: BooleanCompileTime) -> Self {
        Self::Boolean(value)
    }

    /// Hidden wrapper for the `const_item!(choice = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`ChoiceCompileTime`] (a single-select value chosen from a fixed list) as an
    /// [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __choice(value: ChoiceCompileTime) -> Self {
        Self::Choice(value)
    }

    /// Hidden wrapper for the `const_item!(file = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`FileCompileTime`] (a file-picker parameter) as an [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __file(value: FileCompileTime) -> Self {
        Self::File(value)
    }

    /// Hidden wrapper for the `const_item!(folder = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`FolderCompileTime`] (a folder-picker parameter) as an [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __folder(value: FolderCompileTime) -> Self {
        Self::Folder(value)
    }

    /// Hidden wrapper for the `const_item!(integer = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps an [`IntegerCompileTime`] (an integer value with an optional constraint) as an
    /// [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __integer(value: IntegerCompileTime) -> Self {
        Self::Integer(value)
    }

    /// Hidden wrapper for the `const_item!(map = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`MapCompileTime`] (a nested, dynamically keyed collection of map items) as
    /// an [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __map(value: MapCompileTime) -> Self {
        Self::Map(value)
    }

    /// Hidden wrapper for the `const_item!(number = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`NumberCompileTime`] (an `f64` value with an optional constraint) as an
    /// [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __number(value: NumberCompileTime) -> Self {
        Self::Number(value)
    }

    /// Hidden wrapper for the `const_item!(number_with_units = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`NumberWithUnitsCompileTime`] (an `f64` value with a preferred unit and an
    /// optional constraint) as an [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __number_with_units(value: NumberWithUnitsCompileTime) -> Self {
        Self::NumberWithUnits(value)
    }

    /// Hidden wrapper for the `const_item!(string = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`StringCompileTime`] (a free-form text value) as an [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __string(value: StringCompileTime) -> Self {
        Self::String(value)
    }

    /// Hidden wrapper for the `const_item!(table = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`TableCompileTime`] (a table of unit-less numeric columns) as an
    /// [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __table(value: TableCompileTime) -> Self {
        Self::Table(value)
    }

    /// Hidden wrapper for the `const_item!(table_with_units = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`TableWithUnitsCompileTime`] (a table of numeric columns, each with its own
    /// preferred unit) as an [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __table_with_units(value: TableWithUnitsCompileTime) -> Self {
        Self::TableWithUnits(value)
    }

    /// Hidden wrapper for the `const_item!(unit = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`UnitCompileTime`] (a value chosen from the units of a unit family) as an
    /// [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __unit(value: UnitCompileTime) -> Self {
        Self::Unit(value)
    }

    /// Hidden wrapper for the `const_item!(tab = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`TabCompileTime`] (a layout-only tab heading; stores no value) as an
    /// [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __tab(value: TabCompileTime) -> Self {
        Self::Tab(value)
    }

    /// Hidden wrapper for the `const_item!(separator = value)` arm.
    ///
    /// This is an implementation detail; call `const_item!` instead.
    /// Wraps a [`SeparatorCompileTime`] (a layout-only visual divider; stores no value) as
    /// an [`ItemCompileTime`].
    #[doc(hidden)]
    #[must_use]
    pub const fn __separator(value: SeparatorCompileTime) -> Self {
        Self::Separator(value)
    }

    /// Converts this compile-time item into a runtime definition.
    #[must_use]
    pub fn into_definition(self) -> ItemDefinitionType {
        match self {
            ItemCompileTime::Boolean(value) => ItemDefinitionType::Boolean(value.into_definition()),
            ItemCompileTime::Choice(value) => ItemDefinitionType::Choice(value.into_definition()),
            ItemCompileTime::File(value) => ItemDefinitionType::File(value.into_definition()),
            ItemCompileTime::Folder(value) => ItemDefinitionType::Folder(value.into_definition()),
            ItemCompileTime::Integer(value) => ItemDefinitionType::Integer(value.into_definition()),
            ItemCompileTime::Map(value) => ItemDefinitionType::Map(value.into_definition()),
            ItemCompileTime::Number(value) => ItemDefinitionType::Number(value.into_definition()),
            ItemCompileTime::NumberWithUnits(value) => {
                ItemDefinitionType::NumberWithUnits(value.into_definition())
            }
            ItemCompileTime::String(value) => ItemDefinitionType::String(value.into_definition()),
            ItemCompileTime::Table(value) => ItemDefinitionType::Table(value.into_definition()),
            ItemCompileTime::TableWithUnits(value) => {
                ItemDefinitionType::TableWithUnits(value.into_definition())
            }
            ItemCompileTime::Unit(value) => ItemDefinitionType::Unit(value.into_definition()),
            ItemCompileTime::Tab(value) => ItemDefinitionType::Tab(value.into_definition()),
            ItemCompileTime::Separator(value) => {
                ItemDefinitionType::Separator(value.into_definition())
            }
        }
    }
}

/// Wraps a compile-time value as an [`ItemCompileTime`] for use inside
/// `const_global_object!`, `const_parameter_object!`, and
/// `const_variable_object!` item lists.
///
/// Expansion is wrapped in a `const` block, so `value` must be a const-compatible
/// (`'static`) expression; construction is validated at compile time even when the result
/// is bound with a plain `let` instead of `const`.
///
/// # Syntax
/// ```text
/// const_item!(boolean = value)
/// const_item!(choice = value)
/// const_item!(file = value)
/// const_item!(folder = value)
/// const_item!(integer = value)
/// const_item!(map = value)
/// const_item!(number = value)
/// const_item!(number_with_units = value)
/// const_item!(string = value)
/// const_item!(table = value)
/// const_item!(table_with_units = value)
/// const_item!(unit = value)
/// const_item!(tab = value)
/// const_item!(separator = value)
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
/// use datastore::compile_time::ItemCompileTime;
/// use datastore::prelude::*;
///
/// const NAME: ItemCompileTime = const_item!(string = const_string!("Name"));
/// const READY: ItemCompileTime =
///     const_item!(boolean = const_boolean!("Ready", default = false));
/// let _definition = NAME.into_definition();
/// let _definition = READY.into_definition();
/// ```
#[macro_export]
macro_rules! const_item {
    (boolean = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__boolean($value)
        }
    };
    (choice = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__choice($value)
        }
    };
    (file = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__file($value)
        }
    };
    (folder = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__folder($value)
        }
    };
    (integer = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__integer($value)
        }
    };
    (map = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__map($value)
        }
    };
    (number = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__number($value)
        }
    };
    (number_with_units = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__number_with_units($value)
        }
    };
    (string = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__string($value)
        }
    };
    (table = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__table($value)
        }
    };
    (table_with_units = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__table_with_units($value)
        }
    };
    (unit = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__unit($value)
        }
    };
    (tab = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__tab($value)
        }
    };
    (separator = $value:expr) => {
        const {
            #[allow(clippy::disallowed_methods)]
            $crate::compile_time::ItemCompileTime::__separator($value)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{
        const_boolean, const_choice, const_choice_item, const_file, const_folder, const_integer,
        const_map, const_number, const_number_with_units, const_separator, const_string, const_tab,
        const_table, const_table_with_units, const_unit,
    };
    use units::{UnitFamilyId, UnitId};

    #[test]
    #[allow(clippy::disallowed_methods)]
    fn hidden_wrappers_run_at_runtime() {
        let items = [
            ItemCompileTime::__boolean(std::hint::black_box(const_boolean!("Boolean"))),
            ItemCompileTime::__choice(std::hint::black_box(const_choice!(
                "Choice",
                &[const_choice_item!("choice", "Choice")]
            ))),
            ItemCompileTime::__file(std::hint::black_box(const_file!("File", "*", true))),
            ItemCompileTime::__folder(std::hint::black_box(const_folder!("Folder", true))),
            ItemCompileTime::__integer(std::hint::black_box(const_integer!("Integer"))),
            ItemCompileTime::__map(std::hint::black_box(const_map!("Map", &[]))),
            ItemCompileTime::__number(std::hint::black_box(const_number!("Number"))),
            ItemCompileTime::__number_with_units(std::hint::black_box(const_number_with_units!(
                "Number with units",
                UnitId::Length_Meter
            ))),
            ItemCompileTime::__string(std::hint::black_box(const_string!("String"))),
            ItemCompileTime::__table(std::hint::black_box(const_table!("Table", &[]))),
            ItemCompileTime::__table_with_units(std::hint::black_box(const_table_with_units!(
                "Table with units",
                &[]
            ))),
            ItemCompileTime::__unit(std::hint::black_box(const_unit!(
                "Unit",
                UnitFamilyId::Length
            ))),
            ItemCompileTime::__tab(std::hint::black_box(const_tab!("Tab"))),
            ItemCompileTime::__separator(std::hint::black_box(const_separator!("Separator"))),
        ];

        assert!(matches!(items[0], ItemCompileTime::Boolean(_)));
        assert!(matches!(items[1], ItemCompileTime::Choice(_)));
        assert!(matches!(items[2], ItemCompileTime::File(_)));
        assert!(matches!(items[3], ItemCompileTime::Folder(_)));
        assert!(matches!(items[4], ItemCompileTime::Integer(_)));
        assert!(matches!(items[5], ItemCompileTime::Map(_)));
        assert!(matches!(items[6], ItemCompileTime::Number(_)));
        assert!(matches!(items[7], ItemCompileTime::NumberWithUnits(_)));
        assert!(matches!(items[8], ItemCompileTime::String(_)));
        assert!(matches!(items[9], ItemCompileTime::Table(_)));
        assert!(matches!(items[10], ItemCompileTime::TableWithUnits(_)));
        assert!(matches!(items[11], ItemCompileTime::Unit(_)));
        assert!(matches!(items[12], ItemCompileTime::Tab(_)));
        assert!(matches!(items[13], ItemCompileTime::Separator(_)));
    }
}
