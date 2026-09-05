use datastore::compile_time::{NumberConstraint, NumberConstraintEnum};
use datastore::prelude::*;
use units::UnitId;

#[test]
fn number_with_units_compile_time_converts_all_macro_forms() {
    let number = const_number_with_units!("Length", UnitId::Length_Meter);
    let default = const_number_with_units!("Length default", UnitId::Length_Meter, default = "2");
    let maximum = const_number_with_units!(
        "Length maximum",
        UnitId::Length_Meter,
        constraint = NumberConstraint::max(3.0, false)
    );
    let range = const_number_with_units!(
        "Length range",
        UnitId::Length_Meter,
        constraint = NumberConstraint::range(0.0, 2.0, true, false),
        default = "1"
    );

    assert_eq!(number.constraint(), NumberConstraintEnum::None);
    assert_eq!(default.default_value(), "2");
    assert_eq!(
        maximum.constraint(),
        NumberConstraintEnum::Max {
            max: 3.0,
            inclusive: false,
        }
    );
    assert_eq!(range.preferred_units(), UnitId::Length_Meter);
    assert_eq!(range.default_value(), "1");
    assert_eq!(
        range.into_definition().constraint(),
        datastore::definition::NumberConstraintEnum::Range {
            min: 0.0,
            max: 2.0,
            min_inclusive: true,
            max_inclusive: false,
        }
    );
}
