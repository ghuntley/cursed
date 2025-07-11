// Test CURSED Reflection Module
yeet "testz"
yeet "reflection"

// Test type registry creation
test_start("type_registry_creation")
sus registry TypeRegistry = create_type_registry()
assert_true(registry.type_count > 0)
assert_true(has_type(registry, "normie"))
assert_true(has_type(registry, "tea"))
assert_true(has_type(registry, "lit"))

// Test type info creation
test_start("type_info_creation")
sus type_info TypeInfo = create_type_info("test_type", "struct", 16)
assert_eq_string(type_info.name, "test_type")
assert_eq_string(type_info.kind, "struct")
assert_eq_int(type_info.size, 16)
assert_false(type_info.is_pointer)
assert_false(type_info.is_array)
assert_false(type_info.is_slice)

// Test built-in type lookup
test_start("builtin_type_lookup")
sus int_type TypeInfo = get_type_info(registry, "normie")
assert_eq_string(int_type.name, "normie")
assert_eq_string(int_type.kind, "int")
assert_eq_int(int_type.size, 4)

sus string_type TypeInfo = get_type_info(registry, "tea")
assert_eq_string(string_type.name, "tea")
assert_eq_string(string_type.kind, "string")
assert_eq_int(string_type.size, 16)

sus bool_type TypeInfo = get_type_info(registry, "lit")
assert_eq_string(bool_type.name, "lit")
assert_eq_string(bool_type.kind, "bool")
assert_eq_int(bool_type.size, 1)

// Test type registration
test_start("type_registration")
sus custom_type TypeInfo = create_type_info("custom", "struct", 32)
sus old_count normie = registry.type_count
registry = register_type(registry, custom_type)
assert_eq_int(registry.type_count, old_count + 1)
assert_true(has_type(registry, "custom"))

// Test reflect value creation
test_start("reflect_value_creation")
sus value_type TypeInfo = get_type_info(registry, "normie")
sus reflect_val ReflectValue = create_reflect_value(value_type, "42")
assert_true(reflect_val.is_valid)
assert_false(reflect_val.is_nil)
assert_false(reflect_val.is_zero)
assert_eq_string(reflect_val.data, "42")

// Test zero value detection
test_start("zero_value_detection")
sus zero_int_type TypeInfo = get_type_info(registry, "normie")
sus zero_val ReflectValue = create_reflect_value(zero_int_type, "0")
assert_true(zero_val.is_zero)

sus zero_string_type TypeInfo = get_type_info(registry, "tea")
sus zero_string_val ReflectValue = create_reflect_value(zero_string_type, "")
assert_true(zero_string_val.is_zero)

sus zero_bool_type TypeInfo = get_type_info(registry, "lit")
sus zero_bool_val ReflectValue = create_reflect_value(zero_bool_type, "cap")
assert_true(zero_bool_val.is_zero)

// Test value inspection
test_start("value_inspection")
sus test_type TypeInfo = get_type_info(registry, "normie")
sus test_val ReflectValue = create_reflect_value(test_type, "123")
sus retrieved_type TypeInfo = get_value_type(test_val)
assert_eq_string(retrieved_type.name, "normie")
assert_eq_string(get_value_data(test_val), "123")
assert_true(is_valid_value(test_val))
assert_false(is_nil_value(test_val))

// Test numeric type checking
test_start("numeric_type_checking")
sus int_type_info TypeInfo = get_type_info(registry, "normie")
sus float_type_info TypeInfo = get_type_info(registry, "meal")
sus string_type_info TypeInfo = get_type_info(registry, "tea")

assert_true(is_numeric_type(int_type_info))
assert_true(is_numeric_type(float_type_info))
assert_false(is_numeric_type(string_type_info))

// Test type conversion compatibility
test_start("type_conversion_compatibility")
sus from_int TypeInfo = get_type_info(registry, "normie")
sus to_float TypeInfo = get_type_info(registry, "meal")
sus to_string TypeInfo = get_type_info(registry, "tea")

assert_true(can_convert(from_int, to_float))
assert_false(can_convert(from_int, to_string))
assert_true(can_convert(to_float, from_int))

// Test struct type creation
test_start("struct_type_creation")
sus field1 FieldInfo = FieldInfo{
    name: "id",
    type_name: "normie",
    offset: 0,
    size: 4,
    is_exported: based,
    tags: {}
}
sus field2 FieldInfo = FieldInfo{
    name: "name",
    type_name: "tea",
    offset: 4,
    size: 16,
    is_exported: based,
    tags: {}
}
sus fields [FieldInfo] = [field1, field2]
sus struct_type TypeInfo = create_struct_type("Person", fields)

