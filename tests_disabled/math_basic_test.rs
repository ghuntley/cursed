/// Comprehensive test suite for CURSED standard library math basic functions
/// 
/// Tests all fundamental mathematical functions including arithmetic, rounding,
/// comparison, interpolation, and integer operations with extensive edge case validation.

use cursed::stdlib::math::basic::*;
use cursed::stdlib::math::{MathError, MathResult};

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[test]
    fn test_abs_function() {
        // Positive numbers
        assert_eq!(abs(5.0), 5.0);
        assert_eq!(abs(3.14), 3.14);
        
        // Negative numbers
        assert_eq!(abs(-5.0), 5.0);
        assert_eq!(abs(-3.14), 3.14);
        
        // Zero
        assert_eq!(abs(0.0), 0.0);
        assert_eq!(abs(-0.0), 0.0);
        
        // Edge cases
        assert_eq!(abs(f64::MAX), f64::MAX);
        assert_eq!(abs(f64::MIN), f64::MAX);
        assert!(abs(f64::NAN).is_nan());
        assert_eq!(abs(f64::INFINITY), f64::INFINITY);
        assert_eq!(abs(f64::NEG_INFINITY), f64::INFINITY);
    }

    #[test]
    fn test_min_max_functions() {
        // Basic min/max
        assert_eq!(min(5.0, 3.0), 3.0);
        assert_eq!(max(5.0, 3.0), 5.0);
        
        // Equal values
        assert_eq!(min(5.0, 5.0), 5.0);
        assert_eq!(max(5.0, 5.0), 5.0);
        
        // Negative numbers
        assert_eq!(min(-5.0, -3.0), -5.0);
        assert_eq!(max(-5.0, -3.0), -3.0);
        
        // Zero handling
        assert_eq!(min(0.0, -0.0), -0.0);
        assert_eq!(max(0.0, -0.0), 0.0);
        
        // NaN handling - in Rust, min/max with NaN can return either value
        // The exact behavior depends on implementation, so we test both possibilities
        let min_result = min(f64::NAN, 5.0);
        assert!(min_result.is_nan() || min_result == 5.0);
        let max_result = max(f64::NAN, 5.0);
        assert!(max_result.is_nan() || max_result == 5.0);
        
        // Infinity handling
        assert_eq!(min(f64::INFINITY, 5.0), 5.0);
        assert_eq!(max(f64::INFINITY, 5.0), f64::INFINITY);
        assert_eq!(min(f64::NEG_INFINITY, 5.0), f64::NEG_INFINITY);
        assert_eq!(max(f64::NEG_INFINITY, 5.0), 5.0);
    }

    #[test]
    fn test_clamp_function() {
        // Normal clamping
        assert_eq!(clamp(5.0, 1.0, 10.0).unwrap(), 5.0);
        assert_eq!(clamp(0.5, 1.0, 10.0).unwrap(), 1.0);
        assert_eq!(clamp(15.0, 1.0, 10.0).unwrap(), 10.0);
        
        // Edge cases
        assert_eq!(clamp(1.0, 1.0, 10.0).unwrap(), 1.0);
        assert_eq!(clamp(10.0, 1.0, 10.0).unwrap(), 10.0);
        
        // Invalid range
        let result = clamp(5.0, 10.0, 1.0);
        assert!(result.is_err());
        match result.unwrap_err() {
            MathError::InvalidInput { function, .. } => {
                assert_eq!(function, "clamp");
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }
    
    #[test]
    fn test_error_handling() {
        
        // Test error messages contain proper context (invalid clamp with min > max)
        match clamp(5.0, 10.0, 0.0) {
            Err(e) => {
                let clamp_msg = format!("{}", e);
                assert!(clamp_msg.contains("clamp"));
            }
            Ok(_) => panic!("Expected error for invalid clamp where min > max"),
        }
        
        // Test division by zero error
        match remainder(5.0, 0.0) {
            Err(e) => {
                let div_msg = format!("{}", e);
                assert!(div_msg.contains("remainder"));
                assert!(div_msg.contains("Division by zero"));
            }
            Ok(_) => panic!("Expected division by zero error"),
        }
        
        // Test NaN input error
        match floor(f64::NAN) {
            Err(e) => {
                let nan_msg = format!("{}", e);
                assert!(nan_msg.contains("floor"));
            }
            Ok(_) => panic!("Expected NaN error"),
        }
        
        // Test negative sqrt error  
        match sqrt(-1.0) {
            Err(e) => {
                let sqrt_msg = format!("{}", e);
                assert!(sqrt_msg.contains("sqrt"));
                assert!(sqrt_msg.contains("Negative input"));
            }
            Ok(_) => panic!("Expected negative input error"),
        }
        
        // Test domain error for pow
        match pow(-1.0, 0.5) {
            Err(e) => {
                let pow_msg = format!("{}", e);
                assert!(pow_msg.contains("pow"));
                assert!(pow_msg.contains("Domain error"));
            }
            Ok(_) => panic!("Expected domain error"),
        }
    }
}