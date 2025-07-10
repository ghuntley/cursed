yeet "testz"
yeet "reflection"

// Test Reflection Loading
test_start("reflect_load_type_struct")
sus result lit = reflect_load_type("my_struct")
assert_true(result)
assert_true(reflect_is_loaded())
assert_eq_string(reflect_get_type(), "struct")
assert_eq_string(reflect_get_target(), "my_struct")

test_start("reflect_load_type_interface")
reflect_clear()
sus result2 lit = reflect_load_type("my_interface")
assert_true(result2)
assert_eq_string(reflect_get_type(), "interface")

test_start("reflect_load_type_function")
reflect_clear()
sus result3 lit = reflect_load_type("my_function")
assert_true(result3)
assert_eq_string(reflect_get_type(), "function")

test_start("reflect_load_value")
reflect_clear()
sus result4 lit = reflect_load_value("42")
assert_true(result4)
assert_eq_string(reflect_get_type(), "primitive")

test_start("reflect_clear")
reflect_load_type("test_struct")
assert_true(reflect_is_loaded())
reflect_clear()
assert_false(reflect_is_loaded())

// Test Type Information
test_start("reflect_is_struct")
reflect_load_type("test_struct")
assert_true(reflect_is_struct())
assert_false(reflect_is_interface())
assert_false(reflect_is_function())
assert_false(reflect_is_primitive())

test_start("reflect_is_interface")
reflect_load_type("test_interface")
assert_true(reflect_is_interface())
assert_false(reflect_is_struct())

test_start("reflect_is_function")
reflect_load_type("test_function")
assert_true(reflect_is_function())
assert_false(reflect_is_struct())

test_start("reflect_is_primitive")
reflect_load_value("hello")
assert_true(reflect_is_primitive())
assert_false(reflect_is_struct())

// Test Field Reflection
test_start("reflect_get_field_names")
reflect_load_type("test_struct")
sus field_names tea = reflect_get_field_names()
assert_true(field_names.contains("field1"))
assert_true(field_names.contains("field2"))
assert_true(field_names.contains("field3"))

test_start("reflect_get_field_count")
reflect_load_type("test_struct")
sus field_count normie = reflect_get_field_count()
assert_eq_int(field_count, 3)

test_start("reflect_get_field_type")
reflect_load_type("test_struct")
assert_eq_string(reflect_get_field_type("field1"), "tea")
assert_eq_string(reflect_get_field_type("field2"), "normie")
assert_eq_string(reflect_get_field_type("field3"), "lit")

test_start("reflect_has_field")
reflect_load_type("test_struct")
assert_true(reflect_has_field("field1"))
assert_true(reflect_has_field("field2"))
assert_false(reflect_has_field("nonexistent"))

test_start("reflect_get_field_value")
reflect_load_type("test_struct")
assert_eq_string(reflect_get_field_value("field1"), "field1_value")
assert_eq_string(reflect_get_field_value("field2"), "42")
assert_eq_string(reflect_get_field_value("field3"), "true")

test_start("reflect_set_field_value")
reflect_load_type("test_struct")
assert_true(reflect_set_field_value("field1", "new_value"))
assert_false(reflect_set_field_value("nonexistent", "value"))

// Test Method Reflection
test_start("reflect_get_method_names")
reflect_load_type("test_struct")
sus method_names tea = reflect_get_method_names()
assert_true(method_names.contains("method1"))
assert_true(method_names.contains("method2"))
assert_true(method_names.contains("method3"))

test_start("reflect_get_method_count")
reflect_load_type("test_struct")
sus method_count normie = reflect_get_method_count()
assert_eq_int(method_count, 3)

test_start("reflect_has_method")
reflect_load_type("test_struct")
assert_true(reflect_has_method("method1"))
assert_true(reflect_has_method("method2"))
assert_false(reflect_has_method("nonexistent"))

test_start("reflect_get_method_signature")
reflect_load_type("test_struct")
sus signature tea = reflect_get_method_signature("method1")
assert_true(signature.contains("slay"))
assert_true(signature.contains("method1"))

test_start("reflect_call_method")
reflect_load_type("test_struct")
sus result_call tea = reflect_call_method("method1", "arg1,arg2")
assert_eq_string(result_call, "method_result_method1")

// Test Interface Reflection
test_start("reflect_implements_interface")
reflect_load_type("test_struct")
assert_false(reflect_implements_interface("Stringer"))
assert_false(reflect_implements_interface("Reader"))

test_start("reflect_get_interfaces")
reflect_load_type("test_struct")
sus interfaces tea = reflect_get_interfaces()
assert_true(interfaces != "")

