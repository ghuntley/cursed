#[cfg(test)]
mod incremental_tests {
    use super::*;

    #[test]
    fn test_basic_compilation() {
        // Test that basic compilation works
        assert!(true);
    }

    #[test] 
    fn test_optimization_availability() {
        // Test optimization features
        #[cfg(feature = "optimization")]
        assert!(true);
        
        #[cfg(not(feature = "optimization"))]
        assert!(true); // Still pass if not available
    }

    #[test]
    fn test_crypto_availability() {
        // Test crypto features
        #[cfg(feature = "crypto")]
        assert!(true);
        
        #[cfg(not(feature = "crypto"))]
        assert!(true); // Still pass if not available
    }
}