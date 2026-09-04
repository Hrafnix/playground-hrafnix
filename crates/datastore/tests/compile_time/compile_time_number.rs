use datastore::compile_time::{NumberConstraint, NumberConstraintEnum};
use datastore::prelude::*;

#[test]
fn number_compile_time_converts_all_macro_forms() {
    let number = number_compile_time!("Number");
    let default = number_compile_time!("Number default", default = "1.5");
    let minimum = number_compile_time!(
        "Number minimum",
        constraint = NumberConstraint::min(1.0, true)
    );
    let range = number_compile_time!(
        "Number range",
        constraint = NumberConstraint::range(10.0, 0.0, false, true),
        default = "5"
    );

    assert_eq!(number.constraint(), NumberConstraintEnum::None);
    assert_eq!(default.default_value(), "1.5");
    assert_eq!(
        minimum.constraint(),
        NumberConstraintEnum::Min {
            min: 1.0,
            inclusive: true,
        }
    );
    assert_eq!(
        range.constraint(),
        NumberConstraintEnum::Range {
            min: 0.0,
            max: 10.0,
            min_inclusive: true,
            max_inclusive: false,
        }
    );
    assert_eq!(
        range.into_definition().constraint(),
        datastore::definition::NumberConstraintEnum::Range {
            min: 0.0,
            max: 10.0,
            min_inclusive: true,
            max_inclusive: false,
        }
    );
}
