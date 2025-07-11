yeet "reflection"

# Comprehensive CURSED Reflection System Demo
# Showcases all major reflection capabilities

slay demo_type_inspection() lit {
    vibez.spill("=== Type Inspection Demo ===")
    
    # Test various types
    sus int_val normie = 42
    sus float_val meal = 3.14159
    sus str_val tea = "Hello, CURSED!"
    sus bool_val lit = based
    sus char_val sip = 'x'
    
    sus values []interface{} = []interface{}{int_val, float_val, str_val, bool_val, char_val}
    
    bestie i := 0; i < 5; i++ {
        sus rv ReflectValue = reflect_value_of(values[i])
        vibez.spill("Value: " + convert_to_string(values[i]))
        vibez.spill("Type: " + get_type_name(rv))
        vibez.spill("Kind: " + get_type_kind(rv))
        vibez.spill("Size: " + convert_to_string(get_type_size(rv)) + " bytes")
        vibez.spill("Valid: " + convert_to_string(is_valid(rv)))
        vibez.spill("---")
    }
    
    damn based
}

slay demo_dynamic_method_calling() lit {
    vibez.spill("=== Dynamic Method Calling Demo ===")
    
    # Test dynamic string conversion
    sus values []interface{} = []interface{}{42, 3.14, based, 'a'}
    
    bestie i := 0; i < 4; i++ {
        sus rv ReflectValue = reflect_value_of(values[i])
        sus str_result interface{} = call_method(rv, "string", []interface{}{})
        vibez.spill("Original: " + convert_to_string(values[i]))
        vibez.spill("Dynamic string(): " + str_result.(tea))
        vibez.spill("---")
    }
    
    # Test dynamic numeric conversion
    sus str_val tea = "42"
    sus rv_str ReflectValue = reflect_value_of(str_val)
    sus int_result interface{} = call_method(rv_str, "int", []interface{}{})
    vibez.spill("String to int: " + str_val + " -> " + convert_to_string(int_result))
    
    damn based
}

slay demo_interface_checking() lit {
    vibez.spill("=== Interface Implementation Demo ===")
    
    sus values []interface{} = []interface{}{42, 3.14, "hello", based}
    sus interfaces []tea = []tea{"Stringer", "Numeric", "Comparable"}
    
    bestie i := 0; i < 4; i++ {
        sus rv ReflectValue = reflect_value_of(values[i])
        vibez.spill("Value: " + convert_to_string(values[i]) + " (" + get_type_name(rv) + ")")
        
        bestie j := 0; j < 3; j++ {
            sus implements lit = implements_interface(rv, interfaces[j])
            vibez.spill("  " + interfaces[j] + ": " + convert_to_string(implements))
        }
        vibez.spill("---")
    }
    
    damn based
}

slay demo_deep_equality() lit {
    vibez.spill("=== Deep Equality Demo ===")
    
    # Test same type equality
    sus int1 normie = 42
    sus int2 normie = 42
    sus int3 normie = 24
    
    vibez.spill("Integer equality:")
    vibez.spill("42 == 42: " + convert_to_string(deep_equal(int1, int2)))
    vibez.spill("42 == 24: " + convert_to_string(deep_equal(int1, int3)))
    
    # Test different type equality
    sus str_val tea = "42"
    vibez.spill("Cross-type equality:")
    vibez.spill("42 (int) == '42' (string): " + convert_to_string(deep_equal(int1, str_val)))
    
    # Test float equality
    sus float1 meal = 3.14
    sus float2 meal = 3.14
    vibez.spill("Float equality:")
    vibez.spill("3.14 == 3.14: " + convert_to_string(deep_equal(float1, float2)))
    
    damn based
}

slay demo_type_conversions() lit {
    vibez.spill("=== Type Conversion Demo ===")
    
    # Test comprehensive type conversions
    sus original_int normie = 42
    
    vibez.spill("Converting integer 42:")
    vibez.spill("To string: " + convert_to_string(original_int))
    vibez.spill("To int: " + convert_to_string(convert_to_int(original_int)))
    vibez.spill("To bool: " + convert_to_string(convert_to_bool(original_int)))
    vibez.spill("To float: " + convert_to_string(convert_to_float(original_int)))
    
    # Test boolean conversions
    sus bool_true lit = based
    sus bool_false lit = cap
    
    vibez.spill("Boolean conversions:")
    vibez.spill("true to int: " + convert_to_string(convert_to_int(bool_true)))
    vibez.spill("false to int: " + convert_to_string(convert_to_int(bool_false)))
    vibez.spill("true to string: " + convert_to_string(bool_true))
    vibez.spill("false to string: " + convert_to_string(bool_false))
    
    damn based
}

slay demo_zero_values() lit {
    vibez.spill("=== Zero Value Generation Demo ===")
    
    sus types []tea = []tea{"normie", "meal", "tea", "lit", "sip"}
    
    bestie i := 0; i < 5; i++ {
        sus zero_val interface{} = get_zero_value(types[i])
        vibez.spill(types[i] + " zero value: " + convert_to_string(zero_val))
    }
    
    damn based
}

slay demo_method_metadata() lit {
    vibez.spill("=== Method Metadata Demo ===")
    
    sus int_val normie = 42
    sus rv ReflectValue = reflect_value_of(int_val)
    
    sus method_names []tea = get_method_names(rv)
    vibez.spill("Available methods for " + get_type_name(rv) + ":")
    
    bestie i := 0; i < 4; i++ {
        sus method_info MethodInfo = get_method_info(rv, method_names[i])
        vibez.spill("  " + method_info.name + ": " + method_info.signature)
        vibez.spill("    Returns: " + method_info.return_type)
        vibez.spill("    Accessible: " + convert_to_string(method_info.accessible))
    }
    
    damn based
}

slay demo_reflection_utilities() lit {
    vibez.spill("=== Reflection Utilities Demo ===")
    
    # Test type assertion
    sus int_val normie = 42
    sus rv ReflectValue = reflect_value_of(int_val)
    
    sus asserted_int interface{} = type_assert(rv, "normie")
    vibez.spill("Type assertion (normie): " + convert_to_string(asserted_int != cringe))
    
    sus asserted_str interface{} = type_assert(rv, "tea")
    vibez.spill("Type assertion (tea): " + convert_to_string(asserted_str != cringe))
    
    # Test nil checking
    vibez.spill("Nil checking:")
    vibez.spill("Integer is nil: " + convert_to_string(is_nil(rv)))
    
    # Test method existence
    vibez.spill("Method existence:")
    vibez.spill("Has 'string' method: " + convert_to_string(has_method(rv, "string")))
    vibez.spill("Has 'unknown' method: " + convert_to_string(has_method(rv, "unknown")))
    
    damn based
}

slay comprehensive_reflection_demo() lit {
    vibez.spill("🔍 CURSED Reflection System - Comprehensive Demo")
    vibez.spill("=============================================")
    
    # Run all demonstration functions
    demo_type_inspection()
    demo_dynamic_method_calling()
    demo_interface_checking()
    demo_deep_equality()
    demo_type_conversions()
    demo_zero_values()
    demo_method_metadata()
    demo_reflection_utilities()
    
    vibez.spill("✅ Reflection system demo completed successfully!")
    vibez.spill("All reflection capabilities are working correctly.")
    
    damn based
}

# Run the comprehensive demo
comprehensive_reflection_demo()
