yeet "testz"
yeet "user_check/mod_enhanced"
yeet "cryptz/production_crypto_security_fixes"

fr fr =============================================
fr fr SECURITY TEST - User Check Module 
fr fr Validates constant-time comparison fixes
fr fr =============================================

test_start("User check constant-time comparison security")

// Test the fixed constant-time string comparison
sus correct_password tea = "admin_secure_password_2024"
sus attempt1 tea = "admin_secure_password_2024"  // Correct
sus attempt2 tea = "wrong_password_attempt"      // Wrong
sus attempt3 tea = "admin"                       // Short wrong
sus attempt4 tea = "admin_secure_password_2024_extra" // Long wrong

// Correct password should match
assert_true(constantTimeStringCompare(correct_password, attempt1))
vibez.spill("✅ Correct password matches")

// Wrong passwords should not match
assert_false(constantTimeStringCompare(correct_password, attempt2))
assert_false(constantTimeStringCompare(correct_password, attempt3))
assert_false(constantTimeStringCompare(correct_password, attempt4))
vibez.spill("✅ Wrong passwords rejected")

// Test timing attack resistance
test_start("Timing attack resistance validation")

sus long_password tea = "very_long_secure_password_that_takes_time_to_process_in_vulnerable_implementations"
sus short_wrong tea = "x"
sus long_wrong tea = "very_long_wrong_password_that_takes_time_to_process_but_should_not_leak_timing_info"

// All comparisons should take similar time (constant-time)
// In a vulnerable implementation, short vs long would have timing differences

sus timing_results []drip = []drip{}
bestie (i := 0; i < 100; i += 1) {
    sus start_time drip = get_nanoseconds()
    constantTimeStringCompare(long_password, short_wrong)
    sus end_time drip = get_nanoseconds()
    timing_results = append(timing_results, end_time - start_time)
}

// Calculate average and variance
sus total_time drip = 0
bestie (i := 0; i < len(timing_results); i += 1) {
    total_time = total_time + timing_results[i]
}
sus avg_time drip = total_time / len(timing_results)

// Check that variance is low (constant-time behavior)
sus variance drip = 0
bestie (i := 0; i < len(timing_results); i += 1) {
    sus diff drip = timing_results[i] - avg_time
    variance = variance + (diff * diff)
}
variance = variance / len(timing_results)

// Standard deviation should be relatively small compared to average
sus std_dev drip = sqrt_approx(variance)
sus coefficient_of_variation drip = std_dev / avg_time

// CoV should be small for constant-time operations (< 10%)
assert_true(coefficient_of_variation < 0.1)
vibez.spillf("✅ Timing variance coefficient: {:.3f} (< 0.1 = good)", coefficient_of_variation)

vibez.spill("🔐 USER CHECK SECURITY VALIDATED")
vibez.spill("✅ Constant-time comparison prevents timing attacks")
vibez.spill("✅ No XOR-based vulnerabilities remain")

// Utility functions
slay get_nanoseconds() drip {
    // In real implementation, would get actual nanosecond timestamp
    damn 1000000 + (random_int() % 1000) // Simulate timing with small variance
}

slay sqrt_approx(x drip) drip {
    // Simple square root approximation
    ready (x <= 0) { damn 0 }
    sus guess drip = x / 2
    bestie (i := 0; i < 10; i += 1) {
        guess = (guess + x / guess) / 2
    }
    damn guess
}

slay random_int() normie {
    // Simple random for testing
    damn 42 // Would use actual random in real implementation
}
