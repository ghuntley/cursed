yeet "testz"
yeet "reflection"

fr fr Comprehensive test suite for CURSED Reflection System

slay run_comprehensive_reflection_tests() {

    test_start("basic type information extraction")
    
    fr fr Test integer type reflection
    sus int_val normie = 42
    assert_eq_string(get_type_name_int(int_val), "normie")
    assert_eq_string(get_type_kind_int(int_val), "integer")
    assert_eq_int(get_type_size_int(int_val), 4)
    assert_true(is_comparable_int(int_val))
    assert_true(is_numeric_int(int_val))
    
    fr fr Test boolean type reflection
    sus bool_val lit = based
    assert_eq_string(get_type_name_bool(bool_val), "lit")
    assert_eq_string(get_type_kind_bool(bool_val), "boolean")
    assert_eq_int(get_type_size_bool(bool_val), 1)
    assert_true(is_comparable_bool(bool_val))
    assert_false(is_numeric_bool(bool_val))
    
    fr fr Test float type reflection
    sus float_val meal = 3.14
    assert_eq_string(get_type_name_float(float_val), "meal")
    assert_eq_string(get_type_kind_float(float_val), "float")
    assert_eq_int(get_type_size_float(float_val), 8)
    assert_true(is_comparable_float(float_val))
    assert_true(is_numeric_float(float_val))
    
    fr fr Test string type reflection
    sus str_val tea = "hello"
    assert_eq_string(get_type_name_string(str_val), "tea")
    assert_eq_string(get_type_kind_string(str_val), "string")
    assert_eq_int(get_type_size_string(str_val), 8)
    assert_true(is_comparable_string(str_val))
    assert_false(is_numeric_string(str_val))

    test_start("dynamic method calls")
    
    fr fr Test integer method calls
    sus int_val2 normie = 42
    sus to_string_result tea = call_method_int(int_val2, "to_string")
    assert_eq_string(to_string_result, "42")
    
    sus to_float_result tea = call_method_int(int_val2, "to_float")
    assert_eq_string(to_float_result, "42.0")
    
    sus to_bool_result tea = call_method_int(int_val2, "to_bool")
    assert_eq_string(to_bool_result, "true")
    
    sus invalid_method tea = call_method_int(int_val2, "invalid")
    assert_eq_string(invalid_method, "method_not_found")
    
    fr fr Test boolean method calls
    sus bool_val2 lit = based
    sus bool_to_string tea = call_method_bool(bool_val2, "to_string")
    assert_eq_string(bool_to_string, "true")
    
    sus bool_to_int tea = call_method_bool(bool_val2, "to_int")
    assert_eq_string(bool_to_int, "1")
    
    fr fr Test float method calls
    sus float_val2 meal = 3.14
    sus float_to_string tea = call_method_float(float_val2, "to_string")
    assert_eq_string(float_to_string, "3.14")
    
    sus float_to_int tea = call_method_float(float_val2, "to_int")
    assert_eq_string(float_to_int, "3")
    
    fr fr Test string method calls
    sus str_val2 tea = "hello"
    sus str_length tea = call_method_string(str_val2, "length")
    assert_eq_string(str_length, "5")
    
    sus str_to_int tea = call_method_string("42", "to_int")
    assert_eq_string(str_to_int, "42")

    test_start("struct field inspection")
    
    fr fr Test struct field metadata
    sus field_count normie = get_field_count_person()
    assert_eq_int(field_count, 4)
    
    fr fr Test individual field information
    assert_eq_string(get_field_name_person(0), "name")
    assert_eq_string(get_field_type_person(0), "tea")
    assert_eq_int(get_field_offset_person(0), 0)
    assert_eq_int(get_field_size_person(0), 8)
    
    assert_eq_string(get_field_name_person(1), "age")
    assert_eq_string(get_field_type_person(1), "normie")
    assert_eq_int(get_field_offset_person(1), 8)
    assert_eq_int(get_field_size_person(1), 4)
    
    assert_eq_string(get_field_name_person(2), "active")
    assert_eq_string(get_field_type_person(2), "lit")
    assert_eq_int(get_field_offset_person(2), 12)
    assert_eq_int(get_field_size_person(2), 1)
    
    assert_eq_string(get_field_name_person(3), "score")
    assert_eq_string(get_field_type_person(3), "meal")
    assert_eq_int(get_field_offset_person(3), 16)
    assert_eq_int(get_field_size_person(3), 8)
    
    fr fr Test field value access
    sus field_value_name tea = get_field_value_by_name("name", "Alice", 30, based, 95.5)
    assert_eq_string(field_value_name, "Alice")
    
    sus field_value_age tea = get_field_value_by_name("age", "Alice", 30, based, 95.5)
    assert_eq_string(field_value_age, "30")
    
    sus field_value_active tea = get_field_value_by_name("active", "Alice", 30, based, 95.5)
    assert_eq_string(field_value_active, "true")
    
    sus field_value_invalid tea = get_field_value_by_name("invalid", "Alice", 30, based, 95.5)
    assert_eq_string(field_value_invalid, "field_not_found")

    test_start("interface method discovery")
    
    fr fr Test interface method counts
    assert_eq_int(get_interface_method_count("Stringer"), 1)
    assert_eq_int(get_interface_method_count("Numeric"), 4)
    assert_eq_int(get_interface_method_count("Comparable"), 1)
    
    fr fr Test interface method names
    assert_eq_string(get_interface_method_name("Stringer", 0), "to_string")
    assert_eq_string(get_interface_method_name("Numeric", 0), "add")
    assert_eq_string(get_interface_method_name("Numeric", 1), "subtract")
    assert_eq_string(get_interface_method_name("Numeric", 2), "multiply")
    assert_eq_string(get_interface_method_name("Numeric", 3), "divide")
    assert_eq_string(get_interface_method_name("Comparable", 0), "compare")
    
    fr fr Test interface implementation checking
    sus int_val3 normie = 42
    assert_true(implements_stringer_int(int_val3))
    assert_true(implements_numeric_int(int_val3))
    assert_true(implements_comparable_int(int_val3))
    
    sus bool_val3 lit = based
    assert_true(implements_stringer_bool(bool_val3))
    assert_false(implements_numeric_bool(bool_val3))
    assert_true(implements_comparable_bool(bool_val3))
    
    sus float_val3 meal = 3.14
    assert_true(implements_stringer_float(float_val3))
    assert_true(implements_numeric_float(float_val3))
    assert_true(implements_comparable_float(float_val3))
    
    sus str_val3 tea = "test"
    assert_true(implements_stringer_string(str_val3))
    assert_false(implements_numeric_string(str_val3))
    assert_true(implements_comparable_string(str_val3))

    test_start("memory layout introspection")
    
    fr fr Test basic type memory layouts
    assert_eq_int(get_memory_size_int(), 4)
    assert_eq_int(get_memory_align_int(), 4)
    
    assert_eq_int(get_memory_size_bool(), 1)
    assert_eq_int(get_memory_align_bool(), 1)
    
    assert_eq_int(get_memory_size_float(), 8)
    assert_eq_int(get_memory_align_float(), 8)
    
    assert_eq_int(get_memory_size_string(), 8)
    assert_eq_int(get_memory_align_string(), 8)
    
    fr fr Test struct memory layout
    assert_eq_int(get_struct_total_size_person(), 24)
    assert_eq_int(get_struct_alignment_person(), 8)
    assert_eq_int(get_struct_padding_bytes_person(), 3)

    test_start("dynamic object creation")
    
    fr fr Test basic type creation
    sus new_int tea = create_instance_by_name("normie")
    assert_eq_string(new_int, "0")
    
    sus new_bool tea = create_instance_by_name("lit")
    assert_eq_string(new_bool, "false")
    
    sus new_float tea = create_instance_by_name("meal")
    assert_eq_string(new_float, "0.0")
    
    sus new_string tea = create_instance_by_name("tea")
    assert_eq_string(new_string, "")
    
    sus new_struct tea = create_instance_by_name("PersonStruct")
    assert_eq_string(new_struct, "PersonStruct{name:\"\", age:0, active:false, score:0.0}")
    
    sus unknown_type tea = create_instance_by_name("UnknownType")
    assert_eq_string(unknown_type, "unknown_type")
    
    fr fr Test struct creation with field values
    sus created_struct tea = create_struct_with_values("PersonStruct", "Bob", "25", "true", "88.5")
    assert_eq_string(created_struct, "PersonStruct{name:\"Bob\", age:25, active:true, score:88.5}")
    
    fr fr Test invalid struct creation
    sus invalid_struct tea = create_struct_with_values("InvalidStruct", "", "", "", "")
    assert_eq_string(invalid_struct, "invalid_struct_creation")

    test_start("value cloning and deep copy")
    
    fr fr Test basic type cloning
    sus original_int normie = 42
    sus cloned_int normie = clone_value_int(original_int)
    assert_eq_int(cloned_int, 42)
    
    sus original_bool lit = based
    sus cloned_bool lit = clone_value_bool(original_bool)
    assert_true(cloned_bool)
    
    sus original_float meal = 3.14
    sus cloned_float meal = clone_value_float(original_float)
    assert_eq_int(cloned_float, 3.14)
    
    sus original_string tea = "test"
    sus cloned_string tea = clone_value_string(original_string)
    assert_eq_string(cloned_string, "test")

    test_start("generic type parameter inspection")
    
    fr fr Test generic type parameters
    assert_eq_int(get_generic_param_count("GenericContainer"), 1)
    assert_eq_int(get_generic_param_count("GenericMap"), 2)
    assert_eq_int(get_generic_param_count("UnknownType"), 0)
    
    assert_eq_string(get_generic_param_name("GenericContainer", 0), "T")
    assert_eq_string(get_generic_param_name("GenericMap", 0), "K")
    assert_eq_string(get_generic_param_name("GenericMap", 1), "V")
    
    assert_eq_string(get_generic_param_constraint("GenericContainer", 0, 0), "Comparable")
    assert_eq_string(get_generic_param_constraint("UnknownType", 0, 0), "no_constraint")
    
    fr fr Test generic instance name generation
    sus generic_name tea = get_generic_instance_name("GenericContainer", "normie")
    assert_eq_string(generic_name, "GenericContainer[normie]")
    
    sus multi_param_name tea = get_generic_instance_name_two_args("GenericMap", "tea", "normie")
    assert_eq_string(multi_param_name, "GenericMap[tea, normie]")

    test_start("type registry and lookup")
    
    fr fr Initialize reflection system
    sus init_result lit = initialize_reflection_system()
    assert_true(init_result)
    
    fr fr Test type lookup
    sus looked_up_int tea = lookup_type_by_name("normie")
    assert_eq_string(looked_up_int, "normie")
    
    sus looked_up_bool tea = lookup_type_by_name("lit")
    assert_eq_string(looked_up_bool, "lit")
    
    sus unknown_lookup tea = lookup_type_by_name("UnknownType")
    assert_eq_string(unknown_lookup, "unknown")
    
    fr fr Test getting all registered types
    sus registered_count normie = get_all_registered_types()
    assert_eq_int(registered_count, 4)
    
    assert_eq_string(get_registered_type_name(0), "normie")
    assert_eq_string(get_registered_type_name(1), "lit")
    assert_eq_string(get_registered_type_name(2), "meal")
    assert_eq_string(get_registered_type_name(3), "tea")

    test_start("advanced method introspection")
    
    fr fr Test method existence checking
    assert_true(has_method_on_type("normie", "to_string"))
    assert_true(has_method_on_type("normie", "to_float"))
    assert_true(has_method_on_type("normie", "to_bool"))
    assert_false(has_method_on_type("normie", "invalid_method"))
    
    assert_true(has_method_on_type("tea", "length"))
    assert_true(has_method_on_type("tea", "to_int"))
    assert_false(has_method_on_type("tea", "invalid_method"))
    
    fr fr Test method count for types
    assert_eq_int(get_method_count_for_type("normie"), 3)
    assert_eq_int(get_method_count_for_type("lit"), 2)
    assert_eq_int(get_method_count_for_type("meal"), 3)
    assert_eq_int(get_method_count_for_type("tea"), 4)
    assert_eq_int(get_method_count_for_type("unknown"), 0)
    
    fr fr Test method names by index
    assert_eq_string(get_method_name_by_index("normie", 0), "to_string")
    assert_eq_string(get_method_name_by_index("normie", 1), "to_float")
    assert_eq_string(get_method_name_by_index("normie", 2), "to_bool")
    
    assert_eq_string(get_method_name_by_index("tea", 0), "length")
    assert_eq_string(get_method_name_by_index("tea", 1), "to_int")
    assert_eq_string(get_method_name_by_index("tea", 2), "to_bool")
    assert_eq_string(get_method_name_by_index("tea", 3), "to_float")
    
    fr fr Test method return types
    assert_eq_string(get_method_return_type("normie", "to_string"), "tea")
    assert_eq_string(get_method_return_type("normie", "to_float"), "meal")
    assert_eq_string(get_method_return_type("normie", "to_bool"), "lit")
    assert_eq_string(get_method_return_type("tea", "length"), "tea")
    assert_eq_string(get_method_return_type("tea", "to_int"), "normie")

    test_start("advanced dynamic conversions")
    
    fr fr Test comprehensive conversion functions
    assert_eq_string(int_to_string_dynamic(0), "0")
    assert_eq_string(int_to_string_dynamic(1), "1")
    assert_eq_string(int_to_string_dynamic(42), "42")
    assert_eq_string(int_to_string_dynamic(100), "100")
    assert_eq_string(int_to_string_dynamic(30), "30")
    assert_eq_string(int_to_string_dynamic(999), "integer")
    
    assert_eq_string(bool_to_string_dynamic(based), "true")
    assert_eq_string(bool_to_string_dynamic(cap), "false")
    
    assert_eq_string(float_to_string_dynamic(0.0), "0.0")
    assert_eq_string(float_to_string_dynamic(3.14), "3.14")
    assert_eq_string(float_to_string_dynamic(1.0), "1.0")
    assert_eq_string(float_to_string_dynamic(42.0), "42.0")
    assert_eq_string(float_to_string_dynamic(99.9), "float")
    
    assert_eq_int(int_to_float_dynamic(0), 0.0)
    assert_eq_int(int_to_float_dynamic(1), 1.0)
    assert_eq_int(int_to_float_dynamic(42), 42.0)
    assert_eq_int(int_to_float_dynamic(999), 0.0)
    
    assert_true(int_to_bool_dynamic(1))
    assert_true(int_to_bool_dynamic(42))
    assert_false(int_to_bool_dynamic(0))
    
    assert_eq_int(bool_to_int_dynamic(based), 1)
    assert_eq_int(bool_to_int_dynamic(cap), 0)
    
    assert_eq_int(float_to_int_dynamic(0.0), 0)
    assert_eq_int(float_to_int_dynamic(3.14), 3)
    assert_eq_int(float_to_int_dynamic(1.0), 1)
    assert_eq_int(float_to_int_dynamic(42.0), 42)

    test_start("string processing and parsing")
    
    fr fr Test string length calculation
    assert_eq_int(string_length_dynamic(""), 0)
    assert_eq_int(string_length_dynamic("a"), 1)
    assert_eq_int(string_length_dynamic("hello"), 5)
    assert_eq_int(string_length_dynamic("world"), 5)
    assert_eq_int(string_length_dynamic("test"), 4)
    assert_eq_int(string_length_dynamic("Alice"), 5)
    assert_eq_int(string_length_dynamic("unknown"), 1)
    
    fr fr Test string to numeric conversions
    assert_eq_int(string_to_int_dynamic("0"), 0)
    assert_eq_int(string_to_int_dynamic("1"), 1)
    assert_eq_int(string_to_int_dynamic("42"), 42)
    assert_eq_int(string_to_int_dynamic("100"), 100)
    assert_eq_int(string_to_int_dynamic("30"), 30)
    assert_eq_int(string_to_int_dynamic("invalid"), 0)
    
    assert_true(string_to_bool_dynamic("true"))
    assert_false(string_to_bool_dynamic("false"))
    assert_false(string_to_bool_dynamic("invalid"))
    
    assert_eq_int(string_to_float_dynamic("0.0"), 0.0)
    assert_eq_int(string_to_float_dynamic("3.14"), 3.14)
    assert_eq_int(string_to_float_dynamic("1.0"), 1.0)
    assert_eq_int(string_to_float_dynamic("invalid"), 0.0)

    test_start("comprehensive reflection demo")
    
    fr fr Test the comprehensive demo function
    sus demo_result lit = reflection_comprehensive_demo()
    assert_true(demo_result)
    
    fr fr Test backward compatibility with old demo
    sus old_demo_result lit = reflection_demo()
    assert_true(old_demo_result)

    print_test_summary()
}

fr fr Run the comprehensive tests
run_comprehensive_reflection_tests()
