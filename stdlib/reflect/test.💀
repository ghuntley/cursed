yeet "testz"
yeet "stringz"
yeet "fmt"

fr fr Comprehensive test suite for reflection system
fr fr Tests type information, value operations, and runtime introspection

sus main() {
    test_start("Reflection system comprehensive tests")
    
    fr fr Initialize reflection system
    init_reflection()
    
    fr fr Basic type information tests
    test_basic_type_info()
    test_type_checking()
    test_complex_types()
    
    fr fr Value operation tests
    test_value_creation()
    test_value_operations()
    test_type_conversion()
    
    fr fr Field and method tests
    test_field_operations()
    test_method_operations()
    test_struct_reflection()
    
    fr fr Advanced reflection tests
    test_type_registry()
    test_inheritance()
    test_attributes()
    
    fr fr Interface and generic tests
    test_interface_reflection()
    test_array_reflection()
    test_function_reflection()
    
    fr fr Memory management tests
    test_value_lifecycle()
    
    print_test_summary()
}

fr fr Basic type information tests
slay test_basic_type_info() {
    test_group("Basic type information")
    
    fr fr Test integer type info
    sus int_type TypeInfo = type_info_int()
    assert_eq_int(int_type.kind, TYPE_INT)
    assert_eq_string(int_type.name, "normie")
    assert_eq_int(int_type.size, 4)
    assert_true(is_int_type(int_type))
    assert_false(is_string_type(int_type))
    
    fr fr Test string type info
    sus string_type TypeInfo = type_info_string()
    assert_eq_int(string_type.kind, TYPE_STRING)
    assert_eq_string(string_type.name, "tea")
    assert_eq_int(string_type.size, 8)
    assert_true(is_string_type(string_type))
    assert_false(is_int_type(string_type))
    
    fr fr Test boolean type info
    sus bool_type TypeInfo = type_info_bool()
    assert_eq_int(bool_type.kind, TYPE_BOOL)
    assert_eq_string(bool_type.name, "lit")
    assert_eq_int(bool_type.size, 1)
    assert_true(is_bool_type(bool_type))
    
    fr fr Test float types
    sus float_type TypeInfo = type_info_float()
    assert_eq_int(float_type.kind, TYPE_FLOAT)
    assert_eq_string(float_type.name, "snack")
    assert_eq_int(float_type.size, 4)
    assert_true(is_float_type(float_type))
    
    sus double_type TypeInfo = type_info_float64()
    assert_eq_string(double_type.name, "meal")
    assert_eq_int(double_type.size, 8)
    
    fr fr Test character type
    sus char_type TypeInfo = type_info_char()
    assert_eq_string(char_type.name, "sip")
    assert_eq_int(char_type.size, 1)
    
    pass("Basic type information correct")
}

slay test_type_checking() {
    test_group("Type checking functions")
    
    sus int_type TypeInfo = type_info_int()
    sus string_type TypeInfo = type_info_string()
    sus bool_type TypeInfo = type_info_bool()
    sus float_type TypeInfo = type_info_float()
    
    fr fr Test type checking functions
    assert_true(is_int_type(int_type))
    assert_false(is_int_type(string_type))
    
    assert_true(is_string_type(string_type))
    assert_false(is_string_type(int_type))
    
    assert_true(is_bool_type(bool_type))
    assert_false(is_bool_type(float_type))
    
    assert_true(is_float_type(float_type))
    assert_false(is_float_type(bool_type))
    
    pass("Type checking functions work correctly")
}

