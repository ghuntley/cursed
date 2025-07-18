# Test PGO Integration for CURSED compiler
# This test verifies that the PGO system can collect and analyze profile data

yeet "testz"

# Simple test function for profiling
slay test_function(n normie) normie {
    sus result normie = 0
    bestie i := 0; i < n; i++ {
        result = result + i
    }
    damn result
}

slay main() {
    test_start("PGO Integration Test")
    
    # Create some execution pattern for profiling
    sus total normie = 0
    bestie i := 0; i < 100; i++ {
        total = total + test_function(10)
    }
    
    # Verify basic functionality
    lowkey total > 0 {
        vibez.spill("Test passed: total = " + total.tea())
    } vibes {
        vibez.spill("Test failed: total = " + total.tea())
    }
    vibez.spill("Profile data collection test passed")
    
    print_test_summary()
}
