/// Canonicalize a given f64 value to ensure that `-0.0` is converted to `0.0`.
#[inline]
#[must_use]
pub fn canonicalize_f64(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}

#[cfg(test)]
mod tests {
    use super::canonicalize_f64;

    #[test]
    fn canonicalizes_negative_zero() {
        let result = canonicalize_f64(-0.0);

        assert_eq!(result.to_bits(), 0.0f64.to_bits());
        assert!(!result.is_sign_negative());
    }

    #[test]
    fn preserves_nonzero_values() {
        assert_eq!(canonicalize_f64(42.5).to_bits(), 42.5f64.to_bits());
        assert_eq!(canonicalize_f64(-42.5).to_bits(), (-42.5f64).to_bits());
    }
}