assert_eq_string(struct_type.name, "Person")
assert_eq_string(struct_type.kind, "struct")
assert_eq_int(struct_type.size, 20)
assert_eq_int(get_field_count(struct_type), 2)

// Test field access
test_start("field_access")
sus person_type TypeInfo = create_struct_type("Person", fields)
assert_true(has_field(person_type, "id"))
assert_true(has_field(person_type, "name"))
assert_false(has_field(person_type, "age"))

sus id_field FieldInfo = get_field_by_name(person_type, "id")
assert_eq_string(id_field.name, "id")
assert_eq_string(id_field.type_name, "normie")
assert_eq_int(id_field.offset, 0)
assert_eq_int(id_field.size, 4)
assert_true(id_field.is_exported)

sus name_field FieldInfo = get_field_by_name(person_type, "name")
assert_eq_string(name_field.name, "name")
assert_eq_string(name_field.type_name, "tea")
assert_eq_int(name_field.offset, 4)
assert_eq_int(name_field.size, 16)

// Test field access by index
test_start("field_access_by_index")
sus first_field FieldInfo = get_field_by_index(person_type, 0)
assert_eq_string(first_field.name, "id")

sus second_field FieldInfo = get_field_by_index(person_type, 1)
assert_eq_string(second_field.name, "name")

sus invalid_field FieldInfo = get_field_by_index(person_type, 10)
assert_eq_string(invalid_field.name, "")

// Test method info
test_start("method_info")
sus param1 ParameterInfo = ParameterInfo{
    name: "value",
    type_name: "normie",
    is_pointer: cap
}
sus params [ParameterInfo] = [param1]
sus method MethodInfo = MethodInfo{
    name: "set_id",
    return_type: "void",
    parameters: params,
    is_exported: based,
    is_variadic: cap
}

assert_eq_string(method.name, "set_id")
assert_eq_string(method.return_type, "void")
assert_eq_int(len(method.parameters), 1)
assert_true(method.is_exported)
assert_false(method.is_variadic)

// Test array type creation
test_start("array_type_creation")
sus array_type TypeInfo = create_array_type("normie", 10)
assert_eq_string(array_type.name, "[10]normie")
assert_eq_string(array_type.kind, "array")
assert_eq_int(array_type.size, 40)  // 10 * 4
assert_true(array_type.is_array)
assert_false(array_type.is_slice)
assert_false(array_type.is_pointer)
assert_eq_string(array_type.element_type, "normie")

// Test slice type creation
test_start("slice_type_creation")
sus slice_type TypeInfo = create_slice_type("normie")
assert_eq_string(slice_type.name, "[]normie")
assert_eq_string(slice_type.kind, "slice")
assert_eq_int(slice_type.size, 24)  // Slice header size
assert_false(slice_type.is_array)
assert_true(slice_type.is_slice)
assert_false(slice_type.is_pointer)
assert_eq_string(slice_type.element_type, "normie")

// Test pointer type creation
test_start("pointer_type_creation")
sus pointer_type TypeInfo = create_pointer_type("normie")
assert_eq_string(pointer_type.name, "*normie")
assert_eq_string(pointer_type.kind, "pointer")
assert_eq_int(pointer_type.size, 8)  // Pointer size
assert_false(pointer_type.is_array)
assert_false(pointer_type.is_slice)
assert_true(pointer_type.is_pointer)
assert_eq_string(pointer_type.element_type, "normie")

// Test type checking functions
test_start("type_checking_functions")
sus arr_type TypeInfo = create_array_type("normie", 5)
sus sli_type TypeInfo = create_slice_type("normie")
sus ptr_type TypeInfo = create_pointer_type("normie")

assert_true(is_array_type(arr_type))
assert_false(is_slice_type(arr_type))
assert_false(is_pointer_type(arr_type))

assert_false(is_array_type(sli_type))
assert_true(is_slice_type(sli_type))
assert_false(is_pointer_type(sli_type))

assert_false(is_array_type(ptr_type))
assert_false(is_slice_type(ptr_type))
assert_true(is_pointer_type(ptr_type))

// Test element type access
test_start("element_type_access")
sus elem_arr_type TypeInfo = create_array_type("tea", 3)
sus elem_sli_type TypeInfo = create_slice_type("tea")
sus elem_ptr_type TypeInfo = create_pointer_type("tea")

assert_eq_string(get_element_type(elem_arr_type), "tea")
assert_eq_string(get_element_type(elem_sli_type), "tea")
assert_eq_string(get_element_type(elem_ptr_type), "tea")

