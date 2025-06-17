/// Math Basic Functional Tests
/// 
/// Tests basic mathematical operations and functions to ensure
/// the math library provides correct functionality.

use cursed::stdlib::math::basic::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        // Addition
        assert_eq!(2.0 + 3.0, 5.0);
        assert_eq!((-2.0) + 3.0, 1.0);
        
        // Subtraction
        assert_eq!(5.0 - 3.0, 2.0);
        assert_eq!(3.0 - 5.0, -2.0);
        
        // Multiplication
        assert_eq!(4.0 * 5.0, 20.0);
        assert_eq!((-2.0) * 3.0, -6.0);
        
        // Division
        assert_eq!(10.0 / 2.0, 5.0);
        assert_eq!(7.0 / 2.0, 3.5);
        
        println!("Basic arithmetic operations test passed");
    }

    #[test]
    fn test_abs_function() {
        assert_eq!(abs(5.0), 5.0);
        assert_eq!(abs(-5.0), 5.0);
        assert_eq!(abs(0.0), 0.0);
        assert_eq!(abs(-0.0), 0.0);
        
        // Edge cases
        assert_eq!(abs(f64::INFINITY), f64::INFINITY);
        assert_eq!(abs(f64::NEG_INFINITY), f64::INFINITY);
        assert!(abs(f64::NAN).is_nan());
        
        println!("Absolute value function test passed");
    }

    #[test]
    fn test_min_max_functions() {
        // Basic cases
        assert_eq!(min(5.0, 3.0), 3.0);
        assert_eq!(max(5.0, 3.0), 5.0);
        
        // Equal values
        assert_eq!(min(5.0, 5.0), 5.0);
        assert_eq!(max(5.0, 5.0), 5.0);
        
        // Negative numbers
        assert_eq!(min(-5.0, -3.0), -5.0);
        assert_eq!(max(-5.0, -3.0), -3.0);
        
        // Mixed signs
        assert_eq!(min(-5.0, 3.0), -5.0);
        assert_eq!(max(-5.0, 3.0), 3.0);
        
        println!("Min/max functions test passed");
    }

    #[test]
    fn test_power_functions() {
        // Basic powers
        assert_eq!(pow(2.0, 3.0).unwrap(), 8.0);
        assert_eq!(pow(5.0, 0.0).unwrap(), 1.0);
        assert_eq!(pow(1.0, 100.0).unwrap(), 1.0);
        
        // Square and cube
        assert_eq!(square(5.0).unwrap(), 25.0);
        assert_eq!(cube(3.0).unwrap(), 27.0);
        
        // Special powers
        assert_eq!(pow2(3.0).unwrap(), 8.0);   // 2^3
        assert_eq!(pow10(2.0).unwrap(), 100.0); // 10^2
        
        println!("Power functions test passed");
    }

    #[test]
    fn test_root_functions() {
        // Square root
        assert_eq!(sqrt(16.0).unwrap(), 4.0);
        assert_eq!(sqrt(25.0).unwrap(), 5.0);
        assert_eq!(sqrt(0.0).unwrap(), 0.0);
        
        // Cube root  
        assert_eq!(cbrt(27.0).unwrap(), 3.0);
        assert_eq!(cbrt(8.0).unwrap(), 2.0);
        assert_eq!(cbrt(-8.0).unwrap(), -2.0);
        
        // Nth root
        assert_eq!(nth_root(16.0, 4.0).unwrap(), 2.0);
        assert_eq!(nth_root(32.0, 5.0).unwrap(), 2.0);
        
        println!("Root functions test passed");
    }

    #[test]
    fn test_rounding_functions() {
        // Floor
        assert_eq!(floor(3.7), 3.0);
        assert_eq!(floor(-3.7), -4.0);
        assert_eq!(floor(5.0), 5.0);
        
        // Ceiling
        assert_eq!(ceil(3.2), 4.0);
        assert_eq!(ceil(-3.2), -3.0);
        assert_eq!(ceil(5.0), 5.0);
        
        // Round
        assert_eq!(round(3.4), 3.0);
        assert_eq!(round(3.6), 4.0);
        assert_eq!(round(-3.6), -4.0);
        
        // Truncate
        assert_eq!(math_truncate(3.7), 3.0);
        assert_eq!(math_truncate(-3.7), -3.0);
        
        println!("Rounding functions test passed");
    }

    #[test]
    fn test_comparison_functions() {
        // Zero checking
        assert!(is_zero(0.0));
        assert!(!is_zero(0.1));
        assert!(is_zero(-0.0));
        
        // Equality with epsilon
        assert!(is_equal(1.0, 1.0001, 0.001));
        assert!(!is_equal(1.0, 1.1, 0.01));
        assert!(is_equal(3.14159, 3.14160, 0.0001));
        
        println!("Comparison functions test passed");
    }

    #[test]
    fn test_interpolation_functions() {
        // Linear interpolation
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
        
        // Inverse linear interpolation
        assert_eq!(inverse_lerp(0.0, 10.0, 5.0), 0.5);
        assert_eq!(inverse_lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(inverse_lerp(0.0, 10.0, 10.0), 1.0);
        
        // Smooth step
        let smooth_half = smooth_step(0.0, 10.0, 5.0);
        assert!(smooth_half > 0.4 && smooth_half < 0.6, "Smooth step should be around 0.5");
        
        println!("Interpolation functions test passed");
    }

    #[test]
    fn test_geometric_functions() {
        // Pythagorean theorem
        assert_eq!(hypot(3.0, 4.0).unwrap(), 5.0);
        assert_eq!(hypot(5.0, 12.0).unwrap(), 13.0);
        assert_eq!(hypot(0.0, 5.0).unwrap(), 5.0);
        
        // Reciprocal
        assert_eq!(reciprocal(2.0).unwrap(), 0.5);
        assert_eq!(reciprocal(0.5).unwrap(), 2.0);
        assert_eq!(reciprocal(-2.0).unwrap(), -0.5);
        
        println!("Geometric functions test passed");
    }

    #[test]
    fn test_statistical_functions() {
        // Average
        assert_eq!(average(4.0, 6.0), 5.0);
        assert_eq!(average(0.0, 10.0), 5.0);
        assert_eq!(average(-5.0, 5.0), 0.0);
        
        // Geometric mean
        assert_eq!(geometric_mean(4.0, 9.0).unwrap(), 6.0);
        assert_eq!(geometric_mean(1.0, 16.0).unwrap(), 4.0);
        
        // Harmonic mean
        let harmonic = harmonic_mean(2.0, 8.0).unwrap();
        assert!((harmonic - 3.2).abs() < 0.1, "Harmonic mean should be approximately 3.2");
        
        println!("Statistical functions test passed");
    }

    #[test]
    fn test_integer_operations() {
        // GCD
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(0, 5), 5);
        
        // LCM
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(3, 7), 21);
        assert_eq!(lcm(12, 8), 24);
        
        // Even/odd checking
        assert!(is_even(4));
        assert!(!is_even(5));
        assert!(is_odd(7));
        assert!(!is_odd(8));
        
        println!("Integer operations test passed");
    }

    #[test]
    fn test_clamping_functions() {
        // Float clamping
        assert_eq!(clamp(5.0, 1.0, 10.0), 5.0);
        assert_eq!(clamp(-5.0, 1.0, 10.0), 1.0);
        assert_eq!(clamp(15.0, 1.0, 10.0), 10.0);
        
        // Integer clamping
        assert_eq!(clamp_i64(5, 1, 10), 5);
        assert_eq!(clamp_i64(-5, 1, 10), 1);
        assert_eq!(clamp_i64(15, 1, 10), 10);
        
        println!("Clamping functions test passed");
    }

    #[test]
    fn test_range_mapping() {
        // Map from one range to another
        assert_eq!(map_range(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
        assert_eq!(map_range(0.0, 0.0, 10.0, 50.0, 150.0), 50.0);
        assert_eq!(map_range(10.0, 0.0, 10.0, 50.0, 150.0), 150.0);
        
        // Reverse mapping
        assert_eq!(map_range(7.5, 0.0, 10.0, 100.0, 0.0), 25.0);
        
        println!("Range mapping test passed");
    }

    #[test]
    fn test_precision_functions() {
        // Round to specific decimal places
        assert_eq!(round_to_decimals(3.14159, 2), 3.14);
        assert_eq!(round_to_decimals(3.14159, 4), 3.1416);
        assert_eq!(round_to_decimals(123.456, 1), 123.5);
        
        println!("Precision functions test passed");
    }

    #[test]
    fn test_sign_function() {
        assert_eq!(sign(5.0), 1.0);
        assert_eq!(sign(-5.0), -1.0);
        assert_eq!(sign(0.0), 0.0);
        assert_eq!(sign(-0.0), 0.0);
        
        println!("Sign function test passed");
    }

    #[test]
    fn test_fractional_part() {
        assert_eq!(fract(3.14), 0.14);
        assert_eq!(fract(-3.14), 0.86); // or -0.14 depending on implementation
        assert_eq!(fract(5.0), 0.0);
        
        println!("Fractional part test passed");
    }

    #[test]
    fn test_error_handling() {
        // Division by zero in reciprocal
        assert!(reciprocal(0.0).is_err());
        
        // Negative square root
        assert!(sqrt(-1.0).is_err());
        
        // Invalid power operations
        assert!(pow(0.0, -1.0).is_err()); // 0^(-1) is undefined
        
        // Invalid nth root
        assert!(nth_root(-8.0, 2.0).is_err()); // Even root of negative number
        
        println!("Error handling test passed");
    }

    #[test]
    fn test_edge_cases() {
        // Infinity handling
        assert_eq!(abs(f64::INFINITY), f64::INFINITY);
        assert_eq!(abs(f64::NEG_INFINITY), f64::INFINITY);
        
        // NaN handling
        assert!(abs(f64::NAN).is_nan());
        assert!(is_zero(f64::NAN) == false);
        
        // Very small numbers
        assert!(is_zero(1e-16));
        assert!(!is_zero(1e-10));
        
        // Very large numbers
        let large = 1e100;
        assert_eq!(abs(large), large);
        assert_eq!(abs(-large), large);
        
        println!("Edge cases test passed");
    }

    #[test]
    fn test_mathematical_properties() {
        let x = 5.0;
        let y = 3.0;
        
        // Commutative property
        assert_eq!(min(x, y), min(y, x));
        assert_eq!(max(x, y), max(y, x));
        
        // Identity properties
        assert_eq!(x + 0.0, x);
        assert_eq!(x * 1.0, x);
        
        // Inverse properties
        let sqrt_x = sqrt(x).unwrap();
        assert!((square(sqrt_x).unwrap() - x).abs() < 1e-10);
        
        let cube_x = cube(x).unwrap();
        assert!((cbrt(cube_x).unwrap() - x).abs() < 1e-10);
        
        println!("Mathematical properties test passed");
    }
}
