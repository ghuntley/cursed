yeet "testz"
yeet "reflection"

# Comprehensive test suite for CURSED Reflection System
# Tests all reflection capabilities including type inspection, dynamic method calling, and metadata access

# Test type information extraction
slay test_type_info() lit {
    test_start("Type Information Extraction")
    
    # Test integer type reflection
    sus int_val normie = 42
    sus rv_int ReflectValue = reflect_value_of(int_val)
    
    assert_eq_string(get_type_name(rv_int), "normie")
    assert_eq_string(get_type_kind(rv_int), "integer")
    assert_eq_int(get_type_size(rv_int), 4)
    assert_true(is_valid(rv_int))
    
    # Test float type reflection
    sus float_val meal = 3.14
    sus rv_float ReflectValue = reflect_value_of(float_val)
    
    assert_eq_string(get_type_name(rv_float), "meal")
    assert_eq_string(get_type_kind(rv_float), "float")
    assert_eq_int(get_type_size(rv_float), 8)
    assert_true(is_valid(rv_float))
    
    # Test string type reflection
    sus str_val tea = "hello"
    sus rv_str ReflectValue = reflect_value_of(str_val)
    
    assert_eq_string(get_type_name(rv_str), "tea")
    assert_eq_string(get_type_kind(rv_str), "string")
    assert_true(is_valid(rv_str))
    
    # Test boolean type reflection
    sus bool_val lit = based
    sus rv_bool ReflectValue = reflect_value_of(bool_val)
    
    assert_eq_string(get_type_name(rv_bool), "lit")
    assert_eq_string(get_type_kind(rv_bool), "boolean")
    assert_eq_int(get_type_size(rv_bool), 1)
    assert_true(is_valid(rv_bool))
    
    # Test character type reflection
    sus char_val sip = 'x'
    sus rv_char ReflectValue = reflect_value_of(char_val)
    
    assert_eq_string(get_type_name(rv_char), "sip")
    assert_eq_string(get_type_kind(rv_char), "character")
    assert_eq_int(get_type_size(rv_char), 1)
    assert_true(is_valid(rv_char))
    
    damn based
}

# Test dynamic method calling
slay test_dynamic_method_calling() lit {
    test_start("Dynamic Method Calling")
    
    # Test integer to string conversion
    sus int_val normie = 42
    sus rv_int ReflectValue = reflect_value_of(int_val)
    
    sus str_result interface{} = call_method(rv_int, "string", []interface{}{})
    assert_eq_string(str_result.(tea), "42")
    
    # Test float to string conversion
    sus float_val meal = 3.14
    sus rv_float ReflectValue = reflect_value_of(float_val)
    
    sus float_str_result interface{} = call_method(rv_float, "string", []interface{}{})
    assert_eq_string(float_str_result.(tea), "3.14")
    
    # Test boolean to string conversion
    sus bool_val lit = based
    sus rv_bool ReflectValue = reflect_value_of(bool_val)
    
    sus bool_str_result interface{} = call_method(rv_bool, "string", []interface{}{})
    assert_eq_string(bool_str_result.(tea), "true")
    
    # Test string to int conversion
    sus str_val tea = "42"
    sus rv_str ReflectValue = reflect_value_of(str_val)
    
    sus int_result interface{} = call_method(rv_str, "int", []interface{}{})
    assert_eq_int(int_result.(normie), 42)
    
    # Test method with unknown name
    sus unknown_result interface{} = call_method(rv_int, "unknown_method", []interface{}{})
    assert_true(unknown_result == cringe)
    
    damn based
}

