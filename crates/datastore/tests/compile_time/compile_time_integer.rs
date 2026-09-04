use datastore::compile_time::{IntegerConstraint, IntegerConstraintEnum};
use datastore::prelude::*;

#[test]
fn integer_compile_time_converts_all_macro_forms() {
    let integer = integer_compile_time!("Integer");
    let default = integer_compile_time!("Integer default", default = "5");
    let maximum = integer_compile_time!(
        "Integer maximum",
        constraint = IntegerConstraint::max(10, false)
    );
    let minimum = integer_compile_time!(
        "Integer minimum",
        constraint = IntegerConstraint::min(0, true)
    );
    let range = integer_compile_time!(
        "Integer range",
        constraint = IntegerConstraint::range(10, 0, false, true),
        default = "5"
    );

    assert_eq!(integer.constraint(), IntegerConstraintEnum::None);
    assert_eq!(default.default_value(), "5");
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
