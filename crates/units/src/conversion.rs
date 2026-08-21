use crate::unit_definitions::{UnitFamilyId, UnitId};
use std::ops::{Add, Div, Mul, Sub};

#[allow(
    clippy::match_same_arms,
    reason = "This is a simple conversion table, and the repetition is intentional."
)]
/// Converts a given unit to its base unit equivalent.
const fn convert_to_base(unit: UnitId) -> f64 {
    match unit {
        UnitId::None => 1.0,

        UnitId::Area_SquareMeter => 1.0,
        UnitId::Area_SquareMilliMeter => 0.000_001,
        UnitId::Area_SquareCentiMeter => 0.0001,
        UnitId::Area_SquareKiloMeter => 1_000_000.0,
        UnitId::Area_SquareFoot => 0.092_903,
        UnitId::Area_SquareInch => 0.000_645_16,
        UnitId::Area_Acre => 4046.86,
        UnitId::Area_Hectare => 10000.0,
        UnitId::Area_SquareMile => 2_589_988.11,

        UnitId::Current_Ampere => 1.0,
        UnitId::Current_Milliampere => 0.001,
        UnitId::Current_Microampere => 0.000_001,
        UnitId::Current_Nanoampere => 0.000_000_001,
        UnitId::Current_Kiloampere => 1000.0,

        UnitId::Length_Meter => 1.0,
        UnitId::Length_Kilometer => 1000.0,
        UnitId::Length_Centimeter => 0.01,
        UnitId::Length_Millimeter => 0.001,
        UnitId::Length_Foot => 0.3048,
        UnitId::Length_Inch => 0.0254,
        UnitId::Length_Yard => 0.9144,
        UnitId::Length_Mile => 1609.34,

        UnitId::LuminousIntensity_Candela => 1.0,
        UnitId::LuminousIntensity_Millicandela => 0.001,
        UnitId::LuminousIntensity_Kilocandela => 1000.0,
        UnitId::LuminousIntensity_Hefnerkerze => 0.9,
        UnitId::LuminousIntensity_InternationalCandle => 1.0,
        UnitId::LuminousIntensity_DecimalCandle => 1.0,

        UnitId::Amount_Mole => 1.0,
        UnitId::Amount_Millimole => 0.001,
        UnitId::Amount_Micromole => 0.000_001,
        UnitId::Amount_Nanomole => 0.000_000_001,
        UnitId::Amount_Picomole => 0.000_000_000_001,
        UnitId::Amount_Kilomole => 1000.0,

        UnitId::Temperature_Celsius => 0.0,
        UnitId::Temperature_Fahrenheit => 0.0,
        UnitId::Temperature_Kelvin => 0.0,

        UnitId::Time_Second => 1.0,
        UnitId::Time_Minute => 60.0,
        UnitId::Time_Hour => 3600.0,
        UnitId::Time_Day => 86400.0,
        UnitId::Time_Week => 604_800.0,
        UnitId::Time_Year => 31_536_000.0,
        UnitId::Time_MilliSecond => 0.001,
        UnitId::Time_MicroSecond => 0.000_001,
        UnitId::Time_NanoSecond => 0.000_000_001,

        UnitId::Volume_Liter => 1.0,
        UnitId::Volume_Milliliter => 0.001,
        UnitId::Volume_Gallon => 3.78541,
        UnitId::Volume_ImperialGallon => 4.54609,
        UnitId::Volume_FluidOunce => 0.029_573_5,
        UnitId::Volume_ImperialFluidOunce => 0.028_413_1,
        UnitId::Volume_Cup => 0.236_588_236_5,
        UnitId::Volume_Pint => 0.473_176,
        UnitId::Volume_Quart => 0.946_353,

        UnitId::Mass_Kilogram => 1.0,
        UnitId::Mass_Gram => 0.001,
        UnitId::Mass_Pound => 0.453_592,
        UnitId::Mass_Ounce => 0.028_349_5,
        UnitId::Mass_Tonne => 1000.0,
        UnitId::Mass_Stone => 6.35029,
    }
}