// Test Type Conversion
test_start("reflect_convert_to_string")
sus string_result tea = reflect_convert_to_string("42")
assert_eq_string(string_result, "string_42")

test_start("reflect_convert_to_int")
sus int_result normie = reflect_convert_to_int("42")
assert_eq_int(int_result, 42)

test_start("reflect_convert_to_bool")
sus bool_result lit = reflect_convert_to_bool("true")
assert_true(bool_result)

test_start("reflect_can_convert_to")
reflect_load_value("42")
assert_true(reflect_can_convert_to("tea"))
assert_true(reflect_can_convert_to("normie"))
assert_true(reflect_can_convert_to("lit"))

// Test Dynamic Creation
test_start("reflect_create_instance")
sus create_result lit = reflect_create_instance("test_struct")
assert_true(create_result)

test_start("reflect_clone_instance")
reflect_load_type("test_struct")
sus clone_result lit = reflect_clone_instance()
assert_true(clone_result)

// Test Metadata
test_start("reflect_get_tags")
reflect_load_type("test_struct")
sus tags tea = reflect_get_tags("field1")
assert_true(tags.contains("json:"))
assert_true(tags.contains("xml:"))

test_start("reflect_has_tag")
reflect_load_type("test_struct")
assert_true(reflect_has_tag("field1", "json"))
assert_true(reflect_has_tag("field2", "validate"))
assert_false(reflect_has_tag("field1", "nonexistent"))

test_start("reflect_get_tag_value")
reflect_load_type("test_struct")
assert_eq_string(reflect_get_tag_value("field1", "json"), "field1")
assert_eq_string(reflect_get_tag_value("field2", "validate"), "required")

// Test Package and Module Reflection
test_start("reflect_get_package_name")
sus package_name tea = reflect_get_package_name()
assert_eq_string(package_name, "main")

test_start("reflect_get_module_functions")
sus module_functions tea = reflect_get_module_functions()
assert_true(module_functions.contains("function1"))
assert_true(module_functions.contains("function2"))

test_start("reflect_get_module_types")
sus module_types tea = reflect_get_module_types()
assert_true(module_types.contains("struct1"))
assert_true(module_types.contains("interface1"))

test_start("reflect_get_module_constants")
sus module_constants tea = reflect_get_module_constants()
assert_true(module_constants.contains("CONSTANT1"))

// Test Runtime Information
test_start("reflect_get_runtime_info")
reflect_load_type("test_struct")
sus runtime_info tea = reflect_get_runtime_info()
assert_true(runtime_info.contains("reflection_enabled:true"))
assert_true(runtime_info.contains("version:"))
assert_true(runtime_info.contains("target:"))

test_start("reflect_is_reflection_enabled")
assert_true(reflect_is_reflection_enabled())

test_start("reflect_get_type_size")
assert_eq_int(reflect_get_type_size("tea"), 8)
assert_eq_int(reflect_get_type_size("normie"), 4)
assert_eq_int(reflect_get_type_size("lit"), 1)

test_start("reflect_get_type_alignment")
assert_eq_int(reflect_get_type_alignment("tea"), 8)
assert_eq_int(reflect_get_type_alignment("normie"), 4)
assert_eq_int(reflect_get_type_alignment("lit"), 1)

// Test Error Handling
test_start("operations_without_loaded_reflection")
reflect_clear()
assert_eq_string(reflect_get_type(), "")
assert_false(reflect_is_struct())
assert_false(reflect_is_interface())
assert_eq_string(reflect_get_field_names(), "")
assert_eq_int(reflect_get_field_count(), 0)
assert_eq_string(reflect_get_field_type("field1"), "")
assert_false(reflect_has_field("field1"))
assert_eq_string(reflect_get_field_value("field1"), "")
assert_false(reflect_set_field_value("field1", "value"))
assert_eq_string(reflect_get_method_names(), "")
assert_eq_int(reflect_get_method_count(), 0)
assert_false(reflect_has_method("method1"))
assert_eq_string(reflect_get_method_signature("method1"), "")
assert_eq_string(reflect_call_method("method1", "args"), "")
assert_false(reflect_implements_interface("Stringer"))
assert_eq_string(reflect_get_interfaces(), "")
assert_false(reflect_can_convert_to("tea"))
assert_false(reflect_clone_instance())
assert_eq_string(reflect_get_tags("field1"), "")
assert_false(reflect_has_tag("field1", "json"))
assert_eq_string(reflect_get_tag_value("field1", "json"), "")

print_test_summary()