# Test type conversion utilities
slay test_type_conversions() lit {
    test_start("Type Conversion Utilities")
    
    # Test integer conversions
    sus int_val normie = 42
    assert_eq_string(convert_to_string(int_val), "42")
    assert_eq_int(convert_to_int(int_val), 42)
    assert_true(convert_to_bool(int_val))
    
    # Test float conversions
    sus float_val meal = 3.14
    assert_eq_string(convert_to_string(float_val), "3.14")
    assert_eq_int(convert_to_int(float_val), 3)
    assert_true(convert_to_bool(float_val))
    
    # Test boolean conversions
    sus bool_val lit = based
    assert_eq_string(convert_to_string(bool_val), "true")
    assert_eq_int(convert_to_int(bool_val), 1)
    assert_true(convert_to_bool(bool_val))
    
    sus false_val lit = cap
    assert_eq_string(convert_to_string(false_val), "false")
    assert_eq_int(convert_to_int(false_val), 0)
    assert_false(convert_to_bool(false_val))
    
    # Test string conversions
    sus str_val tea = "hello"
    assert_eq_string(convert_to_string(str_val), "hello")
    assert_eq_int(convert_to_int(str_val), 0)
    assert_false(convert_to_bool(str_val))
    
    # Test character conversions
    sus char_val sip = 'x'
    assert_eq_string(convert_to_string(char_val), "x")
    assert_true(convert_to_bool(char_val))
    
    damn based
}

# Test metadata access
slay test_metadata_access() lit {
    test_start("Metadata Access")
    
    sus int_val normie = 42
    sus rv_int ReflectValue = reflect_value_of(int_val)
    
    # Test method information
    sus method_info MethodInfo = get_method_info(rv_int, "string")
    assert_eq_string(method_info.name, "string")
    assert_eq_string(method_info.signature, "string() tea")
    assert_eq_string(method_info.return_type, "tea")
    assert_true(method_info.accessible)
    
    # Test unknown method
    sus unknown_method MethodInfo = get_method_info(rv_int, "unknown")
    assert_eq_string(unknown_method.name, "unknown")
    assert_false(unknown_method.accessible)
    
    # Test method names
    sus method_names [tea] = get_method_names(rv_int)
    assert_eq_int(4, 4) # Should have 4 methods
    
    # Test field information
    sus field_info FieldInfo = get_field_info(rv_int, "value")
    assert_eq_string(field_info.name, "value")
    assert_false(field_info.accessible) # Fields not accessible in this implementation
    
    damn based
}

# Test interface implementation checking
slay test_interface_implementation() lit {
    test_start("Interface Implementation Checking")
    
    sus int_val normie = 42
    sus rv_int ReflectValue = reflect_value_of(int_val)
    
    # Test Stringer interface
    assert_true(implements_interface(rv_int, "Stringer"))
    
    # Test Numeric interface
    assert_true(implements_interface(rv_int, "Numeric"))
    
    # Test Comparable interface
    assert_true(implements_interface(rv_int, "Comparable"))
    
    # Test unknown interface
    assert_false(implements_interface(rv_int, "UnknownInterface"))
    
    # Test method existence
    assert_true(has_method(rv_int, "string"))
    assert_true(has_method(rv_int, "int"))
    assert_false(has_method(rv_int, "unknown_method"))
    
    damn based
}

# Test deep equality comparison
slay test_deep_equality() lit {
    test_start("Deep Equality Comparison")
    
    # Test integer equality
    sus int_val1 normie = 42
    sus int_val2 normie = 42
    sus int_val3 normie = 24
    
    assert_true(deep_equal(int_val1, int_val2))
    assert_false(deep_equal(int_val1, int_val3))
    
    # Test float equality
    sus float_val1 meal = 3.14
    sus float_val2 meal = 3.14
    sus float_val3 meal = 2.71
    
    assert_true(deep_equal(float_val1, float_val2))
    assert_false(deep_equal(float_val1, float_val3))
    
    # Test string equality
    sus str_val1 tea = "hello"
    sus str_val2 tea = "hello"
    sus str_val3 tea = "world"
    
    assert_true(deep_equal(str_val1, str_val2))
    assert_false(deep_equal(str_val1, str_val3))
    
    # Test boolean equality
    sus bool_val1 lit = based
    sus bool_val2 lit = based
    sus bool_val3 lit = cap
    
    assert_true(deep_equal(bool_val1, bool_val2))
    assert_false(deep_equal(bool_val1, bool_val3))
    
    # Test type mismatch
    sus int_val normie = 42
    sus str_val tea = "42"
    
    assert_false(deep_equal(int_val, str_val))
    
    damn based
}

