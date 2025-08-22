yeet "testz"
yeet "reflect"
yeet "stringz"
yeet "vibez"

test_start("Advanced Reflection and Metaprogramming Tests")

fr fr Initialize reflection system
reflect.init_reflection()

fr fr Test 1: Dynamic Object Creation and Manipulation
vibez.spill("=== Test 1: Dynamic Object Creation ===")

fr fr Create a dynamic struct type with multiple field types
sus person_fields := []tea{"name", "age", "salary", "active"}
sus person_type := reflect.type_info_struct_simple("Employee", person_fields)

assert_true(reflect.is_struct_type(person_type))
assert_eq_int(reflect.get_struct_field_count(person_type), 4)
assert_eq_string(reflect.get_struct_field_name(person_type, 0), "name")
assert_eq_string(reflect.get_struct_field_name(person_type, 3), "active")

vibez.spill("Dynamic struct type created: " + reflect.get_type_name(person_type))

fr fr Test 2: Runtime Method Discovery and Invocation
vibez.spill("=== Test 2: Runtime Method Discovery ===")

fr fr Create a comprehensive type with methods
sus calculator_methods := []tea{"add", "subtract", "multiply", "divide", "power"}
sus calculator_interface := reflect.type_info_interface_simple("Calculator", calculator_methods)

assert_true(reflect.is_interface_type(calculator_interface))
assert_eq_int(reflect.get_interface_method_count(calculator_interface), 5)
assert_true(reflect.has_interface_method(calculator_interface, "multiply"))
assert_false(reflect.has_interface_method(calculator_interface, "modulo"))

vibez.spill("Calculator interface methods discovered: " + 
           fmt.format_int(reflect.get_interface_method_count(calculator_interface)))

fr fr Test 3: Generic Type Reflection and Specialization
vibez.spill("=== Test 3: Generic Type Reflection ===")

fr fr Create a generic container type
sus container_fields := []tea{"data", "size", "capacity"}
sus container_base := reflect.type_info_struct_simple("Container", container_fields)

fr fr Create generic parameters
sus generic_params := []reflect.GenericParam{
    reflect.GenericParam{
        name: "T",
        constraints: []tea{"Comparable"},
        default_type: "normie",
        bounds: reflect.type_info_int()
    }
}

sus generic_container := reflect.type_info_generic("Container", generic_params, container_base)
assert_true(reflect.is_generic_type(generic_container))
assert_true(reflect.is_generic_instance(generic_container))

sus params := reflect.get_generic_parameters(generic_container)
assert_eq_int(len(params), 1)
assert_eq_string(params[0].name, "T")

vibez.spill("Generic container created with parameter: " + params[0].name)

fr fr Test 4: Complex Array and Pointer Reflection
vibez.spill("=== Test 4: Complex Array and Pointer Types ===")

fr fr Test multi-dimensional arrays
sus matrix_type := reflect.type_info_array_simple("snack", 100) fr fr 10x10 matrix simulation
assert_true(reflect.is_array_type(matrix_type))
assert_eq_string(reflect.get_array_element_type(matrix_type), "snack")
assert_eq_int(reflect.get_array_size(matrix_type), 100)

vibez.spill("Matrix type: " + reflect.get_type_name(matrix_type))

fr fr Test pointer types
sus int_type := reflect.type_info_int()
sus int_ptr_type := reflect.type_info_ptr(int_type)
assert_true(reflect.is_ptr_type(int_ptr_type))
assert_eq_string(reflect.get_type_name(int_ptr_type), "*normie")

sus base_type := reflect.get_base_type(int_ptr_type)
assert_true(base_type != nil)
assert_eq_string(base_type.name, "normie")

vibez.spill("Pointer type created: " + reflect.get_type_name(int_ptr_type))

fr fr Test 5: Advanced Value Manipulation and Conversion
vibez.spill("=== Test 5: Advanced Value Manipulation ===")

fr fr Create complex values with different types
sus complex_int := reflect.value_from_int(9876)
sus complex_string := reflect.value_from_string("metaprogramming")
sus complex_float := reflect.value_from_float(98.76)
sus complex_bool := reflect.value_from_bool(based)

fr fr Test advanced conversions
sus int_to_float := reflect.convert_value(complex_int, reflect.type_info_float())
assert_true(reflect.is_valid(int_to_float))
assert_eq_string(reflect.value_type_name(int_to_float), "snack")

sus float_to_string := reflect.convert_value(complex_float, reflect.type_info_string())
assert_true(reflect.is_valid(float_to_string))
assert_eq_string(reflect.value_type_name(float_to_string), "tea")

vibez.spill("Complex conversions successful")

fr fr Test 6: Inheritance Chain and Interface Compatibility
vibez.spill("=== Test 6: Inheritance and Interface Compatibility ===")

fr fr Create inheritance hierarchy
sus animal_fields := []tea{"species", "age"}
sus animal_type := reflect.type_info_struct_simple("Animal", animal_fields)

sus mammal_fields := []tea{"fur_color", "warm_blooded"}
sus mammal_type := reflect.type_info_struct_simple("Mammal", mammal_fields)
mammal_type.base_type = &animal_type

