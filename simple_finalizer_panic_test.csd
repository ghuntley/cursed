# Simple test to verify P0 Issue #9 fix
sus test_count drip = 0

slay test_basic_operations() drip {
    test_count = test_count + 1
    damn test_count
}

# Test basic functionality
sus result drip = test_basic_operations()
damn result
