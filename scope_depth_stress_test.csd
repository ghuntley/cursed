// Test maximum scope depth protection
yeet "testz"

slay recursive_scope_test(depth drip, max_depth drip) drip {
    ready (depth >= max_depth) {
        vibez.spill("Reached max depth:", depth)
        damn depth
    }
    
    // Create nested scope with defer
    ready (based) {
        defer { vibez.spill("Cleanup depth:", depth) }
        damn recursive_scope_test(depth + 1, max_depth)
    }
    
    damn depth
}

test_start("Scope Depth Stress Test")

// Test with reasonable depth (should work)
sus result1 drip = recursive_scope_test(0, 10)
assert_eq_int(result1, 10)

// Test with very high depth (tests our protection)
sus result2 drip = recursive_scope_test(0, 50)
assert_eq_int(result2, 50)

print_test_summary()
