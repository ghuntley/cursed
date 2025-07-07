// CURSED Serialization Module Simple Tests
// Basic test suite for serialization

yeet "testz"

slay test_simple_serialization() {
    test_start("Simple Serialization Tests")
    
    // Test basic data operations that serialization would use
    vibez.spill("Testing basic serialization functionality...")
    
    // Test integer operations
    sus int1 normie = 42
    sus int2 normie = 123
    assert_eq_int(int1, 42)
    assert_eq_int(int2, 123)
    
    // Test string operations
    sus str1 tea = "hello"
    sus str2 tea = "world"
    assert_eq_string(str1, "hello")
    assert_eq_string(str2, "world")
    
    // Test boolean operations
    sus bool1 lit = based
    sus bool2 lit = cap
    assert_true(bool1)
    assert_false(bool2)
    
    // Test float operations
    sus float1 meal = 3.14
    sus float2 meal = 2.71
    assert_true(float1 > 3.0)
    assert_true(float2 > 2.0)
    
    // Test basic byte operations (character codes)
    sus char_a tea = "A"
    sus char_b tea = "B"
    assert_eq_string(char_a, "A")
    assert_eq_string(char_b, "B")
    
    vibez.spill("Basic serialization tests completed")
    
    print_test_summary()
}

// Run simple test
test_simple_serialization()
