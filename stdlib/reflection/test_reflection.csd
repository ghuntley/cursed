yeet "testz"
yeet "reflection"

fr fr Comprehensive test suite for CURSED Reflection System

slay run_reflection_tests() {

    test_start("reflection type information for integers")

    fr fr Test integer type reflection
    sus int_val normie = 42
    assert_eq_string(get_type_name_int(int_val), "normie")
    assert_eq_string(get_type_kind_int(int_val), "integer")
    assert_eq_int(get_type_size_int(int_val), 4)
    assert_true(is_valid_int(int_val))

    test_start("reflection type information for booleans")

    fr fr Test boolean type reflection
    sus bool_val lit = based
    assert_eq_string(get_type_name_bool(bool_val), "lit")
    assert_eq_string(get_type_kind_bool(bool_val), "boolean")
    assert_eq_int(get_type_size_bool(bool_val), 1)
    assert_true(is_valid_bool(bool_val))

    test_start("reflection type information for floats")

    fr fr Test float type reflection
    sus float_val meal = 3.14
    assert_eq_string(get_type_name_float(float_val), "meal")
    assert_eq_string(get_type_kind_float(float_val), "float")
    assert_eq_int(get_type_size_float(float_val), 8)
    assert_true(is_valid_float(float_val))

    test_start("reflection type information for strings")

    fr fr Test string type reflection
    sus str_val tea = "hello"
    assert_eq_string(get_type_name_string(str_val), "tea")
    assert_eq_string(get_type_kind_string(str_val), "string")
    assert_eq_int(get_type_size_string(str_val), 8)
    assert_true(is_valid_string(str_val))

    test_start("reflection integer conversions")

    fr fr Test integer conversions
    sus int_val2 normie = 42
    assert_eq_string(convert_int_to_string(int_val2), "42")
    assert_eq_int(convert_int_to_int(int_val2), 42)
    assert_true(convert_int_to_bool(int_val2))
    assert_eq_int(convert_int_to_float(int_val2), 42.0)

    sus zero_int normie = 0
    assert_eq_string(convert_int_to_string(zero_int), "0")
    assert_false(convert_int_to_bool(zero_int))
    assert_eq_int(convert_int_to_float(zero_int), 0.0)

    test_start("reflection boolean conversions")

    fr fr Test boolean conversions
    sus bool_val2 lit = based
    assert_eq_string(convert_bool_to_string(bool_val2), "true")
    assert_eq_int(convert_bool_to_int(bool_val2), 1)
    assert_true(convert_bool_to_bool(bool_val2))
    assert_eq_int(convert_bool_to_float(bool_val2), 1.0)

    sus false_val lit = cap
    assert_eq_string(convert_bool_to_string(false_val), "false")
    assert_eq_int(convert_bool_to_int(false_val), 0)
    assert_false(convert_bool_to_bool(false_val))
    assert_eq_int(convert_bool_to_float(false_val), 0.0)

    test_start("reflection float conversions")

    fr fr Test float conversions
    sus float_val2 meal = 3.14
    assert_eq_string(convert_float_to_string(float_val2), "3.14")
    assert_eq_int(convert_float_to_int(float_val2), 3)
    assert_true(convert_float_to_bool(float_val2))
    assert_eq_int(convert_float_to_float(float_val2), 3.14)

    sus zero_float meal = 0.0
    assert_eq_string(convert_float_to_string(zero_float), "0.0")
    assert_eq_int(convert_float_to_int(zero_float), 0)
    assert_false(convert_float_to_bool(zero_float))

    test_start("reflection string conversions")

    fr fr Test string conversions
    sus str_val2 tea = "hello"
    assert_eq_string(convert_string_to_string(str_val2), "hello")
    assert_eq_int(convert_string_to_int(str_val2), 0)
    assert_false(convert_string_to_bool(str_val2))
    assert_eq_int(convert_string_to_float(str_val2), 0.0)

    sus num_str tea = "42"
    assert_eq_int(convert_string_to_int(num_str), 42)

    sus bool_str tea = "true"
    assert_true(convert_string_to_bool(bool_str))

    test_start("reflection method checking")

    fr fr Test method existence for different types
    sus int_val3 normie = 42
    assert_true(has_method_int(int_val3, "string"))
    assert_true(has_method_int(int_val3, "int"))
    assert_true(has_method_int(int_val3, "bool"))
    assert_true(has_method_int(int_val3, "float"))
    assert_false(has_method_int(int_val3, "unknown_method"))

    sus bool_val3 lit = based
    assert_true(has_method_bool(bool_val3, "string"))
    assert_true(has_method_bool(bool_val3, "int"))

    sus float_val3 meal = 3.14
    assert_true(has_method_float(float_val3, "string"))
    assert_true(has_method_float(float_val3, "float"))

    sus str_val3 tea = "test"
    assert_true(has_method_string(str_val3, "string"))
    assert_true(has_method_string(str_val3, "int"))

    test_start("reflection method metadata")

    fr fr Test method count
    sus method_count normie = get_method_count()
    assert_eq_int(method_count, 4)

    fr fr Test method signatures
    assert_eq_string(get_method_signature("string"), "string() tea")
    assert_eq_string(get_method_signature("int"), "int() normie")
    assert_eq_string(get_method_signature("bool"), "bool() lit")
    assert_eq_string(get_method_signature("float"), "float() meal")
    assert_eq_string(get_method_signature("unknown"), "unknown() unknown")

    fr fr Test return types
    assert_eq_string(get_method_return_type("string"), "tea")
    assert_eq_string(get_method_return_type("int"), "normie")
    assert_eq_string(get_method_return_type("bool"), "lit")
    assert_eq_string(get_method_return_type("float"), "meal")

    fr fr Test method accessibility
    assert_true(is_method_accessible("string"))
    assert_true(is_method_accessible("int"))
    assert_true(is_method_accessible("bool"))
    assert_true(is_method_accessible("float"))
    assert_false(is_method_accessible("unknown"))

    test_start("reflection interface implementation")

    fr fr Test Stringer interface
    sus int_val4 normie = 42
    assert_true(implements_stringer_int(int_val4))

    sus bool_val4 lit = based
    assert_true(implements_stringer_bool(bool_val4))

    sus float_val4 meal = 3.14
    assert_true(implements_stringer_float(float_val4))

    sus str_val4 tea = "test"
    assert_true(implements_stringer_string(str_val4))

    fr fr Test Numeric interface
    assert_true(implements_numeric_int(int_val4))
    assert_false(implements_numeric_bool(bool_val4))
    assert_true(implements_numeric_float(float_val4))
    assert_false(implements_numeric_string(str_val4))

    fr fr Test Comparable interface
    assert_true(implements_comparable_int(int_val4))
    assert_true(implements_comparable_bool(bool_val4))
    assert_true(implements_comparable_float(float_val4))
    assert_true(implements_comparable_string(str_val4))

    test_start("reflection deep equality")

    fr fr Test integer equality
    sus int_val5 normie = 42
    sus int_val6 normie = 42
    sus int_val7 normie = 24

    assert_true(deep_equal_int(int_val5, int_val6))
    assert_false(deep_equal_int(int_val5, int_val7))

    fr fr Test boolean equality
    sus bool_val5 lit = based
    sus bool_val6 lit = based
    sus bool_val7 lit = cap

    assert_true(deep_equal_bool(bool_val5, bool_val6))
    assert_false(deep_equal_bool(bool_val5, bool_val7))

    fr fr Test float equality
    sus float_val5 meal = 3.14
    sus float_val6 meal = 3.14
    sus float_val7 meal = 2.71

    assert_true(deep_equal_float(float_val5, float_val6))
    assert_false(deep_equal_float(float_val5, float_val7))

    fr fr Test string equality
    sus str_val5 tea = "hello"
    sus str_val6 tea = "hello"
    sus str_val7 tea = "world"

    assert_true(deep_equal_string(str_val5, str_val6))
    assert_false(deep_equal_string(str_val5, str_val7))

    test_start("reflection zero values")

    fr fr Test zero values for all types
    sus zero_int2 normie = get_zero_int()
    assert_eq_int(zero_int2, 0)

    sus zero_bool2 lit = get_zero_bool()
    assert_false(zero_bool2)

    sus zero_float2 meal = get_zero_float()
    assert_eq_int(zero_float2, 0.0)

    sus zero_str2 tea = get_zero_string()
    assert_eq_string(zero_str2, "")

    test_start("reflection string parsing")

    fr fr Test string parsing utilities
    assert_eq_int(parse_string_to_int("0"), 0)
    assert_eq_int(parse_string_to_int("1"), 1)
    assert_eq_int(parse_string_to_int("42"), 42)
    assert_eq_int(parse_string_to_int("unknown"), 0)

    assert_eq_int(parse_string_to_float("0.0"), 0.0)
    assert_eq_int(parse_string_to_float("3.14"), 3.14)
    assert_eq_int(parse_string_to_float("unknown"), 0.0)

    assert_true(parse_string_to_bool("true"))
    assert_false(parse_string_to_bool("false"))
    assert_false(parse_string_to_bool("unknown"))

    test_start("reflection type checking utilities")

    fr fr Test numeric type checking
    sus numeric_int normie = 42
    assert_true(is_numeric_type_int(numeric_int))

    sus numeric_bool lit = based
    assert_false(is_numeric_type_bool(numeric_bool))

    sus numeric_float meal = 3.14
    assert_true(is_numeric_type_float(numeric_float))

    sus numeric_string tea = "hello"
    assert_false(is_numeric_type_string(numeric_string))

    fr fr Test comparable type checking
    assert_true(is_comparable_type_int(numeric_int))
    assert_true(is_comparable_type_bool(numeric_bool))
    assert_true(is_comparable_type_float(numeric_float))
    assert_true(is_comparable_type_string(numeric_string))

    fr fr Test type conversion checking
    assert_true(can_convert_int_to_string(numeric_int))
    assert_true(can_convert_int_to_float(numeric_int))
    assert_true(can_convert_float_to_int(numeric_float))
    assert_true(can_convert_bool_to_string(numeric_bool))

    test_start("reflection system demo")

    fr fr Test the main demonstration function
    sus demo_result lit = reflection_demo()
    assert_true(demo_result)

    print_test_summary()
}

fr fr Run the tests
run_reflection_tests()