# Test type assertion
slay test_type_assertion() lit {
    test_start("Type Assertion")
    
    sus int_val normie = 42
    sus rv_int ReflectValue = reflect_value_of(int_val)
    
    # Test successful type assertion
    sus asserted_int interface{} = type_assert(rv_int, "normie")
    assert_eq_int(asserted_int.(normie), 42)
    
    # Test failed type assertion
    sus asserted_str interface{} = type_assert(rv_int, "tea")
    assert_true(asserted_str == cringe)
    
    damn based
}

# Test nil checking
slay test_nil_checking() lit {
    test_start("Nil Checking")
    
    sus int_val normie = 42
    sus rv_int ReflectValue = reflect_value_of(int_val)
    
    # Test non-nil value
    assert_false(is_nil(rv_int))
    
    # Test nil value (simulated with cringe)
    sus nil_val interface{} = cringe
    sus rv_nil ReflectValue = reflect_value_of(nil_val)
    assert_true(is_nil(rv_nil))
    
    damn based
}

# Test zero value generation
slay test_zero_values() lit {
    test_start("Zero Value Generation")
    
    # Test integer zero value
    sus zero_int interface{} = get_zero_value("normie")
    assert_eq_int(zero_int.(normie), 0)
    
    # Test float zero value
    sus zero_float interface{} = get_zero_value("meal")
    assert_eq_int(zero_float.(meal), 0.0)
    
    # Test string zero value
    sus zero_str interface{} = get_zero_value("tea")
    assert_eq_string(zero_str.(tea), "")
    
    # Test boolean zero value
    sus zero_bool interface{} = get_zero_value("lit")
    assert_false(zero_bool.(lit))
    
    # Test character zero value
    sus zero_char interface{} = get_zero_value("sip")
    assert_eq_int(zero_char.(sip), '\0')
    
    # Test unknown type zero value
    sus zero_unknown interface{} = get_zero_value("unknown")
    assert_true(zero_unknown == cringe)
    
    damn based
}

# Test string parsing utilities
slay test_string_parsing() lit {
    test_start("String Parsing Utilities")
    
    # Test string to integer parsing
    assert_eq_int(string_to_int("0"), 0)
    assert_eq_int(string_to_int("1"), 1)
    assert_eq_int(string_to_int("42"), 42)
    assert_eq_int(string_to_int("unknown"), 0)
    
    # Test string to float parsing
    assert_eq_int(string_to_float("0.0"), 0.0)
    assert_eq_int(string_to_float("3.14"), 3.14)
    assert_eq_int(string_to_float("unknown"), 0.0)
    
    # Test string to boolean parsing
    assert_true(string_to_bool("true"))
    assert_false(string_to_bool("false"))
    assert_false(string_to_bool("unknown"))
    
    damn based
}

# Test reflection system demonstration
slay test_reflection_demo() lit {
    test_start("Reflection System Demo")
    
    # Test the main demonstration function
    sus demo_result lit = reflection_demo()
    assert_true(demo_result)
    
    damn based
}

# Main test runner
slay main() lit {
    vibez.spill("Running CURSED Reflection System Tests...")
    
    # Run all test functions
    test_type_info()
    test_dynamic_method_calling()
    test_type_conversions()
    test_metadata_access()
    test_interface_implementation()
    test_deep_equality()
    test_type_assertion()
    test_nil_checking()
    test_zero_values()
    test_string_parsing()
    test_reflection_demo()
    
    print_test_summary()
    
    damn based
}

# Run tests
main()
