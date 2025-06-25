/// Tests for MathZ module - Mathematical functions with CURSED types and Gen Z naming
/// 
/// This test suite validates the comprehensive mathematical functionality
/// provided by the MathZ module, ensuring all CURSED type operations work
/// correctly with Gen Z naming conventions.

#[cfg(test)]
mod tests {
    use cursed::stdlib::mathz::*;
    use std::f64::consts;

    #[test]
    fn test_normie_operations() {
        // Test basic normie (i32) operations
        assert_eq!(abs_normie(-42), 42);
        assert_eq!(abs_normie(42), 42);
        assert_eq!(abs_normie(0), 0);
        
        assert_eq!(min_normie(10, 20), 10);
        assert_eq!(min_normie(20, 10), 10);
        assert_eq!(min_normie(-5, 5), -5);
        
        assert_eq!(max_normie(10, 20), 20);
        assert_eq!(max_normie(20, 10), 20);
        assert_eq!(max_normie(-5, 5), 5);
        
        assert_eq!(clamp_normie(25, 10, 20), 20);
        assert_eq!(clamp_normie(5, 10, 20), 10);
        assert_eq!(clamp_normie(15, 10, 20), 15);
        
        assert_eq!(sign_normie(-42), -1);
        assert_eq!(sign_normie(42), 1);
        assert_eq!(sign_normie(0), 0);
        
        assert!(is_even_normie(42));
        assert!(!is_even_normie(43));
        assert!(is_even_normie(0));
        
        assert!(!is_odd_normie(42));
        assert!(is_odd_normie(43));
        assert!(!is_odd_normie(0));
        
        assert_eq!(gcd_normie(12, 8), 4);
        assert_eq!(gcd_normie(17, 13), 1); // Coprime
        assert_eq!(gcd_normie(0, 5), 5);
        
        assert_eq!(lcm_normie(12, 8), 24);
        assert_eq!(lcm_normie(3, 5), 15);
        assert_eq!(lcm_normie(0, 5), 0);
    }

    #[test]
    fn test_thicc_operations() {
        // Test basic thicc (i64) operations
        let big_num: Thicc = 9223372036854775807; // i64::MAX
        
        assert_eq!(abs_thicc(-big_num), big_num);
        assert_eq!(abs_thicc(big_num), big_num);
        assert_eq!(abs_thicc(0), 0);
        
        assert_eq!(min_thicc(1000000000, 2000000000), 1000000000);
        assert_eq!(max_thicc(1000000000, 2000000000), 2000000000);
        
        assert_eq!(clamp_thicc(2500000000, 1000000000, 2000000000), 2000000000);
        assert_eq!(clamp_thicc(500000000, 1000000000, 2000000000), 1000000000);
        assert_eq!(clamp_thicc(1500000000, 1000000000, 2000000000), 1500000000);
        
        assert_eq!(sign_thicc(-1000000000), -1);
        assert_eq!(sign_thicc(1000000000), 1);
        assert_eq!(sign_thicc(0), 0);
        
        assert!(is_even_thicc(1000000000));
        assert!(!is_even_thicc(1000000001));
        
        assert!(!is_odd_thicc(1000000000));
        assert!(is_odd_thicc(1000000001));
    }