// Test interface support
test_start("interface_support")
sus interface_type TypeInfo = create_type_info("Writer", "interface", 8)
interface_type = add_interface(interface_type, "Writer")
assert_true(implements_interface(interface_type, "Writer"))
assert_false(implements_interface(interface_type, "Reader"))

sus interfaces [tea] = get_implemented_interfaces(interface_type)
assert_eq_int(len(interfaces), 1)
assert_eq_string(interfaces[0], "Writer")

// Test type equality
test_start("type_equality")
sus type1 TypeInfo = create_type_info("Test", "struct", 16)
sus type2 TypeInfo = create_type_info("Test", "struct", 16)
sus type3 TypeInfo = create_type_info("Other", "struct", 16)

assert_true(types_equal(type1, type2))
assert_false(types_equal(type1, type3))

// Test type size utilities
test_start("type_size_utilities")
assert_eq_int(get_type_size("normie"), 4)
assert_eq_int(get_type_size("thicc"), 8)
assert_eq_int(get_type_size("smol"), 1)
assert_eq_int(get_type_size("mid"), 2)
assert_eq_int(get_type_size("meal"), 8)
assert_eq_int(get_type_size("snack"), 4)
assert_eq_int(get_type_size("drip"), 4)
assert_eq_int(get_type_size("lit"), 1)
assert_eq_int(get_type_size("tea"), 16)
assert_eq_int(get_type_size("sip"), 1)
assert_eq_int(get_type_size("byte"), 1)

// Test parsing functions
test_start("parsing_functions")
assert_eq_int(parse_int("0"), 0)
assert_eq_int(parse_int("1"), 1)
assert_eq_int(parse_int("42"), 42)

assert_true(parse_float("0.0") == 0.0)
assert_true(parse_float("1.0") == 1.0)
assert_true(parse_float("3.14") == 3.14)

assert_true(parse_bool("true"))
assert_true(parse_bool("based"))
assert_false(parse_bool("false"))
assert_false(parse_bool("cap"))

// Test type info accessors
test_start("type_info_accessors")
sus accessor_type TypeInfo = create_type_info("AccessorTest", "struct", 32)
assert_eq_string(get_type_name(accessor_type), "AccessorTest")
assert_eq_string(get_type_kind(accessor_type), "struct")
assert_eq_int(get_type_size_info(accessor_type), 32)

// Test field info accessors
test_start("field_info_accessors")
sus tag_map map[tea]tea = {}
tag_map["json"] = "id"
tag_map["db"] = "user_id"

sus tagged_field FieldInfo = FieldInfo{
    name: "id",
    type_name: "normie",
    offset: 0,
    size: 4,
    is_exported: based,
    tags: tag_map
}

assert_true(is_exported_field(tagged_field))
assert_eq_string(get_field_tag(tagged_field, "json"), "id")
assert_eq_string(get_field_tag(tagged_field, "db"), "user_id")
assert_eq_string(get_field_tag(tagged_field, "nonexistent"), "")
assert_true(has_field_tag(tagged_field, "json"))
assert_true(has_field_tag(tagged_field, "db"))
assert_false(has_field_tag(tagged_field, "nonexistent"))

// Test method info accessors
test_start("method_info_accessors")
sus exported_method MethodInfo = MethodInfo{
    name: "Export",
    return_type: "tea",
    parameters: [],
    is_exported: based,
    is_variadic: cap
}

assert_true(is_exported_method(exported_method))

sus private_method MethodInfo = MethodInfo{
    name: "private",
    return_type: "void",
    parameters: [],
    is_exported: cap,
    is_variadic: cap
}

assert_false(is_exported_method(private_method))

// Test all types retrieval
test_start("all_types_retrieval")
sus test_registry TypeRegistry = create_type_registry()
sus extra_type TypeInfo = create_type_info("ExtraType", "struct", 8)
test_registry = register_type(test_registry, extra_type)

sus all_types [TypeInfo] = get_all_types(test_registry)
assert_true(len(all_types) > 10)  // Should have built-in types plus our extra type

// Test struct size calculation
test_start("struct_size_calculation")
sus size_field1 FieldInfo = FieldInfo{
    name: "field1",
    type_name: "normie",
    offset: 0,
    size: 4,
    is_exported: based,
    tags: {}
}
sus size_field2 FieldInfo = FieldInfo{
    name: "field2",
    type_name: "thicc",
    offset: 4,
    size: 8,
    is_exported: based,
    tags: {}
}
sus size_fields [FieldInfo] = [size_field1, size_field2]
sus calculated_size normie = calculate_struct_size(size_fields)
assert_eq_int(calculated_size, 12)  // 4 + 8

print_test_summary()
