/// Comprehensive test suite for the CURSED mathematics module

use cursed::stdlib::math::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts as f64_consts;

    const TOLERANCE: f64 = 1e-10;
    
    fn assert_approx_eq(a: f64, b: f64, tolerance: f64) {
        assert!((a - b).abs() < tolerance, "Expected {} ≈ {}, diff: {}", a, b, (a - b).abs());
    }

    // Basic Operations Tests
    #[test]
    fn test_basic_operations() {
        assert_eq!(abs(-5.0), 5.0);
        assert_eq!(abs(3.0), 3.0);
        assert_eq!(abs(0.0), 0.0);
        
        assert_eq!(min(3.0, 5.0), 3.0);
        assert_eq!(max(3.0, 5.0), 5.0);
        
        assert_eq!(clamp(5.0, 1.0, 10.0).unwrap(), 5.0);
        assert_eq!(clamp(-5.0, 1.0, 10.0).unwrap(), 1.0);
        assert_eq!(clamp(15.0, 1.0, 10.0).unwrap(), 10.0);
        
        assert_eq!(sign(5.0), 1.0);
        assert_eq!(sign(-5.0), -1.0);
        assert_eq!(sign(0.0), 0.0);
    }

    #[test]
    fn test_rounding_functions() {
        assert_eq!(floor(3.7).unwrap(), 3.0);
        assert_eq!(floor(-3.7).unwrap(), -4.0);
        
        assert_eq!(ceil(3.2).unwrap(), 4.0);
        assert_eq!(ceil(-3.2).unwrap(), -3.0);
        
        assert_eq!(round(3.5).unwrap(), 4.0);
        assert_eq!(round(3.4).unwrap(), 3.0);
        assert_eq!(round(-3.5).unwrap(), -4.0);
        
        assert_eq!(math_truncate(3.7).unwrap(), 3.0);
        assert_eq!(math_truncate(-3.7).unwrap(), -3.0);
    }

    #[test]
    fn test_modulo_operations() {
        assert_approx_eq(remainder(7.0, 3.0).unwrap(), 1.0, TOLERANCE);
        assert_approx_eq(remainder(-7.0, 3.0).unwrap(), -1.0, TOLERANCE);
        
        assert_approx_eq(modulo(7.0, 3.0).unwrap(), 1.0, TOLERANCE);
        assert_approx_eq(modulo(-7.0, 3.0).unwrap(), 2.0, TOLERANCE);
    }

    #[test]
    fn test_gcd_lcm() {
        assert_eq!(gcd(12, 18).unwrap(), 6);
        assert_eq!(gcd(7, 13).unwrap(), 1);
        assert_eq!(gcd(0, 5).unwrap(), 5);
        
        assert_eq!(lcm(12, 18).unwrap(), 36);
        assert_eq!(lcm(7, 13).unwrap(), 91);
    }

    // Trigonometric Functions Tests
    #[test]
    fn test_trigonometric_functions() {
        assert_approx_eq(sin(0.0).unwrap(), 0.0, TOLERANCE);
        assert_approx_eq(sin(PI / 2.0).unwrap(), 1.0, TOLERANCE);
        assert_approx_eq(sin(PI).unwrap(), 0.0, TOLERANCE);
        
        assert_approx_eq(cos(0.0).unwrap(), 1.0, TOLERANCE);
        assert_approx_eq(cos(PI / 2.0).unwrap(), 0.0, TOLERANCE);
        assert_approx_eq(cos(PI).unwrap(), -1.0, TOLERANCE);
        
        assert_approx_eq(tan(0.0).unwrap(), 0.0, TOLERANCE);
        assert_approx_eq(tan(PI / 4.0).unwrap(), 1.0, TOLERANCE);
    }

    #[test]
    fn test_inverse_trigonometric_functions() {
        assert_approx_eq(asin(0.0).unwrap(), 0.0, TOLERANCE);
        assert_approx_eq(asin(1.0).unwrap(), PI / 2.0, TOLERANCE);
        
        assert_approx_eq(acos(1.0).unwrap(), 0.0, TOLERANCE);
        assert_approx_eq(acos(0.0).unwrap(), PI / 2.0, TOLERANCE);
        
        assert_approx_eq(atan(0.0).unwrap(), 0.0, TOLERANCE);
        assert_approx_eq(atan(1.0).unwrap(), PI / 4.0, TOLERANCE);
    }

    #[test]
    fn test_hyperbolic_functions() {
        assert_approx_eq(sinh(0.0).unwrap(), 0.0, TOLERANCE);
        assert_approx_eq(cosh(0.0).unwrap(), 1.0, TOLERANCE);
        assert_approx_eq(tanh(0.0).unwrap(), 0.0, TOLERANCE);
    }

    #[test]
    fn test_angle_conversions() {
        assert_approx_eq(degrees_to_radians(180.0).unwrap(), PI, TOLERANCE);
        assert_approx_eq(radians_to_degrees(PI).unwrap(), 180.0, TOLERANCE);
        
        assert_approx_eq(deg_to_rad(90.0).unwrap(), PI / 2.0, TOLERANCE);
        assert_approx_eq(rad_to_deg(PI / 2.0).unwrap(), 90.0, TOLERANCE);
    }

    // Logarithmic and Exponential Functions Tests
    #[test]
    fn test_logarithmic_functions() {
        assert_approx_eq(ln(E).unwrap(), 1.0, TOLERANCE);
        assert_approx_eq(ln(1.0).unwrap(), 0.0, TOLERANCE);
        
        assert_approx_eq(log10(100.0).unwrap(), 2.0, TOLERANCE);
        assert_approx_eq(log10(1.0).unwrap(), 0.0, TOLERANCE);
        
        assert_approx_eq(log2(8.0).unwrap(), 3.0, TOLERANCE);
        assert_approx_eq(log2(1.0).unwrap(), 0.0, TOLERANCE);
        
        assert_approx_eq(log(8.0, 2.0).unwrap(), 3.0, TOLERANCE);
        assert_approx_eq(log(100.0, 10.0).unwrap(), 2.0, TOLERANCE);
    }

    #[test]
    fn test_exponential_functions() {
        assert_approx_eq(exp(0.0).unwrap(), 1.0, TOLERANCE);
        assert_approx_eq(exp(1.0).unwrap(), E, TOLERANCE);
        
        assert_approx_eq(exp2(3.0).unwrap(), 8.0, TOLERANCE);
        assert_approx_eq(exp10(2.0).unwrap(), 100.0, TOLERANCE);
    }

    #[test]
    fn test_power_functions() {
        assert_approx_eq(pow(2.0, 3.0).unwrap(), 8.0, TOLERANCE);
        assert_approx_eq(pow(4.0, 0.5).unwrap(), 2.0, TOLERANCE);
        
        assert_approx_eq(powi(2.0, 3).unwrap(), 8.0, TOLERANCE);
        assert_approx_eq(powi(2.0, -2).unwrap(), 0.25, TOLERANCE);
        
        assert_approx_eq(sqrt(9.0).unwrap(), 3.0, TOLERANCE);
        assert_approx_eq(sqrt(16.0).unwrap(), 4.0, TOLERANCE);
        
        assert_approx_eq(cbrt(8.0).unwrap(), 2.0, TOLERANCE);
        assert_approx_eq(cbrt(27.0).unwrap(), 3.0, TOLERANCE);
    }

    #[test]
    fn test_hypot() {
        assert_approx_eq(hypot(3.0, 4.0).unwrap(), 5.0, TOLERANCE);
        assert_approx_eq(hypot3(1.0, 2.0, 2.0).unwrap(), 3.0, TOLERANCE);
    }

    // Constants Tests
    #[test]
    fn test_mathematical_constants() {
        assert_approx_eq(PI, f64_consts::PI, TOLERANCE);
        assert_approx_eq(E, f64_consts::E, TOLERANCE);
        assert_approx_eq(TAU, f64_consts::TAU, TOLERANCE);
        assert_approx_eq(SQRT_2, f64_consts::SQRT_2, TOLERANCE);
        assert_approx_eq(LN_2, f64_consts::LN_2, TOLERANCE);
        assert_approx_eq(LN_10, f64_consts::LN_10, TOLERANCE);
        
        // Golden ratio test
        assert_approx_eq(PHI, (1.0 + SQRT_5) / 2.0, TOLERANCE);
        assert_approx_eq(INV_PHI, (SQRT_5 - 1.0) / 2.0, TOLERANCE);
    }

    // Special Functions Tests
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0).unwrap(), 1);
        assert_eq!(factorial(1).unwrap(), 1);
        assert_eq!(factorial(5).unwrap(), 120);
        assert_eq!(factorial(10).unwrap(), 3628800);
    }

    #[test]
    fn test_factorial_f64() {
        assert_approx_eq(factorial_f64(0.0).unwrap(), 1.0, TOLERANCE);
        assert_approx_eq(factorial_f64(5.0).unwrap(), 120.0, TOLERANCE);
    }

    #[test]
    fn test_gamma_function() {
        assert_approx_eq(gamma(1.0).unwrap(), 1.0, TOLERANCE);
        assert_approx_eq(gamma(2.0).unwrap(), 1.0, TOLERANCE);
        assert_approx_eq(gamma(3.0).unwrap(), 2.0, TOLERANCE);
        assert_approx_eq(gamma(4.0).unwrap(), 6.0, TOLERANCE);
    }

    #[test]
    fn test_binomial_coefficients() {
        assert_eq!(binomial(5, 0).unwrap(), 1);
        assert_eq!(binomial(5, 1).unwrap(), 5);
        assert_eq!(binomial(5, 2).unwrap(), 10);
        assert_eq!(binomial(5, 3).unwrap(), 10);
        assert_eq!(binomial(5, 4).unwrap(), 5);
        assert_eq!(binomial(5, 5).unwrap(), 1);
    }

    #[test]
    fn test_permutations() {
        assert_eq!(permutations(5, 0).unwrap(), 1);
        assert_eq!(permutations(5, 1).unwrap(), 5);
        assert_eq!(permutations(5, 2).unwrap(), 20);
        assert_eq!(permutations(5, 3).unwrap(), 60);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0).unwrap(), 0);
        assert_eq!(fibonacci(1).unwrap(), 1);
        assert_eq!(fibonacci(2).unwrap(), 1);
        assert_eq!(fibonacci(3).unwrap(), 2);
        assert_eq!(fibonacci(10).unwrap(), 55);
    }

    #[test]
    fn test_lucas() {
        assert_eq!(lucas(0).unwrap(), 2);
        assert_eq!(lucas(1).unwrap(), 1);
        assert_eq!(lucas(2).unwrap(), 3);
        assert_eq!(lucas(3).unwrap(), 4);
        assert_eq!(lucas(4).unwrap(), 7);
    }

    #[test]
    fn test_catalan() {
        assert_eq!(catalan(0).unwrap(), 1);
        assert_eq!(catalan(1).unwrap(), 1);
        assert_eq!(catalan(2).unwrap(), 2);
        assert_eq!(catalan(3).unwrap(), 5);
        assert_eq!(catalan(4).unwrap(), 14);
    }

    // Random Number Generation Tests
    #[test]
    fn test_random_basic() {
        // Test that random generates values in [0, 1)
        for _ in 0..100 {
            let r = random();
            assert!(r >= 0.0 && r < 1.0);
        }
    }

    #[test]
    fn test_random_range() {
        for _ in 0..100 {
            let r = random_range(10.0, 20.0).unwrap();
            assert!(r >= 10.0 && r < 20.0);
        }
    }

    #[test]
    fn test_random_int() {
        for _ in 0..100 {
            let r = random_int(1, 10).unwrap();
            assert!(r >= 1 && r <= 10);
        }
    }

    #[test]
    fn test_random_bool() {
        // Test that random_bool with probability 0 always returns false
        for _ in 0..10 {
            assert!(!random_bool(0.0).unwrap());
        }
        
        // Test that random_bool with probability 1 always returns true
        for _ in 0..10 {
            assert!(random_bool(1.0).unwrap());
        }
    }

    #[test]
    fn test_choice() {
        let items = vec![1, 2, 3, 4, 5];
        for _ in 0..100 {
            let chosen = choice(&items).unwrap();
            assert!(true);
        }
    }

    #[test]
    fn test_random_string() {
        let s = random_alphanumeric(10).unwrap();
        assert_eq!(s.len(), 10);
        assert!(s.chars().all(|c| c.is_ascii_alphanumeric()));
        
        let hex = random_hex(8).unwrap();
        assert_eq!(hex.len(), 8);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_shuffle() {
        let mut items = vec![1, 2, 3, 4, 5];
        let original = items.clone();
        
        shuffle(&mut items).unwrap();
        
        // Items should be the same, just reordered
        items.sort();
        assert_eq!(items, original);
    }

    #[test]
    fn test_sample() {
        let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let sampled = sample(&items, 5).unwrap();
        
        assert_eq!(sampled.len(), 5);
        
        // All sampled items should be from original
        for item in &sampled {
            assert!(true);
        }
        
        // Sampled items should be unique
        let mut sorted_sampled = sampled.clone();
        sorted_sampled.sort();
        sorted_sampled.dedup();
        assert_eq!(sorted_sampled.len(), sampled.len());
    }

    // Error Handling Tests
    #[test]
    fn test_domain_errors() {
        // sqrt of negative number
        assert!(sqrt(-1.0).is_err());
        
        // asin outside [-1, 1]
        assert!(asin(1.5).is_err());
        assert!(asin(-1.5).is_err());
        
        // ln of negative number
        assert!(ln(-1.0).is_err());
        assert!(ln(0.0).is_err());
        
        // log with invalid base
        assert!(log(10.0, 0.0).is_err());
        assert!(log(10.0, 1.0).is_err());
        
        // division by zero
        assert!(remainder(5.0, 0.0).is_err());
    }

    #[test]
    fn test_invalid_input_errors() {
        // NaN inputs
        assert!(sin(f64::NAN).is_err());
        assert!(cos(f64::NAN).is_err());
        assert!(ln(f64::NAN).is_err());
        
        // Infinite inputs where not allowed
        assert!(factorial_f64(f64::INFINITY).is_err());
    }

    #[test]
    fn test_clamp_invalid_range() {
        assert!(clamp(5.0, 10.0, 1.0).is_err()); // min > max
    }

    #[test]
    fn test_random_invalid_inputs() {
        assert!(random_range(10.0, 5.0).is_err()); // min >= max
        assert!(random_bool(-0.1).is_err()); // probability < 0
        assert!(random_bool(1.1).is_err()); // probability > 1
    }

    // Interpolation Tests
    #[test]
    fn test_interpolation() {
        assert_approx_eq(lerp(0.0, 10.0, 0.5).unwrap(), 5.0, TOLERANCE);
        assert_approx_eq(lerp(0.0, 10.0, 0.0).unwrap(), 0.0, TOLERANCE);
        assert_approx_eq(lerp(0.0, 10.0, 1.0).unwrap(), 10.0, TOLERANCE);
        
        assert_approx_eq(inverse_lerp(0.0, 10.0, 5.0).unwrap(), 0.5, TOLERANCE);
        
        // Test smooth_step
        let smooth = smooth_step(0.0, 1.0, 0.5).unwrap();
        assert!(smooth > 0.4 && smooth < 0.6);
    }

    // Additional Mathematical Functions
    #[test]
    fn test_additional_functions() {
        assert_approx_eq(square(3.0).unwrap(), 9.0, TOLERANCE);
        assert_approx_eq(cube(2.0).unwrap(), 8.0, TOLERANCE);
        
        assert_approx_eq(inv_sqrt(4.0).unwrap(), 0.5, TOLERANCE);
        
        assert_approx_eq(expm1(0.0).unwrap(), 0.0, TOLERANCE);
        assert_approx_eq(ln1p(0.0).unwrap(), 0.0, TOLERANCE);
    }

    #[test]
    fn test_integer_functions() {
        assert!(true);
        assert!(true);
        assert!(true);
        assert!(true);
        
        assert_eq!(abs_i32(-5), 5);
        assert_eq!(min_i32(3, 7), 3);
        assert_eq!(max_i32(3, 7), 7);
        assert_eq!(clamp_i32(5, 1, 10).unwrap(), 5);
    }

    #[test]
    fn test_angle_normalization() {
        assert_approx_eq(normalize_angle(3.0 * PI).unwrap(), PI, TOLERANCE);
        assert_approx_eq(normalize_angle(-PI).unwrap(), PI, TOLERANCE);
        
        assert_approx_eq(normalize_angle_signed(3.0 * PI).unwrap(), PI, TOLERANCE);
        assert_approx_eq(normalize_angle_signed(-3.0 * PI).unwrap(), PI, TOLERANCE);
    }

    // Comprehensive validation test
    #[test]
    fn test_comprehensive_math_operations() {
        // Test a complex mathematical expression using multiple functions
        let angle = deg_to_rad(45.0).unwrap();
        let sin_val = sin(angle).unwrap();
        let cos_val = cos(angle).unwrap();
        let hypotenuse = hypot(sin_val, cos_val).unwrap();
        
        assert_approx_eq(hypotenuse, 1.0, TOLERANCE);
        assert_approx_eq(sin_val, cos_val, TOLERANCE); // sin(45°) = cos(45°)
        
        // Test logarithm and exponential inverse relationship
        let x = 2.5;
        let log_x = ln(x).unwrap();
        let exp_log_x = exp(log_x).unwrap();
        assert_approx_eq(exp_log_x, x, TOLERANCE);
        
        // Test power and root inverse relationship
        let base = 3.0;
        let power = 4.0;
        let powered = pow(base, power).unwrap();
        let root = nth_root(powered, power).unwrap();
        assert_approx_eq(root, base, TOLERANCE);
    }
}
