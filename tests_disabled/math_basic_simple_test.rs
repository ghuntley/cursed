/// Simple test for basic mathematical functions
/// Tests only essential functions that are guaranteed to exist

use cursed::stdlib::math::basic::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math_functions() {
        // Test abs function
        assert_eq!(abs(5.0), 5.0);
        assert_eq!(abs(-5.0), 5.0);
        assert_eq!(abs(0.0), 0.0);
        
        // Test min/max functions
        assert_eq!(min(3.0, 5.0), 3.0);
        assert_eq!(max(3.0, 5.0), 5.0);
        
        // Test sign function
        assert_eq!(sign(5.0), 1.0);
        assert_eq!(sign(-5.0), -1.0);
        assert_eq!(sign(0.0), 0.0);
    }

    #[test]
    fn test_clamp_function() {
        // Test valid clamping
        let result = clamp(5.0, 1.0, 10.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
        
        let result = clamp(-5.0, 1.0, 10.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1.0);
        
        let result = clamp(15.0, 1.0, 10.0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10.0);
    }

    #[test]
    fn test_floor_function() {
        let result = floor(3.7);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3.0);
        
        let result = floor(-3.7);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), -4.0);
    }
}
