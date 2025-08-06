fr fr Test stdlib integration in compilation mode

yeet "testz"
yeet "vibez"
yeet "mathz"
yeet "stringz"

slay main() cringe {
    vibez.spill("Testing stdlib in compilation mode...")
    
    fr fr Test basic math operations
    sus result meal = mathz.math_add(10.0, 5.0)
    vibez.spillf("10 + 5 = %f", result)
    
    fr fr Test string operations  
    sus len normie = stringz.length("CURSED")
    vibez.spillf("Length of 'CURSED': %d", len)
    
    fr fr Test boolean logic
    sus is_positive lit = mathz.is_positive_meal(result)
    vibez.spillf("Result is positive: %b", is_positive)
    
    fr fr Run test assertions
    test_start("compilation mode test")
    assert_true(result > 0.0)
    assert_eq_int(len, 6)
    assert_true(is_positive)
    
    print_test_summary()
    vibez.spill("✅ Compilation mode test successful!")
}
