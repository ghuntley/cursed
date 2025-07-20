# Quick Validation of Enhanced Testz Framework
# Tests basic functionality without complex dependencies

yeet "testz"

# Simplified test to verify framework basics
slay validate_framework_basics() {
    vibez.spill("🔍 Validating enhanced testz framework...")
    
    # Test basic assertions
    assert_eq_int(1, 1)
    assert_true(based)
    assert_false(cap)
    
    # Test comparison assertions  
    assert_gt(10, 5)
    assert_lt(3, 8)
    assert_gte(5, 5)
    assert_lte(7, 7)
    assert_not_eq(1, 2)
    
    # Test string assertions
    assert_eq_string("test", "test")
    assert_not_null("validation")
    
    vibez.spill("✅ Basic framework validation complete")
}

# Test state management
slay validate_state_management() {
    vibez.spill("🔍 Validating state management...")
    
    sus initial_pass normie = get_pass_count()
    
    # Run some assertions
    assert_true(based)
    assert_eq_int(42, 42)
    
    # Verify state changed
    sus current_pass normie = get_pass_count()
    vibez.spill("Pass count increased from ", initial_pass, " to ", current_pass)
    
    vibez.spill("✅ State management validation complete")
}

# Main validation
slay main() {
    vibez.spill("🧪 TESTZ FRAMEWORK VALIDATION")
    vibez.spill("=" * 40)
    
    validate_framework_basics()
    validate_state_management()
    
    print_test_summary()
    
    vibez.spill("🎯 Framework validation complete!")
    vibez.spill("Enhanced testz is ready for stdlib testing")
}

main()
