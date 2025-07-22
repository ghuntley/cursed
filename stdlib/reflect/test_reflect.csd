yeet "testz"
yeet "reflect"

test_start("Reflection module comprehensive tests")

fr fr Initialize reflection system
reflect.init_reflection()

fr fr Test type info creation
sus int_type := reflect.type_info_int()
assert_true(reflect.is_int_type(int_type))
assert_false(reflect.is_string_type(int_type))
assert_eq_string(reflect.get_type_name(int_type), "normie")
assert_eq_int(reflect.get_type_kind(int_type), 0)
assert_eq_int(reflect.get_type_size(int_type), 4)

sus string_type := reflect.type_info_string()
assert_true(reflect.is_string_type(string_type))
assert_false(reflect.is_int_type(string_type))
assert_eq_string(reflect.get_type_name(string_type), "tea")
assert_eq_int(reflect.get_type_kind(string_type), 1)
assert_eq_int(reflect.get_type_size(string_type), 8)

sus bool_type := reflect.type_info_bool()
assert_true(reflect.is_bool_type(bool_type))
assert_false(reflect.is_int_type(bool_type))
assert_eq_string(reflect.get_type_name(bool_type), "lit")
assert_eq_int(reflect.get_type_kind(bool_type), 2)
assert_eq_int(reflect.get_type_size(bool_type), 1)

sus float_type := reflect.type_info_float()
assert_true(reflect.is_float_type(float_type))
assert_false(reflect.is_int_type(float_type))
assert_eq_string(reflect.get_type_name(float_type), "snack")
assert_eq_int(reflect.get_type_kind(float_type), 3)
assert_eq_int(reflect.get_type_size(float_type), 4)

fr fr Test value creation
sus int_value := reflect.value_from_int(42)
assert_true(reflect.is_valid(int_value))
assert_eq_string(reflect.value_type_name(int_value), "normie")
assert_eq_int(reflect.value_type_kind(int_value), 0)
assert_eq_string(reflect.get_value_data(int_value), "42")

sus string_value := reflect.value_from_string("hello")
assert_true(reflect.is_valid(string_value))
assert_eq_string(reflect.value_type_name(string_value), "tea")
assert_eq_int(reflect.value_type_kind(string_value), 1)
assert_eq_string(reflect.get_value_data(string_value), "hello")

sus bool_value := reflect.value_from_bool(based)
assert_true(reflect.is_valid(bool_value))
assert_eq_string(reflect.value_type_name(bool_value), "lit")
assert_eq_int(reflect.value_type_kind(bool_value), 2)
assert_eq_string(reflect.get_value_data(bool_value), "based")

sus float_value := reflect.value_from_float(3.14)
assert_true(reflect.is_valid(float_value))
assert_eq_string(reflect.value_type_name(float_value), "snack")
assert_eq_int(reflect.value_type_kind(float_value), 3)

fr fr Test invalid value
sus invalid := reflect.invalid_value()
assert_false(reflect.is_valid(invalid))

fr fr Test type conversion
assert_true(reflect.can_convert(int_type, string_type))
assert_true(reflect.can_convert(int_type, float_type))
assert_true(reflect.can_convert(float_type, int_type))
assert_true(reflect.can_convert(bool_type, string_type))

sus converted := reflect.convert_value(int_value, string_type)
assert_true(reflect.is_valid(converted))
assert_eq_string(reflect.value_type_name(converted), "tea")
assert_eq_string(reflect.get_value_data(converted), "42")

sus converted_float := reflect.convert_value(int_value, float_type)
assert_true(reflect.is_valid(converted_float))
assert_eq_string(reflect.value_type_name(converted_float), "snack")
assert_eq_string(reflect.get_value_data(converted_float), "42.0")

fr fr Test struct type
sus struct_fields := []tea{"name", "age", "active"}
sus struct_type := reflect.type_info_struct("Person", struct_fields)
assert_true(reflect.is_struct_type(struct_type))
assert_eq_string(reflect.get_type_name(struct_type), "Person")
assert_eq_int(reflect.get_struct_field_count(struct_type), 3)
assert_eq_string(reflect.get_struct_field_name(struct_type, 0), "name")
assert_eq_string(reflect.get_struct_field_name(struct_type, 1), "age")
assert_eq_string(reflect.get_struct_field_name(struct_type, 2), "active")

assert_true(reflect.has_struct_field(struct_type, "name"))
assert_true(reflect.has_struct_field(struct_type, "age"))
assert_false(reflect.has_struct_field(struct_type, "unknown"))

fr fr Test array type
sus array_type := reflect.type_info_array("normie", 5)
assert_true(reflect.is_array_type(array_type))
assert_eq_string(reflect.get_type_name(array_type), "[5]normie")
assert_eq_string(reflect.get_array_element_type(array_type), "normie")
assert_eq_int(reflect.get_array_size(array_type), 5)

fr fr Test function type
sus func_params := []tea{"normie", "tea"}
sus func_type := reflect.type_info_func("test_func", func_params, "lit")
assert_true(reflect.is_func_type(func_type))
assert_eq_int(reflect.get_func_param_count(func_type), 2)
assert_eq_string(reflect.get_func_param_type(func_type, 0), "normie")
assert_eq_string(reflect.get_func_param_type(func_type, 1), "tea")

fr fr Test interface type
sus interface_methods := []tea{"method1", "method2"}
sus interface_type := reflect.type_info_interface("TestInterface", interface_methods)
assert_true(reflect.is_interface_type(interface_type))
assert_eq_int(reflect.get_interface_method_count(interface_type), 2)
assert_eq_string(reflect.get_interface_method_name(interface_type, 0), "method1")
assert_eq_string(reflect.get_interface_method_name(interface_type, 1), "method2")

assert_true(reflect.has_interface_method(interface_type, "method1"))
assert_true(reflect.has_interface_method(interface_type, "method2"))
assert_false(reflect.has_interface_method(interface_type, "unknown"))

fr fr Test type equality
sus int_type2 := reflect.type_info_int()
assert_true(reflect.types_equal(int_type, int_type2))
assert_false(reflect.types_equal(int_type, string_type))

fr fr Test value equality
sus int_value2 := reflect.value_from_int(42)
sus int_value3 := reflect.value_from_int(99)
assert_true(reflect.values_equal(int_value, int_value2))
assert_false(reflect.values_equal(int_value, int_value3))
assert_false(reflect.values_equal(int_value, string_value))

fr fr Test string representation
sus type_str := reflect.type_to_string(int_type)
assert_true(stringz.contains(type_str, "normie"))
assert_true(stringz.contains(type_str, "kind: 0"))
assert_true(stringz.contains(type_str, "size: 4"))

sus value_str := reflect.value_to_string(int_value)
assert_true(stringz.contains(value_str, "normie"))
assert_true(stringz.contains(value_str, "42"))

sus invalid_str := reflect.value_to_string(invalid)
assert_eq_string(invalid_str, "invalid value")

fr fr Test type registration
assert_true(reflect.is_type_registered("normie"))
assert_true(reflect.is_type_registered("tea"))
assert_true(reflect.is_type_registered("lit"))
assert_false(reflect.is_type_registered("unknown"))

reflect.register_type("CustomType")
assert_true(reflect.is_type_registered("CustomType"))

sus registered := reflect.get_registered_types()
assert_true(stringz.len(registered) >= 6)

print_test_summary()
