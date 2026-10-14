use super::{ComputedItem, evaluate_number_expression};

// Frozen with libm 0.2.16 on Rust 1.99.0 using the repository's baseline CPU flags.
// These are bit-pattern compatibility references, not approximate accuracy tests.
fn assert_expression_bits(expression: &str, expected: u64) {
    match evaluate_number_expression(expression) {
        ComputedItem::Float(actual) => assert_eq!(
            actual.to_bits(),
            expected,
            "expression `{expression}` changed its binary64 result: actual {:#018x}, expected {expected:#018x}",
            actual.to_bits(),
        ),
        other => panic!("expected expression `{expression}` to produce a float, got {other:?}"),
    }
}

#[test]
fn transcendental_reference_bits() {
    for (expression, expected) in [
        ("sin(0.123456789)", 0x3fbf_8653_acec_ff98),
        ("cos(0.987654321)", 0x3fe1_9eeb_7c6c_2316),
        ("tan(0.314159265)", 0x3fd4_cb7b_fadc_49e5),
        ("arcsin(0.333333333)", 0x3fd5_bfe3_4ea3_e1f7),
        ("arccos(0.666666667)", 0x3fea_ea08_d7fb_7a64),
        ("arctan(0.777777777)", 0x3fe5_2744_00ac_0c47),
        ("arctan2(0.123456789, -0.987654321)", 0x4008_234d_7f95_54c5),
        ("sinh(0.125)", 0x3fc0_0aac_cd00_d2f1),
        ("cosh(0.375)", 0x3ff1_2364_0f68_5b59),
        ("tanh(0.625)", 0x3fe1_bf47_eabb_8f96),
        ("sqrt(12.3456789)", 0x400c_1bf0_3f3f_15b6),
        ("log(1.23456789)", 0x3fca_f8e8_0d7f_7a34),
        ("log2(3.14159265)", 0x3ffa_6c87_3427_94e8),
        ("log10(98.7654321)", 0x3fff_e9e6_e628_2503),
        ("exp(0.123456789)", 0x3ff2_1a38_0e17_ced3),
        ("to_degrees(0.123456789)", 0x401c_4b51_77b8_2cf3),
        ("to_radians(12.3456789)", 0x3fcb_949c_64e7_8da9),
        ("sin(123456789.125)", 0x3fef_fefc_c2ee_7d37),
    ] {
        assert_expression_bits(expression, expected);
    }
}

#[test]
fn power_and_remainder_reference_bits() {
    for (expression, expected) in [
        ("1.23456789 ^ 0.333333333", 0x3ff1_2a0c_a8e7_f099),
        ("2.0 ^ -1074.0", 0x0000_0000_0000_0001),
        ("2.0 ^ -1075.0", 0x0000_0000_0000_0000),
        ("(-1.23456789) ^ 7.0", 0xc011_7c26_d5ec_7cfb),
        ("5.3 % 0.2", 0x3fb9_9999_9999_9978),
        ("-5.3 % 0.2", 0xbfb9_9999_9999_9978),
        ("5.3 % -0.2", 0x3fb9_9999_9999_9978),
        ("-4.0 % 2.0", 0x0000_0000_0000_0000),
        ("1e-323 % 5e-324", 0x0000_0000_0000_0000),
    ] {
        assert_expression_bits(expression, expected);
    }
}

#[test]
fn subnormal_and_underflow_reference_bits() {
    for (expression, expected) in [
        ("5e-324", 0x0000_0000_0000_0001),
        ("5e-324 + 5e-324", 0x0000_0000_0000_0002),
        // Half of the least subnormal ties to zero; 1.5 and 2.5 ULP both tie to 2.
        ("5e-324 * 0.5", 0x0000_0000_0000_0000),
        ("-5e-324 * 0.5", 0x0000_0000_0000_0000),
        ("1.5e-323 * 0.5", 0x0000_0000_0000_0002),
        ("2.5e-323 * 0.5", 0x0000_0000_0000_0002),
        ("2.2250738585072014e-308 / 2.0", 0x0008_0000_0000_0000),
        ("sin(5e-324)", 0x0000_0000_0000_0001),
        ("sqrt(5e-324)", 0x1e60_0000_0000_0000),
        ("exp(-745.0)", 0x0000_0000_0000_0001),
        ("exp(-746.0)", 0x0000_0000_0000_0000),
    ] {
        assert_expression_bits(expression, expected);
    }
}

#[test]
fn rounding_boundary_and_zero_reference_bits() {
    for (expression, expected) in [
        ("round(0.49999999999999994)", 0x0000_0000_0000_0000),
        ("round(0.5)", 0x3ff0_0000_0000_0000),
        ("round(0.5000000000000001)", 0x3ff0_0000_0000_0000),
        ("round(-0.49999999999999994)", 0x0000_0000_0000_0000),
        ("round(-0.5)", 0xbff0_0000_0000_0000),
        ("ceil(1.0000000000000002)", 0x4000_0000_0000_0000),
        ("floor(0.9999999999999999)", 0x0000_0000_0000_0000),
        // The engine canonicalizes every signed-zero result to positive zero.
        ("ceil(-0.25)", 0x0000_0000_0000_0000),
        ("-0.0", 0x0000_0000_0000_0000),
        ("sin(-0.0)", 0x0000_0000_0000_0000),
        ("-0.0 / 3.0", 0x0000_0000_0000_0000),
        ("9007199254740992.0 + 1.0", 0x4340_0000_0000_0000),
        ("9007199254740994.0 + 1.0", 0x4340_0000_0000_0002),
    ] {
        assert_expression_bits(expression, expected);
    }
}

#[test]
fn arithmetic_order_and_unfused_reference_bits() {
    for (expression, expected) in [
        (
            "(10000000000000000.0 + -10000000000000000.0) + 1.0",
            0x3ff0_0000_0000_0000,
        ),
        (
            "10000000000000000.0 + (-10000000000000000.0 + 1.0)",
            0x0000_0000_0000_0000,
        ),
        // A fused multiply-add would instead retain the exact product's -2^-104.
        (
            "1.0000000000000002 * 0.9999999999999998 - 1.0",
            0x0000_0000_0000_0000,
        ),
        (
            "1.0000000000000002 * (0.9999999999999998 - 1.0)",
            0xbcb0_0000_0000_0001,
        ),
        ("0.1 + 0.2 + 0.3", 0x3fe3_3333_3333_3334),
        ("0.1 + (0.2 + 0.3)", 0x3fe3_3333_3333_3333),
        ("2.0 ^ 3.0 ^ 2.0", 0x4080_0000_0000_0000),
        ("(2.0 ^ 3.0) ^ 2.0", 0x4050_0000_0000_0000),
        // Unit conversion intentionally fuses this formula; expression operators must not.
        (
            "-17.77777777777778 * (9.0 / 5.0) + 32.0",
            0x0000_0000_0000_0000,
        ),
    ] {
        assert_expression_bits(expression, expected);
    }
}
