yeet "testz"
yeet "lookin_glass"

test_group_start("Type Information")

test_start("typeof_functions_test")
assert_eq_int(typeof_int(42), INT)
assert_eq_int(typeof_float(3.14), FLOAT) 
assert_eq_int(typeof_bool(based), BOOL)
assert_eq_int(typeof_string("hello"), STRING)

test_start("type_name_mapping_test")
assert_eq_string(get_type_name_by_kind(INT), "normie")
assert_eq_string(get_type_name_by_kind(FLOAT), "meal")
assert_eq_string(get_type_name_by_kind(BOOL), "lit")
assert_eq_string(get_type_name_by_kind(STRING), "tea")

test_start("get_type_size_test")
assert_eq_int(get_type_size(BOOL), 1)
assert_eq_int(get_type_size(INT), 8)
assert_eq_int(get_type_size(FLOAT), 8)
assert_eq_int(get_type_size(STRING), 16)

test_group_end()

test_group_start("Type Classification")

test_start("is_numeric_type_test")
assert_true(is_numeric_type(INT))
assert_true(is_numeric_type(FLOAT))
assert_false(is_numeric_type(STRING))
assert_false(is_numeric_type(BOOL))

test_start("is_collection_type_test")
assert_true(is_collection_type(ARRAY))
assert_true(is_collection_type(STRING))
assert_false(is_collection_type(INT))
assert_false(is_collection_type(BOOL))

test_start("is_composite_type_test")
assert_true(is_composite_type(STRUCT))
assert_true(is_composite_type(INTERFACE))
assert_false(is_composite_type(INT))
assert_false(is_composite_type(STRING))

test_group_end()

test_group_start("Deep Operations")

test_start("deep_equal_test")
assert_true(DeepEqual(42, 42))
assert_false(DeepEqual(42, 24))
assert_true(DeepEqualStrings("hello", "hello"))
assert_false(DeepEqualStrings("hello", "world"))

test_start("deep_copy_test")
assert_eq_int(DeepCopy(42), 42)
assert_eq_string(DeepCopyString("test"), "test")
assert_true(DeepEqualFloats(DeepCopyFloat(3.14), 3.14))
assert_eq_bool(DeepCopyBool(based), based)

test_group_end()

test_group_start("String Analysis")

test_start("string_utilities_test")
assert_eq_int(len_str("hello"), 5)
assert_eq_string(char_at("hello", 0), "a")
assert_eq_string(first_char("hello"), "a")
assert_eq_string(first_char(""), "")

test_start("numeric_string_detection_test")
assert_true(is_numeric_string("42"))
assert_true(is_numeric_string("0"))
assert_false(is_numeric_string("hello"))
assert_false(is_numeric_string(""))

test_start("float_string_detection_test")
assert_true(is_float_string("3.14"))
assert_true(is_float_string("1.0"))
assert_false(is_float_string("42"))
assert_false(is_float_string("hello"))

test_group_end()

test_group_start("Type Info Structures")

test_start("type_info_creation_test")
sus info TypeInfo = create_type_info("test", STRING)
assert_eq_string(info.name, "test")
assert_eq_int(info.kind, STRING)
assert_eq_int(info.size, 16)
assert_true(info.is_primitive)
assert_true(info.can_compare)
assert_true(info.can_copy)

test_start("type_info_functions_test")
sus string_info TypeInfo = get_type_info_string("hello")
assert_eq_string(string_info.name, "tea")
assert_eq_int(string_info.kind, STRING)
assert_true(string_info.is_primitive)

sus int_info TypeInfo = get_type_info_int(42)
assert_eq_string(int_info.name, "normie")
assert_eq_int(int_info.kind, INT)
assert_true(int_info.is_primitive)

test_group_end()

test_group_start("Value Inspection")

test_start("inspect_value_test") 
sus string_inspection tea = inspect_string_value("hello")
assert_true(len_str(string_inspection) > 0)

sus int_inspection tea = inspect_int_value(42)
assert_true(len_str(int_inspection) > 0)

sus float_inspection tea = inspect_float_value(3.14)
assert_true(len_str(float_inspection) > 0)

test_start("stringify_functions_test")
assert_eq_string(stringify_kind(STRING), "tea")
assert_eq_string(stringify_kind(INT), "normie")
assert_eq_string(stringify_int(8), "8")
assert_eq_string(stringify_int(16), "16")

test_group_end()

test_group_start("Method Reflection")

test_start("has_method_test")
assert_true(has_method("tea", "len"))
assert_true(has_method("array", "append"))
assert_false(has_method("tea", "nonexistent"))

test_start("method_signature_test")
sus sig tea = get_method_signature("tea", "len")
assert_eq_string(sig, "slay len(s tea) normie")

test_group_end()

test_group_start("Memory Analysis")

test_start("memory_estimation_test")
sus info TypeInfo = create_type_info("test", STRING)
assert_eq_int(estimate_memory_usage(info), 16)

test_start("allocation_type_test")
sus primitive_info TypeInfo = create_type_info("int", INT)
sus composite_info TypeInfo = create_type_info("struct", STRUCT)
assert_true(is_stack_allocated(primitive_info))
assert_false(is_stack_allocated(composite_info))

test_group_end()

test_group_start("Comprehensive Tests")

test_start("comprehensive_reflection_test")
assert_true(test_reflection_comprehensive())

test_group_end()

print_test_summary()

vibez.spill("✅ LookinGlass Reflection Module Test Complete!")
