//! Fixed binary64 compatibility references for unit conversions.

#![allow(clippy::expect_used)]

use hrafnix_units::conversion::convert;
use hrafnix_units::unit_definitions::UnitId;
use std::ops::{Add, Div, Mul};

// Frozen on Rust 1.99.0 with the repository's baseline CPU flags.

#[test]
fn temperature_conversion_reference_bits() {
    use UnitId::{
        Temperature_Celsius as Celsius, Temperature_Fahrenheit as Fahrenheit,
        Temperature_Kelvin as Kelvin, Temperature_Millikelvin as Millikelvin,
    };

    for (value, from, to, expected) in [
        (
            -17.777_777_777_777_78,
            Celsius,
            Fahrenheit,
            0xbce3_e93e_93e9_3e94,
        ),
        (-459.67, Fahrenheit, Kelvin, 0xbd28_9b45_6789_abce),
        (-459.67, Fahrenheit, Millikelvin, 0xbdc8_07a1_c71c_71c7),
        (
            255.372_222_222_222_18,
            Kelvin,
            Fahrenheit,
            0xbd26_d82d_82d8_2d83,
        ),
        (
            255_372.222_222_222_2,
            Millikelvin,
            Fahrenheit,
            0x3d07_d27d_27d2_7d28,
        ),
        (98.765_432_1, Fahrenheit, Celsius, 0x4042_8bc3_9977_1d0e),
        (12.345_678_9, Celsius, Millikelvin, 0x4111_6cde_b731_8fc4),
    ] {
        let actual = convert(value, from, to).expect("reference conversion should succeed");
        assert_eq!(
            actual.to_bits(),
            expected,
            "{value} {from:?} -> {to:?} changed temperature conversion bits; preserve the intentional mul_add and operation order: actual {:#018x}, expected {expected:#018x}",
            actual.to_bits(),
        );
    }

    // This cancellation-sensitive input distinguishes deliberate mul_add from two roundings.
    let unfused = (-17.777_777_777_777_78_f64).mul(9.0_f64.div(5.0)).add(32.0);
    assert_eq!(
        unfused.to_bits(),
        0x0000_0000_0000_0000,
        "the separately rounded Celsius formula must be zero, unlike the conversion reference",
    );
}

#[test]
fn scaled_conversion_reference_bits() {
    use UnitId::{
        Length_Foot as Foot, Length_Inch as Inch, Mass_Gram as Gram, Mass_Pound as Pound,
        Time_Minute as Minute, Time_Second as Second,
    };

    for (input, from, to, expected) in [
        (0x3ff3_c0ca_4283_de1b, Foot, Inch, 0x402d_a12f_63c5_cd2a),
        (0x3ff3_c0ca_4283_de1b, Pound, Gram, 0x4081_7fec_b2aa_2e2d),
        (0x0000_0000_0000_0001, Minute, Second, 0x0000_0000_0000_003c),
        (0x0000_0000_0000_0001, Second, Minute, 0x0000_0000_0000_0000),
        (0x0000_0000_0000_001f, Second, Minute, 0x0000_0000_0000_0001),
        (0x8000_0000_0000_0001, Second, Minute, 0x0000_0000_0000_0000),
        (0x8000_0000_0000_0000, Second, Minute, 0x0000_0000_0000_0000),
        (0x8000_0000_0000_0000, Second, Second, 0x0000_0000_0000_0000),
        (
            0x8000_0000_0000_0000,
            Second,
            UnitId::None,
            0x0000_0000_0000_0000,
        ),
    ] {
        let actual =
            convert(f64::from_bits(input), from, to).expect("reference conversion should succeed");
        assert_eq!(
            actual.to_bits(),
            expected,
            "input {input:#018x} {from:?} -> {to:?} changed scaling, gradual underflow, or zero canonicalization: actual {:#018x}, expected {expected:#018x}",
            actual.to_bits(),
        );
    }
}