    #[test]
    fn test_chonky_basic_operations() {
        // Test basic chonky (f64) operations
        assert_eq!(abs_chonky(-3.14), 3.14);
        assert_eq!(abs_chonky(3.14), 3.14);
        assert_eq!(abs_chonky(0.0), 0.0);
        
        assert_eq!(min_chonky(3.14, 2.71), 2.71);
        assert_eq!(max_chonky(3.14, 2.71), 3.14);
        
        assert_eq!(clamp_chonky(5.0, 1.0, 4.0), 4.0);
        assert_eq!(clamp_chonky(0.5, 1.0, 4.0), 1.0);
        assert_eq!(clamp_chonky(2.5, 1.0, 4.0), 2.5);
        
        assert_eq!(sign_chonky(-3.14), -1.0);
        assert_eq!(sign_chonky(3.14), 1.0);
        assert_eq!(sign_chonky(0.0), 0.0);
        
        assert_eq!(floor_chonky(3.7), 3.0);
        assert_eq!(floor_chonky(-3.7), -4.0);
        
        assert_eq!(ceil_chonky(3.2), 4.0);
        assert_eq!(ceil_chonky(-3.2), -3.0);
        
        assert_eq!(round_chonky(3.7), 4.0);
        assert_eq!(round_chonky(3.2), 3.0);
        assert_eq!(round_chonky(-3.7), -4.0);
        
        assert_eq!(truncate_chonky(3.7), 3.0);
        assert_eq!(truncate_chonky(-3.7), -3.0);
        
        assert!((fract_chonky(3.7) - 0.7).abs() < 1e-10);
        assert!((fract_chonky(-3.7) - (-0.7)).abs() < 1e-10);
        
        assert!(is_zero_chonky(0.0));
        assert!(!is_zero_chonky(0.0001));
        assert!(!is_zero_chonky(-0.0001));
        
        assert!(is_equal_chonky(3.14159, 3.14159, 0.00001));
        assert!(is_equal_chonky(3.14159, 3.14160, 0.0001));
        assert!(!is_equal_chonky(3.14159, 3.14170, 0.00001));
    }

    #[test]
    fn test_power_and_root_functions() {
        // Test power functions
        assert!((pow_chonky(2.0, 3.0).unwrap() - 8.0).abs() < 1e-10);
        assert!((pow_chonky(4.0, 0.5).unwrap() - 2.0).abs() < 1e-10);
        
        assert!((powi_chonky(2.0, 3).unwrap() - 8.0).abs() < 1e-10);
        assert!((powi_chonky(5.0, 0).unwrap() - 1.0).abs() < 1e-10);
        
        assert_eq!(square_chonky(5.0), 25.0);
        assert_eq!(square_chonky(-5.0), 25.0);
        assert_eq!(square_chonky(0.0), 0.0);
        
        assert_eq!(cube_chonky(3.0), 27.0);
        assert_eq!(cube_chonky(-3.0), -27.0);
        assert_eq!(cube_chonky(0.0), 0.0);
        
        // Test root functions
        assert!((sqrt_chonky(25.0).unwrap() - 5.0).abs() < 1e-10);
        assert!((sqrt_chonky(0.0).unwrap() - 0.0).abs() < 1e-10);
        assert!(sqrt_chonky(-1.0).is_err()); // Domain error
        
        assert!((cbrt_chonky(27.0).unwrap() - 3.0).abs() < 1e-10);
        assert!((cbrt_chonky(-27.0).unwrap() - (-3.0)).abs() < 1e-10);
        
        assert!((nth_root_chonky(16.0, 4.0).unwrap() - 2.0).abs() < 1e-10);
        assert!((nth_root_chonky(32.0, 5.0).unwrap() - 2.0).abs() < 1e-10);
        
        assert_eq!(hypot_chonky(3.0, 4.0), 5.0);
        assert_eq!(hypot_chonky(0.0, 5.0), 5.0);
        
        assert!((reciprocal_chonky(4.0).unwrap() - 0.25).abs() < 1e-10);
        assert!((reciprocal_chonky(0.5).unwrap() - 2.0).abs() < 1e-10);
        assert!(reciprocal_chonky(0.0).is_err()); // Division by zero
    }

