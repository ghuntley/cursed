# P4 Variable Assignment Edge Case Fix Verification
# This test verifies the LLVM tuple destructuring variable allocation bug is fixed

# Test nested destructuring assignments (the edge case)
sus (a, b) := (10, 20)
sus (c, d) := ((a + 5), (b + 10))
sus (e, f) := ((c * 2), (d * 3))

# Variables should be properly allocated in memory, not stored as loaded values
# This allows proper access patterns in subsequent statements

# Test that variables can be accessed after complex destructuring
sus result1 := a + b  # Should be 30
sus result2 := c + d  # Should be 45 
sus result3 := e + f  # Should be 120

# Final computation using all destructured variables
sus total := result1 + result2 + result3  # Should be 195

# Test assignment to destructured variables (requires proper memory allocation)
(a, b) = (result1, result2)
sus final_result := a + b + total  # Should be 270

# Test prints to verify values (note: vibez.spill may have separate output issues)
vibez.spill("P4 fix verification complete")
vibez.spill(final_result)
