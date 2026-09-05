use datastore::compile_time::{IntegerConstraint, IntegerConstraintEnum};
use datastore::prelude::*;

#[test]
fn integer_compile_time_converts_all_macro_forms() {
    let integer = const_integer!("Integer");
    let default = const_integer!("Integer default", default = "5");
    let maximum = const_integer!(
        "Integer maximum",
        constraint = IntegerConstraint::max(10, false)
    );
    let minimum = const_integer!(
        "Integer minimum",
        constraint = IntegerConstraint::min(0, true)
    );
    let range = const_integer!(
        "Integer range",
        constraint = IntegerConstraint::range(10, 0, false, true),
        default = "5"
    );

    assert_eq!(integer.description(), "Integer");
    assert_eq!(integer.constraint(), IntegerConstraintEnum::None);
    assert_eq!(integer.default_value(), "");
    assert_eq!(integer.into_definition().default_value(), "");
    assert_eq!(default.default_value(), "5");
    assert_eq!(default.into_definition().default_value(), "5");
    assert_eq!(
        maximum.constraint(),
        IntegerConstraintEnum::Max {
            max: 10,
            inclusive: false,
        }
    );
    assert_eq!(
        minimum.constraint(),
        IntegerConstraintEnum::Min {
            min: 0,
            inclusive: true,
        }
    );
    assert_eq!(
        range.constraint(),
        IntegerConstraintEnum::Range {
            min: 0,
            max: 10,
            min_inclusive: true,
            max_inclusive: false,
        }
    );
    assert_eq!(
        range.into_definition().constraint(),
        datastore::definition::IntegerConstraintEnum::Range {
            min: 0,
            max: 10,
            min_inclusive: true,
            max_inclusive: false,
        }
    );
}