    #[test]
    fn test_logarithmic_and_exponential() {
        // Test logarithmic functions
        assert!((ln_chonky(E_CHONKY).unwrap() - 1.0).abs() < 1e-10);
        assert!((ln_chonky(1.0).unwrap() - 0.0).abs() < 1e-10);
        assert!(ln_chonky(0.0).is_err()); // Domain error
        assert!(ln_chonky(-1.0).is_err()); // Domain error
        
        assert!((log10_chonky(100.0).unwrap() - 2.0).abs() < 1e-10);
        assert!((log10_chonky(1.0).unwrap() - 0.0).abs() < 1e-10);
        assert!(log10_chonky(0.0).is_err()); // Domain error
        
        assert!((log2_chonky(8.0).unwrap() - 3.0).abs() < 1e-10);
        assert!((log2_chonky(1.0).unwrap() - 0.0).abs() < 1e-10);
        
        assert!((log_chonky(27.0, 3.0).unwrap() - 3.0).abs() < 1e-10);
        assert!((log_chonky(100.0, 10.0).unwrap() - 2.0).abs() < 1e-10);
        
        // Test exponential functions
        assert!((exp_chonky(1.0).unwrap() - E_CHONKY).abs() < 1e-10);
        assert!((exp_chonky(0.0).unwrap() - 1.0).abs() < 1e-10);
        
        assert!((exp2_chonky(3.0).unwrap() - 8.0).abs() < 1e-10);
        assert!((exp2_chonky(0.0).unwrap() - 1.0).abs() < 1e-10);
        
        assert!((exp10_chonky(2.0).unwrap() - 100.0).abs() < 1e-10);
        assert!((exp10_chonky(0.0).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_trigonometric_functions() {
        // Test basic trigonometric functions
        assert!((sin_chonky(PI_CHONKY / 2.0).unwrap() - 1.0).abs() < 1e-10);
        assert!((sin_chonky(0.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((sin_chonky(PI_CHONKY).unwrap() - 0.0).abs() < 1e-10);
        
        assert!((cos_chonky(0.0).unwrap() - 1.0).abs() < 1e-10);
        assert!((cos_chonky(PI_CHONKY / 2.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((cos_chonky(PI_CHONKY).unwrap() - (-1.0)).abs() < 1e-10);
        
        assert!((tan_chonky(PI_CHONKY / 4.0).unwrap() - 1.0).abs() < 1e-10);
        assert!((tan_chonky(0.0).unwrap() - 0.0).abs() < 1e-10);
        
        // Test inverse trigonometric functions
        assert!((asin_chonky(1.0).unwrap() - PI_CHONKY / 2.0).abs() < 1e-10);
        assert!((asin_chonky(0.0).unwrap() - 0.0).abs() < 1e-10);
        assert!(asin_chonky(2.0).is_err()); // Domain error
        
        assert!((acos_chonky(1.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((acos_chonky(0.0).unwrap() - PI_CHONKY / 2.0).abs() < 1e-10);
        assert!(acos_chonky(2.0).is_err()); // Domain error
        
        assert!((atan_chonky(1.0).unwrap() - PI_CHONKY / 4.0).abs() < 1e-10);
        assert!((atan_chonky(0.0).unwrap() - 0.0).abs() < 1e-10);
        
        assert!((atan2_chonky(1.0, 1.0).unwrap() - PI_CHONKY / 4.0).abs() < 1e-10);
        assert!((atan2_chonky(0.0, 1.0).unwrap() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_hyperbolic_functions() {
        // Test hyperbolic functions
        assert!((sinh_chonky(0.0).unwrap() - 0.0).abs() < 1e-10);
        assert!((sinh_chonky(1.0).unwrap() - (E_CHONKY - 1.0/E_CHONKY) / 2.0).abs() < 1e-10);
        
        assert!((cosh_chonky(0.0).unwrap() - 1.0).abs() < 1e-10);
        assert!((cosh_chonky(1.0).unwrap() - (E_CHONKY + 1.0/E_CHONKY) / 2.0).abs() < 1e-10);
        
        assert!((tanh_chonky(0.0).unwrap() - 0.0).abs() < 1e-10);
        // tanh(inf) = 1, tanh(-inf) = -1
        assert!((tanh_chonky(100.0).unwrap() - 1.0).abs() < 1e-10);
        assert!((tanh_chonky(-100.0).unwrap() - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_mathematical_constants() {
        // Test that constants are approximately correct
        assert!((PI_CHONKY - consts::PI).abs() < 1e-10);
        assert!((TAU_CHONKY - 2.0 * consts::PI).abs() < 1e-10);
        assert!((E_CHONKY - consts::E).abs() < 1e-10);
        assert!((PHI_CHONKY - 1.618033988749895).abs() < 1e-10);
        assert!((SQRT_2_CHONKY - consts::SQRT_2).abs() < 1e-10);
        assert!((LN_2_CHONKY - consts::LN_2).abs() < 1e-10);
        assert!((LN_10_CHONKY - consts::LN_10).abs() < 1e-10);
    }

    #[test]
    fn test_random_functions() {
        // Test that random functions don't panic and return reasonable values
        let r = random_chonky();
        assert!(r >= 0.0 && r < 1.0);
        
        let r_range = random_range_chonky(1.0, 10.0);
        assert!(r_range >= 1.0 && r_range <= 10.0);
        
        let r_int = random_normie();
        assert!(r_int >= i32::MIN && r_int <= i32::MAX);
        
        let r_bool = random_bool();
        assert!(r_bool == true || r_bool == false);
        
        // Test that multiple calls give different results (probabilistically)
        let r1 = random_chonky();
        let r2 = random_chonky();
        // Very unlikely they're exactly equal
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_interpolation_functions() {
        // Test linear interpolation
        assert_eq!(lerp_chonky(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp_chonky(0.0, 10.0, 1.0), 10.0);
        assert_eq!(lerp_chonky(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp_chonky(5.0, 15.0, 0.25), 7.5);
        
        // Test inverse linear interpolation
        assert_eq!(inverse_lerp_chonky(0.0, 10.0, 0.0), 0.0);
        assert_eq!(inverse_lerp_chonky(0.0, 10.0, 10.0), 1.0);
        assert_eq!(inverse_lerp_chonky(0.0, 10.0, 5.0), 0.5);
        assert_eq!(inverse_lerp_chonky(5.0, 15.0, 7.5), 0.25);
        
        // Test smooth step (at boundaries and midpoint)
        assert_eq!(smooth_step_chonky(0.0, 1.0, 0.0), 0.0);
        assert_eq!(smooth_step_chonky(0.0, 1.0, 1.0), 1.0);
        assert_eq!(smooth_step_chonky(0.0, 1.0, 0.5), 0.5);
        
        // Test smoother step (at boundaries and midpoint)
        assert_eq!(smoother_step_chonky(0.0, 1.0, 0.0), 0.0);
        assert_eq!(smoother_step_chonky(0.0, 1.0, 1.0), 1.0);
        assert_eq!(smoother_step_chonky(0.0, 1.0, 0.5), 0.5);
        
        // Test map range
        assert_eq!(map_range_chonky(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
        assert_eq!(map_range_chonky(0.0, 0.0, 10.0, 100.0, 200.0), 100.0);
        assert_eq!(map_range_chonky(10.0, 0.0, 10.0, 100.0, 200.0), 200.0);
    }

    #[test]
    fn test_utility_functions() {
        // Test validity checking
        assert!(is_valid_chonky(3.14));
        assert!(is_valid_chonky(0.0));
        assert!(is_valid_chonky(-3.14));
        assert!(!is_valid_chonky(Chonky::NAN));
        assert!(!is_valid_chonky(Chonky::INFINITY));
        assert!(!is_valid_chonky(Chonky::NEG_INFINITY));
        
        // Test decimal rounding
        assert!((round_to_decimals_chonky(3.14159, 2).unwrap() - 3.14).abs() < 1e-10);
        assert!((round_to_decimals_chonky(3.14159, 4).unwrap() - 3.1416).abs() < 1e-10);
        assert!((round_to_decimals_chonky(3.0, 2).unwrap() - 3.0).abs() < 1e-10);
        
        // Test average calculation
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(average_chonky(&values).unwrap(), 3.0);
        
        let values = [10.0, 20.0];
        assert_eq!(average_chonky(&values).unwrap(), 15.0);
        
        assert!(average_chonky(&[]).is_err()); // Empty slice
        
        // Test geometric mean
        let values = [1.0, 4.0, 9.0];
        let expected = (1.0 * 4.0 * 9.0).powf(1.0/3.0);
        assert!((geometric_mean_chonky(&values).unwrap() - expected).abs() < 1e-10);
        
        assert!(geometric_mean_chonky(&[]).is_err()); // Empty slice
        assert!(geometric_mean_chonky(&[1.0, -1.0]).is_err()); // Negative value
        assert!(geometric_mean_chonky(&[1.0, 0.0]).is_err()); // Zero value
        
        // Test harmonic mean
        let values = [1.0, 2.0, 4.0];
        let expected = 3.0 / (1.0/1.0 + 1.0/2.0 + 1.0/4.0);
        assert!((harmonic_mean_chonky(&values).unwrap() - expected).abs() < 1e-10);
        
        assert!(harmonic_mean_chonky(&[]).is_err()); // Empty slice
        assert!(harmonic_mean_chonky(&[1.0, 0.0]).is_err()); // Zero value
    }

    #[test]
    fn test_module_functions() {
        // Test module initialization
        assert!(init_mathz().is_ok());
        
        // Test module statistics
        let stats = get_mathz_stats();
        assert!(stats.contains_key("version"));
        assert!(stats.contains_key("functions"));
        assert!(stats.contains_key("features"));
        assert!(stats.contains_key("types"));
        
        assert_eq!(stats.get("version").unwrap(), "1.0.0");
        assert!(stats.get("functions").unwrap().contains("70+"));
        assert!(stats.get("features").unwrap().contains("CURSED types"));
        assert!(stats.get("types").unwrap().contains("normie"));
        assert!(stats.get("types").unwrap().contains("thicc"));
        assert!(stats.get("types").unwrap().contains("chonky"));
    }

    #[test]
    fn test_type_aliases() {
        // Test that type aliases work correctly
        let normie_val: Normie = 42;
        let thicc_val: Thicc = 1000000000;
        let smol_val: Smol = 3.14;
        let chonky_val: Chonky = 3.141592653589793;
        
        assert_eq!(abs_normie(normie_val), 42);
        assert_eq!(abs_thicc(thicc_val), 1000000000);
        assert_eq!(abs_chonky(chonky_val), 3.141592653589793);
        
        // Test type size constraints
        assert_eq!(std::mem::size_of::<Normie>(), 4); // i32
        assert_eq!(std::mem::size_of::<Thicc>(), 8);  // i64
        assert_eq!(std::mem::size_of::<Smol>(), 4);   // f32
        assert_eq!(std::mem::size_of::<Chonky>(), 8); // f64
    }

    #[test]
    fn test_edge_cases() {
        // Test edge cases for normie operations
        assert_eq!(abs_normie(i32::MIN + 1), i32::MAX);
        assert_eq!(min_normie(i32::MIN, i32::MAX), i32::MIN);
        assert_eq!(max_normie(i32::MIN, i32::MAX), i32::MAX);
        
        // Test edge cases for chonky operations
        assert!(is_valid_chonky(f64::MIN));
        assert!(is_valid_chonky(f64::MAX));
        assert!(!is_valid_chonky(f64::NAN));
        assert!(!is_valid_chonky(f64::INFINITY));
        
        // Test zero cases
        assert_eq!(pow_chonky(0.0, 2.0).unwrap(), 0.0);
        assert_eq!(pow_chonky(5.0, 0.0).unwrap(), 1.0);
        assert!(pow_chonky(0.0, -1.0).is_err()); // 0^(-1) is undefined
        
        // Test infinity cases
        assert_eq!(lerp_chonky(0.0, f64::INFINITY, 0.5), f64::INFINITY);
        assert_eq!(map_range_chonky(0.5, 0.0, 1.0, 0.0, f64::INFINITY), f64::INFINITY);
    }
}