slay test_complex_types() {
    test_group("Complex type construction")
    
    fr fr Test struct type creation
    sus field_names tea[value] = ["name", "age"]
    sus person_type TypeInfo = type_info_struct_simple("Person", field_names)
    
    assert_eq_int(person_type.kind, TYPE_STRUCT)
    assert_eq_string(person_type.name, "Person")
    assert_true(is_struct_type(person_type))
    assert_eq_int(len(person_type.fields), 2)
    
    fr fr Test array type creation
    sus int_type TypeInfo = type_info_int()
    sus array_type TypeInfo = type_info_array(int_type, 10)
    
    assert_eq_int(array_type.kind, TYPE_ARRAY)
    assert_true(is_array_type(array_type))
    assert_eq_int(array_type.size, 40)  fr fr 10 * 4 bytes
    
    fr fr Test pointer type creation
    sus ptr_type TypeInfo = type_info_ptr(int_type)
    
    assert_eq_int(ptr_type.kind, TYPE_PTR)
    assert_eq_int(ptr_type.size, 8)  fr fr Pointer size
    assert_true(is_ptr_type(ptr_type))
    
    pass("Complex types created correctly")
}

fr fr Value operation tests
slay test_value_creation() {
    test_group("Value creation")
    
    fr fr Test creating values from primitives
    sus int_val Value = value_from_int(42)
    assert_true(int_val.is_valid)
    assert_eq_int(int_val.type_info.kind, TYPE_INT)
    assert_eq_int(int_val.ref_count, 1)
    
    sus str_val Value = value_from_string("hello")
    assert_true(str_val.is_valid)
    assert_eq_int(str_val.type_info.kind, TYPE_STRING)
    assert_eq_string(str_val.data_ptr, "hello")
    
    sus bool_val Value = value_from_bool(based)
    assert_true(bool_val.is_valid)
    assert_eq_int(bool_val.type_info.kind, TYPE_BOOL)
    assert_eq_string(bool_val.data_ptr, "based")
    
    sus float_val Value = value_from_float(3.14)
    assert_true(float_val.is_valid)
    assert_eq_int(float_val.type_info.kind, TYPE_FLOAT)
    
    fr fr Test invalid value creation
    sus invalid Value = invalid_value()
    assert_false(invalid.is_valid)
    
    pass("Values created correctly from primitives")
}

slay test_value_operations() {
    test_group("Value operations")
    
    sus val Value = value_from_int(42)
    
    fr fr Test value type queries
    assert_eq_string(value_type_name(val), "normie")
    assert_eq_int(value_type_kind(val), TYPE_INT)
    assert_true(is_valid(val))
    
    fr fr Test value data access
    assert_eq_string(get_value_data(val), "42")
    
    fr fr Test const value creation
    sus const_val Value = make_const_value(val)
    assert_true(const_val.is_const)
    
    fr fr Test value string representation
    sus val_str tea = value_to_string(val)
    assert_true(stringz.len(val_str) > 0)
    
    pass("Value operations work correctly")
}

slay test_type_conversion() {
    test_group("Type conversion")
    
    sus int_val Value = value_from_int(42)
    sus string_val Value = value_from_string("hello")
    
    fr fr Test type equality
    assert_true(types_equal(int_val.type_info, int_val.type_info))
    assert_false(types_equal(int_val.type_info, string_val.type_info))
    
    fr fr Test value equality
    sus int_val2 Value = value_from_int(42)
    sus int_val3 Value = value_from_int(43)
    
    assert_true(values_equal(int_val, int_val2))
    assert_false(values_equal(int_val, int_val3))
    assert_false(values_equal(int_val, string_val))
    
    pass("Type conversion and comparison work")
}

fr fr Field and method operation tests
slay test_field_operations() {
    test_group("Field operations")
    
    fr fr Create struct type with fields
    sus field_names tea[value] = ["name", "age", "active"]
    sus person_type TypeInfo = type_info_struct_simple("Person", field_names)
    
    fr fr Test field count
    assert_eq_int(get_struct_field_count(person_type), 3)
    
    fr fr Test field access by index
    assert_eq_string(get_struct_field_name(person_type, 0), "name")
    assert_eq_string(get_struct_field_name(person_type, 1), "age")
    assert_eq_string(get_struct_field_name(person_type, 2), "active")
    
    fr fr Test invalid field index
    assert_eq_string(get_struct_field_name(person_type, 5), "")
    
    fr fr Test field existence
    assert_true(has_struct_field(person_type, "name"))
    assert_true(has_struct_field(person_type, "age"))
    assert_false(has_struct_field(person_type, "nonexistent"))
    
    fr fr Test field lookup by name
    sus name_field *FieldInfo = get_field_by_name(person_type, "name")
    assert_true(name_field != nil)
    
    sus missing_field *FieldInfo = get_field_by_name(person_type, "missing")
    assert_true(missing_field == nil)
    
    pass("Field operations work correctly")
}

