# Test to validate pattern matching fix
# Run this BEFORE and AFTER applying the fix to see the difference

vibez.spill("=== PATTERN MATCHING FIX VALIDATION ===")
vibez.spill("")

# Test Case 1: Simple integer matching
vibez.spill("Test 1: x = 3, should only match branch 3")
sus x drip = 3
ready (x) {
    1 => vibez.spill("  ERROR: Branch 1 executed")
    2 => vibez.spill("  ERROR: Branch 2 executed") 
    3 => vibez.spill("  SUCCESS: Branch 3 executed")
    4 => vibez.spill("  ERROR: Branch 4 executed")
    _ => vibez.spill("  ERROR: Wildcard executed")
}
vibez.spill("")

# Test Case 2: Wildcard matching
vibez.spill("Test 2: y = 99, should only match wildcard")
sus y drip = 99
ready (y) {
    1 => vibez.spill("  ERROR: Branch 1 executed")
    2 => vibez.spill("  ERROR: Branch 2 executed")
    _ => vibez.spill("  SUCCESS: Wildcard executed")
}
vibez.spill("")

# Test Case 3: First match should win
vibez.spill("Test 3: z = 7, first match should win")
sus z drip = 7
ready (z) {
    7 => vibez.spill("  SUCCESS: First 7 executed")
    7 => vibez.spill("  ERROR: Second 7 executed")
    _ => vibez.spill("  ERROR: Wildcard executed")
}
vibez.spill("")

vibez.spill("=== EXPECTED RESULTS AFTER FIX ===")
vibez.spill("Should only see SUCCESS messages, no ERROR messages")
vibez.spill("If you see ERROR messages, the fix is not yet applied")
