yeet "testz"  
yeet "lookin_glass"

fr fr Reflection and Introspection Demonstration
fr fr Shows real runtime type information and object inspection

vibez.spill("=== CURSED LookinGlass Reflection Demo ===")

fr fr Demonstrate type reflection for different value types
sus demo_int normie = 42
sus demo_float meal = 3.14159
sus demo_bool lit = based
sus demo_string tea = "Hello, Reflection!"
sus demo_array []normie = [1, 2, 3, 4, 5]

fr fr Get type information for each value
sus int_type TypeInfo = get_type_info_int(demo_int)
sus float_type TypeInfo = get_type_info_float(demo_float)
sus bool_type TypeInfo = get_type_info_bool(demo_bool)
sus string_type TypeInfo = get_type_info_string(demo_string)
sus array_type TypeInfo = get_type_info_array(demo_array)

vibez.spill("--- Type Information ---")
vibez.spill("Integer Type: " + int_type.name + ", Kind: " + stringify_int(int_type.kind) + ", Size: " + stringify_int(int_type.size))
vibez.spill("Float Type: " + float_type.name + ", Kind: " + stringify_int(float_type.kind) + ", Size: " + stringify_int(float_type.size))
vibez.spill("Bool Type: " + bool_type.name + ", Kind: " + stringify_int(bool_type.kind) + ", Size: " + stringify_int(bool_type.size))
vibez.spill("String Type: " + string_type.name + ", Kind: " + stringify_int(string_type.kind) + ", Size: " + stringify_int(string_type.size))
vibez.spill("Array Type: " + array_type.name + ", Kind: " + stringify_int(array_type.kind) + ", Size: " + stringify_int(array_type.size))

fr fr Demonstrate value inspection
vibez.spill("")
vibez.spill("--- Value Inspection ---")
vibez.spill(inspect_int_value(demo_int))
vibez.spill(inspect_float_value(demo_float))
vibez.spill(inspect_string_value(demo_string))
vibez.spill(inspect_array_value(demo_array))

fr fr Demonstrate type classification
vibez.spill("")
vibez.spill("--- Type Classification ---")
vibez.spill("INT is primitive: " + stringify_bool(is_primitive_type(INT)))
vibez.spill("INT is reference: " + stringify_bool(is_reference_type(INT)))
vibez.spill("INT can compare: " + stringify_bool(can_compare_type(INT)))
vibez.spill("INT can copy: " + stringify_bool(can_copy_type(INT)))
vibez.spill("INT can hash: " + stringify_bool(can_hash_type(INT)))

vibez.spill("ARRAY is primitive: " + stringify_bool(is_primitive_type(ARRAY)))
vibez.spill("ARRAY is reference: " + stringify_bool(is_reference_type(ARRAY)))
vibez.spill("CHANNEL can copy: " + stringify_bool(can_copy_type(CHANNEL)))

fr fr Demonstrate deep operations
vibez.spill("")
vibez.spill("--- Deep Operations ---")

sus original_int normie = 100
sus copied_int normie = DeepCopy(original_int)
vibez.spill("Original int: " + stringify_int(original_int) + ", Copied int: " + stringify_int(copied_int))
vibez.spill("Deep equal: " + stringify_bool(DeepEqual(original_int, copied_int)))

sus original_string tea = "test"
sus copied_string tea = DeepCopyString(original_string)
vibez.spill("Original string: \"" + original_string + "\", Copied string: \"" + copied_string + "\"")
vibez.spill("Deep equal: " + stringify_bool(DeepEqualStrings(original_string, copied_string)))

sus float1 meal = 3.14159
sus float2 meal = 3.14159
vibez.spill("Float comparison (3.14159 == 3.14159): " + stringify_bool(DeepEqualFloats(float1, float2)))

fr fr Demonstrate method reflection
vibez.spill("")
vibez.spill("--- Method Reflection ---")

sus tea_methods []tea = list_methods("tea")
vibez.spill("String (tea) methods: " + stringify_int(len(tea_methods)) + " total")
sus i normie = 0
bestie (i < len(tea_methods)) {
    vibez.spill("  " + tea_methods[i] + " -> " + get_method_signature("tea", tea_methods[i]))
    i = i + 1
}

sus int_methods []tea = list_methods("normie")
vibez.spill("Integer (normie) methods: " + stringify_int(len(int_methods)) + " total")
i = 0
bestie (i < len(int_methods)) {
    vibez.spill("  " + int_methods[i] + " -> " + get_method_signature("normie", int_methods[i]))
    i = i + 1
}

fr fr Demonstrate comprehensive testing
vibez.spill("")
vibez.spill("--- Comprehensive Test ---")
sus test_result lit = test_reflection_comprehensive()
vibez.spill("Comprehensive reflection test: " + stringify_bool(test_result))

fr fr Demonstrate string analysis
vibez.spill("")
vibez.spill("--- String Analysis ---")
sus test_strings []tea = ["hello", "42", "3.14", ""]
i = 0
bestie (i < len(test_strings)) {
    sus s tea = test_strings[i]
    vibez.spill("String: \"" + s + "\"")
    vibez.spill("  Length: " + stringify_int(len_str(s)))
    vibez.spill("  First char: \"" + first_char(s) + "\"")
    vibez.spill("  Is numeric: " + stringify_bool(is_numeric_string(s)))
    vibez.spill("  Is float: " + stringify_bool(is_float_string(s)))
    i = i + 1
}

vibez.spill("")
vibez.spill("✅ Reflection demonstration complete!")
vibez.spill("The lookin_glass module provides real runtime type introspection,")
vibez.spill("deep comparison, copying, method reflection, and comprehensive")
vibez.spill("object inspection capabilities for production CURSED applications.")
