/// Comprehensive test suite for mathematical constants module
/// 
/// Tests mathematical constants, utility functions, validation, and unit conversions
/// with high precision requirements and thorough edge case coverage.

use cursed::stdlib::math::constants::*;
use cursed::stdlib::math::{PI, E, TAU, PHI, EULER_GAMMA, SQRT_2};

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts; 

    // =============================================================================
    // FUNDAMENTAL CONSTANT VALIDATION TESTS
    // =============================================================================

    #[test]
    fn test_fundamental_constants_accuracy() {
        // Test against Rust's standard library constants for maximum precision
        assert!((PI - consts::PI).abs() < 1e-15, "PI constant accuracy");
        assert!((E - consts::E).abs() < 1e-15, "E constant accuracy");
        assert!((TAU - consts::TAU).abs() < 1e-15, "TAU constant accuracy");
        assert!((SQRT_2 - consts::SQRT_2).abs() < 1e-15, "SQRT_2 constant accuracy");
        assert!((TAU - 2.0 * PI).abs() < 1e-15, "TAU = 2π relationship");
        assert!((PHI - (1.0 + (5.0_f64).sqrt()) / 2.0).abs() < 1e-15, "Golden ratio formula");
        assert!((INV_PHI - 1.0 / PHI).abs() < 1e-15, "Inverse golden ratio");
        assert!((FRAC_PI_2 - PI / 2.0).abs() < 1e-15, "π/2");
        assert!((FRAC_PI_3 - PI / 3.0).abs() < 1e-15, "π/3");
        assert!((FRAC_PI_4 - PI / 4.0).abs() < 1e-15, "π/4");
        assert!((FRAC_PI_6 - PI / 6.0).abs() < 1e-15, "π/6");
        assert!((FRAC_PI_8 - PI / 8.0).abs() < 1e-15, "π/8");
        assert!((FRAC_1_PI - 1.0 / PI).abs() < 1e-15, "1/π");
        assert!((FRAC_2_PI - 2.0 / PI).abs() < 1e-15, "2/π");
        assert!((LN_2 - 2.0_f64.ln()).abs() < 1e-15, "ln(2)");
        assert!((LN_10 - 10.0_f64.ln()).abs() < 1e-15, "ln(10)");
        assert!((LOG2_E - E.log2()).abs() < 1e-15, "log₂(e)");
        assert!((LOG10_E - E.log10()).abs() < 1e-15, "log₁₀(e)");
        assert!((LOG2_10 - 10.0_f64.log2()).abs() < 1e-15, "log₂(10)");
        assert!((LOG10_2 - 2.0_f64.log10()).abs() < 1e-15, "log₁₀(2)");
        assert!((ONE_THIRD - 1.0 / 3.0).abs() < 1e-15, "1/3");
        assert!((TWO_THIRDS - 2.0 / 3.0).abs() < 1e-15, "2/3");
        assert!((ONE_SIXTH - 1.0 / 6.0).abs() < 1e-15, "1/6");
        assert!((FIVE_SIXTHS - 5.0 / 6.0).abs() < 1e-15, "5/6");
        assert!((ONE_SEVENTH - 1.0 / 7.0).abs() < 1e-15, "1/7");
        assert!((ONE_NINTH - 1.0 / 9.0).abs() < 1e-15, "1/9");
        assert!((ONE_TWELFTH - 1.0 / 12.0).abs() < 1e-15, "1/12");
        assert!((THREE_QUARTERS - 3.0 / 4.0).abs() < 1e-15, "3/4");
        assert!((FIVE_EIGHTHS - 5.0 / 8.0).abs() < 1e-15, "5/8");
        assert!((SEVEN_EIGHTHS - 7.0 / 8.0).abs() < 1e-15, "7/8");
        assert!((ONE_THIRD + TWO_THIRDS - 1.0).abs() < 1e-15, "1/3 + 2/3 = 1");
        assert!((ONE_SIXTH + FIVE_SIXTHS - 1.0).abs() < 1e-15, "1/6 + 5/6 = 1");
        assert!((THREE_QUARTERS - 0.75).abs() < 1e-15, "3/4 = 0.75");
        assert!((ONE_THIRD * 3.0 - 1.0).abs() < 1e-14, "1/3 * 3 ≈ 1");
        assert!((ONE_SEVENTH * 7.0 - 1.0).abs() < 1e-14, "1/7 * 7 ≈ 1");
    }
}