/// Converts a value from one unit to another, returning an error if the units are incompatible.
///
/// # Errors
///
/// Returns an error if the units are not compatible for conversion (i.e., they belong to different unit families).
#[hotpath::measure]
pub fn convert(value: f64, from_unit: UnitId, to_unit: UnitId) -> Result<f64, String> {
    if !value.is_finite() {
        return Err("Unit conversion input must be finite".into());
    }

    if from_unit == to_unit {
        return Ok(value);
    }

    if to_unit == UnitId::None {
        return Ok(value);
    }

    if from_unit == UnitId::None {
        return Err("Cannot convert a unitless value to a unit".into());
    }

    if from_unit.family_id() != to_unit.family_id() {
        return Err("Units are not compatible for conversion".into());
    }

    if from_unit.family_id() == UnitFamilyId::Temperature {
        let converted = match (from_unit, to_unit) {
            (UnitId::Temperature_Celsius, UnitId::Temperature_Fahrenheit) => {
                value.mul_add(9.0.div(5.0), 32.0)
            }
            (UnitId::Temperature_Fahrenheit, UnitId::Temperature_Celsius) => {
                value.sub(32.0).mul(5.0.div(9.0))
            }
            (UnitId::Temperature_Celsius, UnitId::Temperature_Kelvin) => value.add(273.15),
            (UnitId::Temperature_Kelvin, UnitId::Temperature_Celsius) => value.sub(273.15),
            (UnitId::Temperature_Fahrenheit, UnitId::Temperature_Kelvin) => {
                value.sub(32.0).mul_add(5.0.div(9.0), 273.15)
            }
            (UnitId::Temperature_Kelvin, UnitId::Temperature_Fahrenheit) => {
                value.sub(273.15).mul_add(9.0.div(5.0), 32.0)
            }
            _ => {
                // This case should never happen because we check for unit family compatibility before calling this function.
                return Err("Units are not compatible for conversion".into());
            }
        };
        return converted
            .is_finite()
            .then_some(converted)
            .ok_or_else(|| "Unit conversion result must be finite".into());
    }

    let from_base = convert_to_base(from_unit);
    let to_base = convert_to_base(to_unit);

    let converted = value.mul(from_base.div(to_base));
    converted
        .is_finite()
        .then_some(converted)
        .ok_or_else(|| "Unit conversion result must be finite".into())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{convert, convert_to_base};
    use crate::unit_definitions::{UnitFamilyId, UnitId};
    use std::ops::{Mul, Sub};

    fn assert_approx_eq(actual: f64, expected: f64) {
        let tolerance = 1e-12_f64.mul(expected.abs().max(1.0));
        assert!(
            actual.sub(expected).abs() <= tolerance,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn preserves_values_when_units_match() {
        assert_eq!(
            convert(42.5, UnitId::Length_Meter, UnitId::Length_Meter),
            Ok(42.5)
        );
    }

    #[test]
    fn convert_to_base_temperature() {
        assert_approx_eq(convert_to_base(UnitId::Temperature_Celsius), 0.0);
        assert_approx_eq(convert_to_base(UnitId::Temperature_Fahrenheit), 0.0);
        assert_approx_eq(convert_to_base(UnitId::Temperature_Kelvin), 0.0);
    }

    #[test]
    fn converts_proportional_units() {
        let cases = [
            (
                1.0,
                UnitId::Area_SquareKiloMeter,
                UnitId::Area_Hectare,
                100.0,
            ),
            (
                2_500.0,
                UnitId::Current_Milliampere,
                UnitId::Current_Ampere,
                2.5,
            ),
            (1.0, UnitId::Length_Yard, UnitId::Length_Foot, 3.0),
            (
                1.0,
                UnitId::LuminousIntensity_Hefnerkerze,
                UnitId::LuminousIntensity_Candela,
                0.9,
            ),
            (3.0, UnitId::Amount_Kilomole, UnitId::Amount_Mole, 3_000.0),
            (2.0, UnitId::Time_Hour, UnitId::Time_Minute, 120.0),
            (
                1.0,
                UnitId::Volume_ImperialGallon,
                UnitId::Volume_Liter,
                4.54609,
            ),
            (1.0, UnitId::Mass_Stone, UnitId::Mass_Kilogram, 6.35029),
        ];

        for (value, from_unit, to_unit, expected) in cases {
            let actual = convert(value, from_unit, to_unit).unwrap();
            assert_approx_eq(actual, expected);
        }
    }

    #[test]
    fn converts_temperature_scales_with_offsets() {
        let cases = [
            (
                0.0,
                UnitId::Temperature_Celsius,
                UnitId::Temperature_Fahrenheit,
                32.0,
            ),
            (
                -40.0,
                UnitId::Temperature_Fahrenheit,
                UnitId::Temperature_Celsius,
                -40.0,
            ),
            (
                273.15,
                UnitId::Temperature_Kelvin,
                UnitId::Temperature_Celsius,
                0.0,
            ),
            (
                373.15,
                UnitId::Temperature_Kelvin,
                UnitId::Temperature_Fahrenheit,
                212.0,
            ),
        ];

        for (value, from_unit, to_unit, expected) in cases {
            let actual = convert(value, from_unit, to_unit).unwrap();
            assert_approx_eq(actual, expected);
        }
    }

    #[test]
    fn rejects_incompatible_units() {
        assert_eq!(
            convert(1.0, UnitId::Length_Meter, UnitId::Time_Second),
            Err("Units are not compatible for conversion".into())
        );
    }

    #[test]
    fn converts_concrete_units_to_unitless_values() {
        assert_eq!(convert(42.5, UnitId::Length_Meter, UnitId::None), Ok(42.5));
    }

    #[test]
    fn rejects_converting_unitless_values_to_concrete_units() {
        assert_eq!(
            convert(42.5, UnitId::None, UnitId::Length_Meter),
            Err("Cannot convert a unitless value to a unit".into())
        );
    }

    #[test]
    fn rejects_non_finite_inputs_and_results() {
        assert!(convert(f64::NAN, UnitId::Length_Meter, UnitId::Length_Foot).is_err());
        assert!(convert(f64::INFINITY, UnitId::Length_Meter, UnitId::Length_Foot).is_err());
        assert!(convert(f64::MAX, UnitId::Length_Kilometer, UnitId::Length_Meter).is_err());
    }

    #[test]
    fn round_trips_every_unit_within_its_family() {
        for from_unit in UnitId::ALL {
            for to_unit in UnitId::ALL {
                if from_unit.family_id() != to_unit.family_id() {
                    continue;
                }

                let converted = convert(12.5, *from_unit, *to_unit).unwrap();
                let round_tripped = convert(converted, *to_unit, *from_unit).unwrap();
                assert_approx_eq(round_tripped, 12.5);
            }
        }
    }

    #[test]
    fn every_family_has_convertible_units() {
        for family in UnitFamilyId::ALL {
            let units = UnitId::ALL
                .iter()
                .filter(|unit| unit.family_id() == family)
                .count();

            assert!(units > 0, "{family:?} must contain at least one unit");
        }
    }
}