slay test_method_operations() {
    test_group("Method operations")
    
    fr fr Create interface type with methods
    sus method_names tea[value] = ["draw", "move", "resize"]
    sus drawable_type TypeInfo = type_info_interface_simple("Drawable", method_names)
    
    fr fr Test method count
    assert_eq_int(get_interface_method_count(drawable_type), 3)
    
    fr fr Test method access by index
    assert_eq_string(get_interface_method_name(drawable_type, 0), "draw")
    assert_eq_string(get_interface_method_name(drawable_type, 1), "move")
    assert_eq_string(get_interface_method_name(drawable_type, 2), "resize")
    
    fr fr Test method existence
    assert_true(has_interface_method(drawable_type, "draw"))
    assert_true(has_method(drawable_type, "move"))
    assert_false(has_interface_method(drawable_type, "nonexistent"))
    
    fr fr Test method lookup
    sus draw_method *MethodInfo = get_method_by_name(drawable_type, "draw")
    assert_true(draw_method != nil)
    
    pass("Method operations work correctly")
}

slay test_struct_reflection() {
    test_group("Struct reflection")
    
    fr fr Test built-in Person struct from init_reflection
    sus person_type *TypeInfo = find_type_by_name("Person")
    assert_true(person_type != nil)
    assert_true(is_struct_type(*person_type))
    
    fr fr Test struct field inspection
    assert_eq_int(get_struct_field_count(*person_type), 2)
    assert_true(has_struct_field(*person_type, "name"))
    assert_true(has_struct_field(*person_type, "age"))
    
    fr fr Test method inspection
    assert_true(has_method(*person_type, "get_name"))
    assert_true(has_method(*person_type, "set_age"))
    
    pass("Struct reflection works correctly")
}

fr fr Advanced reflection tests
slay test_type_registry() {
    test_group("Type registry")
    
    fr fr Test registering custom type
    register_type_by_name("CustomType")
    assert_true(is_type_registered("CustomType"))
    assert_false(is_type_registered("NonExistentType"))
    
    fr fr Test finding registered type
    sus custom_type *TypeInfo = find_type_by_name("CustomType")
    assert_true(custom_type != nil)
    assert_eq_string(custom_type.name, "CustomType")
    
    fr fr Test getting all registered types
    sus all_types TypeInfo[value] = get_all_registered_types()
    assert_true(len(all_types) > 0)
    
    fr fr Test getting registered type names
    sus type_names tea[value] = get_registered_type_names()
    assert_true(len(type_names) > 0)
    
    fr fr Test getting types as string
    sus types_str tea = get_registered_types()
    assert_true(stringz.len(types_str) > 0)
    
    pass("Type registry works correctly")
}

slay test_inheritance() {
    test_group("Type inheritance")
    
    sus base_type TypeInfo = type_info_int()
    sus derived_type TypeInfo = type_info_generic("GenericInt", GenericParam[value]{}, base_type)
    
    fr fr Test base type access
    sus base_ptr *TypeInfo = get_base_type(derived_type)
    assert_true(base_ptr != nil)
    assert_true(types_equal(*base_ptr, base_type))
    
    fr fr Test inheritance checking
    assert_true(is_derived_from(derived_type, base_type))
    assert_false(is_derived_from(base_type, derived_type))
    
    fr fr Test inheritance chain
    sus chain TypeInfo[value] = get_inheritance_chain(derived_type)
    assert_eq_int(len(chain), 2)  fr fr derived + base
    
    pass("Type inheritance works correctly")
}

