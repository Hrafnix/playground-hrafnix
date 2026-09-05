use datastore::compile_time::{NumberConstraint, NumberConstraintEnum};
use datastore::prelude::*;

#[test]
fn number_compile_time_converts_all_macro_forms() {
    let number = const_number!("Number");
    let default = const_number!("Number default", default = "1.5");
    let minimum = const_number!(
        "Number minimum",
        constraint = NumberConstraint::min(1.0, true)
    );
    let range = const_number!(
        "Number range",
        constraint = NumberConstraint::range(10.0, 0.0, false, true),
        default = "5"
    );

    assert_eq!(number.description(), "Number");
    assert_eq!(number.constraint(), NumberConstraintEnum::None);
    assert_eq!(number.default_value(), "");
    assert_eq!(number.into_definition().default_value(), "");
    assert_eq!(default.default_value(), "1.5");
    assert_eq!(default.into_definition().default_value(), "1.5");
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
