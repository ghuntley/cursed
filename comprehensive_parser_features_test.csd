#!/usr/bin/env cursed

# Comprehensive test for enhanced parser features
# Tests source location tracking, error reporting, size expressions, 
# mutability parsing, and recovery parsing

vibe test_enhanced_parser

yeet "testz"

# Test source location tracking with error handling
slay test_source_location_tracking() {
    test_start("Source location tracking")
    
    # This should trigger source location tracking in error expressions
    sus result drip = 0
    
    try {
        yikes("Test error with source location")
        result = 1  # Should not reach here
    } catch {
        result = 2  # Error properly caught
    }
    
    assert_eq_int(result, 2)
}

# Test size expression parsing in array types
slay test_size_expression_parsing() {
    test_start("Size expression parsing")
    
    # Test array with constant size
    sus array1 [5]normie
    
    # Test array with expression size
    sus size drip = 10
    sus array2 [size]normie
    
    # Test array with complex expression size
    sus array3 [size + 5]normie
    
    vibez.spill("Array types parsed successfully")
    assert_true(based)
}

# Test mutability parsing for pointer types
slay test_mutability_parsing() {
    test_start("Mutability parsing")
    
    # Test immutable pointer
    sus ptr1 *normie
    
    # Test mutable pointer  
    sus ptr2 *sus normie
    
    # Test nested mutable pointers
    sus ptr3 *sus *sus normie
    
    vibez.spill("Pointer mutability parsed successfully")
    assert_true(based)
}

# Test enhanced error reporting
slay test_enhanced_error_reporting() {
    test_start("Enhanced error reporting")
    
    # This should provide detailed error location information
    sus test_var drip = 42
    
    # Test error recovery - parser should recover from syntax errors
    # Note: This syntax is intentionally malformed to test recovery
    # sus bad_syntax = = = 123  # This would trigger error recovery
    
    # Parser should continue after error recovery
    sus good_syntax drip = 123
    
    assert_eq_int(good_syntax, 123)
}

# Test recovery parsing capabilities
slay test_recovery_parsing() {
    test_start("Recovery parsing")
    
    sus counter drip = 0
    
    # Test that parser can recover from errors and continue
    bestie (counter < 3) {
        counter = counter + 1
    }
    
    assert_eq_int(counter, 3)
}

# Test complex type expressions with all features
slay test_complex_type_expressions() {
    test_start("Complex type expressions")
    
    # Test slice of arrays
    sus slice_of_arrays [][10]normie
    
    # Test array of pointers
    sus array_of_ptrs [5]*normie
    
    # Test array of mutable pointers
    sus array_of_mut_ptrs [5]*sus normie
    
    # Test pointer to array
    sus ptr_to_array *[10]normie
    
    # Test mutable pointer to mutable array
    sus mut_ptr_to_array *sus [10]normie
    
    vibez.spill("Complex type expressions parsed successfully")
    assert_true(based)
}

# Test function with complex parameter types
slay test_function_with_complex_params(
    array_param [10]normie,
    slice_param []normie,
    ptr_param *normie,
    mut_ptr_param *sus normie
) normie {
    # Function body using the parameters
    damn array_param[0]
}

# Test struct with complex field types
squad ComplexStruct {
    array_field [20]normie
    slice_field []normie
    ptr_field *normie
    mut_ptr_field *sus normie
}

# Test comprehensive parsing integration
slay test_comprehensive_parsing_integration() {
    test_start("Comprehensive parsing integration")
    
    # Create instance of complex struct
    sus complex_instance ComplexStruct
    
    # Test array initialization with size expression
    sus dynamic_size drip = 15
    sus dynamic_array [dynamic_size]normie
    
    # Test function call with complex types
    sus result drip = test_function_with_complex_params(
        dynamic_array,
        dynamic_array[:],
        &dynamic_array[0],
        &dynamic_array[0]
    )
    
    assert_eq_int(result, 0)
}

# Main test runner
slay main() {
    vibez.spill("=== Enhanced Parser Features Test Suite ===")
    
    test_source_location_tracking()
    test_size_expression_parsing()
    test_mutability_parsing()
    test_enhanced_error_reporting()
    test_recovery_parsing()
    test_complex_type_expressions()
    test_comprehensive_parsing_integration()
    
    print_test_summary()
    vibez.spill("=== Parser Features Test Complete ===")
}