slay test_attributes() {
    test_group("Attribute system")
    
    sus int_type TypeInfo = type_info_int()
    
    fr fr Test attribute existence (none on basic types)
    assert_false(has_attribute(int_type, "readonly"))
    
    fr fr Test getting attributes
    sus attrs tea[value] = get_attributes(int_type)
    assert_eq_int(len(attrs), 0)  fr fr Basic types have no attributes
    
    fr fr Test generic type attributes
    sus generic_type TypeInfo = type_info_generic("GenericType", GenericParam[value]{}, int_type)
    assert_true(has_attribute(generic_type, "generic"))
    
    pass("Attribute system works correctly")
}

fr fr Interface and generic tests
slay test_interface_reflection() {
    test_group("Interface reflection")
    
    sus method_names tea[value] = ["serialize", "deserialize"]
    sus serializable TypeInfo = type_info_interface_simple("Serializable", method_names)
    
    assert_true(is_interface_type(serializable))
    assert_eq_int(serializable.size, 16)  fr fr vtable + data pointer
    assert_eq_int(get_interface_method_count(serializable), 2)
    
    pass("Interface reflection works correctly")
}

slay test_array_reflection() {
    test_group("Array reflection")
    
    fr fr Test simple array type
    sus array_type TypeInfo = type_info_array_simple("normie", 5)
    
    assert_true(is_array_type(array_type))
    assert_eq_int(get_array_size(array_type), 5)
    assert_eq_string(get_array_element_type(array_type), "normie")
    
    fr fr Test complex array type
    sus int_type TypeInfo = type_info_int()
    sus complex_array TypeInfo = type_info_array(int_type, 10)
    
    assert_true(is_array_type(complex_array))
    assert_eq_int(complex_array.size, 40)
    
    pass("Array reflection works correctly")
}

slay test_function_reflection() {
    test_group("Function reflection")
    
    sus param_types tea[value] = ["normie", "tea"]
    sus func_type TypeInfo = type_info_func("testFunc", param_types, "lit")
    
    assert_true(is_func_type(func_type))
    assert_eq_int(get_func_param_count(func_type), 2)
    assert_eq_string(get_func_param_type(func_type, 0), "normie")
    assert_eq_string(get_func_param_type(func_type, 1), "tea")
    
    fr fr Test invalid parameter access
    assert_eq_string(get_func_param_type(func_type, 5), "")
    
    pass("Function reflection works correctly")
}

fr fr Memory management tests
slay test_value_lifecycle() {
    test_group("Value lifecycle management")
    
    sus val Value = value_from_int(42)
    assert_eq_int(val.ref_count, 1)
    
    fr fr Test reference counting
    retain_value(&val)
    assert_eq_int(val.ref_count, 2)
    
    release_value(&val)
    assert_eq_int(val.ref_count, 1)
    assert_true(val.is_valid)  fr fr Still valid with ref_count > 0
    
    release_value(&val)
    assert_eq_int(val.ref_count, 0)
    assert_false(val.is_valid)  fr fr Should be invalid now
    
    pass("Value lifecycle management works correctly")
}

fr fr Test string representation functions
slay test_string_representation() {
    test_group("String representation")
    
    sus int_type TypeInfo = type_info_int()
    sus type_str tea = type_to_string(int_type)
    
    assert_true(stringz.len(type_str) > 0)
    assert_true(stringz.contains(type_str, "normie"))
    
    sus val Value = value_from_string("test")
    sus val_str tea = value_to_string(val)
    
    assert_true(stringz.len(val_str) > 0)
    assert_true(stringz.contains(val_str, "test"))
    
    pass("String representation works correctly")
}

fr fr Mock helper functions for testing
slay stringz_contains(text tea, substr tea) lit {
    fr fr Simple contains check - in real implementation would be in stringz
    fr fr For testing, just check if substr matches the text
    damn text == substr || stringz.len(text) > stringz.len(substr)
}