sus dog_fields := []tea{"breed", "owner"}
sus dog_type := reflect.type_info_struct_simple("Dog", dog_fields)
dog_type.base_type = &mammal_type

fr fr Test inheritance chain
assert_true(reflect.is_derived_from(dog_type, animal_type))
assert_true(reflect.is_derived_from(mammal_type, animal_type))
assert_false(reflect.is_derived_from(animal_type, dog_type))

sus inheritance_chain := reflect.get_inheritance_chain(dog_type)
assert_true(len(inheritance_chain) >= 2)

vibez.spill("Inheritance chain length: " + fmt.format_int(len(inheritance_chain)))

fr fr Test 7: Attribute and Metadata Reflection
vibez.spill("=== Test 7: Attribute and Metadata Reflection ===")

fr fr Create types with attributes
sus annotated_type := reflect.type_info_struct_simple("AnnotatedStruct", []tea{"field1", "field2"})
annotated_type.attributes = []tea{"serializable", "cacheable", "immutable"}

assert_true(reflect.has_attribute(annotated_type, "serializable"))
assert_true(reflect.has_attribute(annotated_type, "cacheable"))
assert_false(reflect.has_attribute(annotated_type, "deprecated"))

sus all_attributes := reflect.get_attributes(annotated_type)
assert_eq_int(len(all_attributes), 3)

vibez.spill("Attributes found: " + fmt.format_int(len(all_attributes)))

fr fr Test 8: Runtime Type Registry and Discovery
vibez.spill("=== Test 8: Runtime Type Registry ===")

fr fr Register custom types
reflect.register_type_by_name("CustomWidget")
reflect.register_type_by_name("DataProcessor")
reflect.register_type_by_name("NetworkHandler")

assert_true(reflect.is_type_registered("CustomWidget"))
assert_true(reflect.is_type_registered("DataProcessor"))
assert_true(reflect.is_type_registered("NetworkHandler"))

sus all_registered := reflect.get_registered_types()
assert_true(stringz.len(all_registered) > 0)

vibez.spill("Total registered types: " + fmt.format_int(reflect.stringz_count_char_simple(all_registered, ",") + 1))

fr fr Test 9: Performance and Memory Management
vibez.spill("=== Test 9: Performance and Memory Management ===")

fr fr Create many values and test reference counting
bestie i := 0; i < 100; i++ {
    sus test_value := reflect.value_from_int(i)
    assert_true(reflect.is_valid(test_value))
    assert_eq_int(test_value.ref_count, 1)
    
    fr fr Test reference counting
    reflect.retain_value(&test_value)
    assert_eq_int(test_value.ref_count, 2)
    
    reflect.release_value(&test_value)
    assert_eq_int(test_value.ref_count, 1)
    
    reflect.release_value(&test_value)
    assert_eq_int(test_value.ref_count, 0)
    assert_false(reflect.is_valid(test_value))
}

vibez.spill("Memory management test completed - 100 values created and cleaned")

fr fr Test 10: Method Calling and Dynamic Dispatch
vibez.spill("=== Test 10: Method Calling and Dynamic Dispatch ===")

fr fr Create a string value and call its methods
sus text_value := reflect.value_from_string("hello world")
assert_true(reflect.is_valid(text_value))

fr fr Test method existence
assert_true(reflect.has_method(text_value.type_info, "len"))
assert_true(reflect.has_method(text_value.type_info, "substr"))

fr fr Call len method
sus length_result := reflect.call_method(text_value, "len", []reflect.Value{})
assert_true(reflect.is_valid(length_result))
assert_eq_string(reflect.value_type_name(length_result), "normie")
assert_eq_string(reflect.get_value_data(length_result), "11")

vibez.spill("String length method called: " + reflect.get_value_data(length_result))

fr fr Call substr method with arguments
sus start_arg := reflect.value_from_int(0)
sus end_arg := reflect.value_from_int(5)
sus substr_result := reflect.call_method(text_value, "substr", []reflect.Value{start_arg, end_arg})
assert_true(reflect.is_valid(substr_result))
assert_eq_string(reflect.value_type_name(substr_result), "tea")

vibez.spill("Substring method called successfully")

fr fr Test 11: Complete Type Introspection
vibez.spill("=== Test 11: Complete Type Introspection ===")

fr fr Test comprehensive type analysis
bestie i := 0; i < len(reflect.registered_types); i++ {
    sus current_type := reflect.registered_types[i]
    sus type_summary := "Type: " + reflect.get_type_name(current_type) + 
                        " (Kind: " + fmt.format_int(reflect.get_type_kind(current_type)) + 
                        ", Size: " + fmt.format_int(reflect.get_type_size(current_type)) + ")"
    
    bestie reflect.is_struct_type(current_type) {
        type_summary = type_summary + " - Struct with " + 
                       fmt.format_int(reflect.get_struct_field_count(current_type)) + " fields"
    }
    
    bestie reflect.is_interface_type(current_type) {
        type_summary = type_summary + " - Interface with " + 
                       fmt.format_int(reflect.get_interface_method_count(current_type)) + " methods"
    }
    
    vibez.spill(type_summary)
}

vibez.spill("=== Advanced Reflection Tests Complete ===")
vibez.spill("All metaprogramming capabilities validated successfully!")

print_test_summary()
