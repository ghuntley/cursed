fr fr Test suite for typez module - Type reflection and type system
yeet "testz"
yeet "typez"

fr fr ===== TYPE REGISTRATION TESTS =====

slay test_primitive_types() lit {
    testz.test_start("Primitive type registration and lookup")
    
    fr fr Initialize type system
    typez.init_type_system()
    
    fr fr Test primitive types are registered
    sus tea_type typez.TypeInfo = typez.get_type_by_name("tea")
    testz.assert_eq_int(tea_type.type_id, 1)
    testz.assert_eq_string(tea_type.name, "tea")
    testz.assert_eq_int(tea_type.size, 8)
    testz.assert_true(tea_type.is_primitive)
    testz.assert_false(tea_type.is_struct)
    
    sus normie_type typez.TypeInfo = typez.get_type_by_name("normie")
    testz.assert_eq_int(normie_type.type_id, 2)
    testz.assert_eq_int(normie_type.size, 4)
    testz.assert_true(normie_type.is_primitive)
    
    sus lit_type typez.TypeInfo = typez.get_type_by_name("lit")
    testz.assert_eq_int(lit_type.type_id, 5)
    testz.assert_eq_int(lit_type.size, 1)
    testz.assert_true(lit_type.is_primitive)
    
    damn based
}

slay test_struct_type_registration() lit {
    testz.test_start("Struct type registration")
    
    fr fr Create field info for a test struct
    sus name_field typez.FieldInfo = typez.FieldInfo{
        name: "name",
        type_info: typez.get_type_by_name("tea"),
        offset: 0,
        tag: "json:\"name\""
    }
    
    sus age_field typez.FieldInfo = typez.FieldInfo{
        name: "age",
        type_info: typez.get_type_by_name("normie"),
        offset: 8,
        tag: "json:\"age\""
    }
    
    sus fields typez[value].FieldInfo = [name_field, age_field]
    sus methods typez[value].MethodInfo = []
    
    fr fr Register struct type
    sus person_type_id normie = typez.register_struct_type("Person", fields, methods)
    testz.assert_true(person_type_id > 0)
    
    fr fr Verify struct type was registered
    sus person_type typez.TypeInfo = typez.get_type_by_id(person_type_id)
    testz.assert_eq_string(person_type.name, "Person")
    testz.assert_true(person_type.is_struct)
    testz.assert_false(person_type.is_primitive)
    testz.assert_eq_int(person_type.size, 12)  fr fr 8 + 4 bytes aligned
    
    fr fr Verify struct info
    sus person_struct typez.StructInfo = typez.get_struct_info("Person")
    testz.assert_eq_string(person_struct.name, "Person")
    testz.assert_eq_int(person_struct.fields.len(), 2)
    testz.assert_eq_string(person_struct.fields[0].name, "name")
    testz.assert_eq_string(person_struct.fields[1].name, "age")
    
    damn based
}

slay test_interface_type_registration() lit {
    testz.test_start("Interface type registration")
    
    fr fr Create method info for test interface
    sus draw_method typez.MethodInfo = typez.MethodInfo{
        name: "draw",
        receiver_type: typez.get_type_by_id(0),  fr fr Generic receiver
        param_types: [],
        return_type: typez.get_type_by_name("lit"),
        is_public: based
    }
    
    sus move_method typez.MethodInfo = typez.MethodInfo{
        name: "move",
        receiver_type: typez.get_type_by_id(0),
        param_types: [typez.get_type_by_name("normie"), typez.get_type_by_name("normie")],
        return_type: typez.get_type_by_name("lit"),
        is_public: based
    }
    
    sus methods typez[value].MethodInfo = [draw_method, move_method]
    
    fr fr Register interface type
    sus drawable_type_id normie = typez.register_interface_type("Drawable", methods)
    testz.assert_true(drawable_type_id > 0)
    
    fr fr Verify interface type
    sus drawable_type typez.TypeInfo = typez.get_type_by_id(drawable_type_id)
    testz.assert_eq_string(drawable_type.name, "Drawable")
    testz.assert_true(drawable_type.is_interface)
    testz.assert_eq_int(drawable_type.size, 16)  fr fr Interface fat pointer
    
    fr fr Verify interface info
    sus drawable_interface typez.InterfaceInfo = typez.get_interface_info("Drawable")
    testz.assert_eq_string(drawable_interface.name, "Drawable")
    testz.assert_eq_int(drawable_interface.methods.len(), 2)
    testz.assert_eq_string(drawable_interface.methods[0].name, "draw")
    testz.assert_eq_string(drawable_interface.methods[1].name, "move")
    
    damn based
}

slay test_array_type_registration() lit {
    testz.test_start("Array type registration")
    
    sus element_type typez.TypeInfo = typez.get_type_by_name("normie")
    sus array_type_id normie = typez.register_array_type(element_type, 10)
    
    testz.assert_true(array_type_id > 0)
    
    sus array_type typez.TypeInfo = typez.get_type_by_id(array_type_id)
    testz.assert_eq_string(array_type.name, "normie[value]")
    testz.assert_true(array_type.is_array)
    testz.assert_eq_int(array_type.size, 40)  fr fr 10 * 4 bytes
    testz.assert_eq_int(array_type.alignment, 4)
    
    damn based
}

fr fr ===== TYPE LOOKUP TESTS =====

slay test_type_lookup_operations() lit {
    testz.test_start("Type lookup operations")
    
    fr fr Test lookup by name
    sus tea_type typez.TypeInfo = typez.get_type_by_name("tea")
    testz.assert_eq_string(tea_type.name, "tea")
    testz.assert_true(tea_type.type_id > 0)
    
    fr fr Test lookup by ID
    sus same_type typez.TypeInfo = typez.get_type_by_id(tea_type.type_id)
    testz.assert_eq_string(same_type.name, tea_type.name)
    testz.assert_eq_int(same_type.size, tea_type.size)
    
    fr fr Test unknown type lookup
    sus unknown_type typez.TypeInfo = typez.get_type_by_name("nonexistent")
    testz.assert_eq_string(unknown_type.name, "unknown")
    testz.assert_eq_int(unknown_type.type_id, 0)
    
    fr fr Test getting all types
    sus all_types typez[value].TypeInfo = typez.get_all_types()
    testz.assert_true(all_types.len() >= 6)  fr fr At least the primitive types
    
    damn based
}

slay test_types_by_kind() lit {
    testz.test_start("Types by kind filtering")
    
    fr fr Get primitive types
    sus primitive_types typez[value].TypeInfo = typez.get_types_by_kind(typez.TYPE_KIND_PRIMITIVE)
    testz.assert_true(primitive_types.len() >= 6)  fr fr tea, normie, thicc, meal, lit, smol
    
    fr fr Verify all returned types are primitive
    bestie prim_type in primitive_types {
        testz.assert_true(prim_type.is_primitive)
        testz.assert_eq_int(prim_type.kind.id, typez.TYPE_KIND_PRIMITIVE)
    }
    
    fr fr Get struct types (should include any we registered)
    sus struct_types typez[value].TypeInfo = typez.get_types_by_kind(typez.TYPE_KIND_STRUCT)
    bestie struct_type in struct_types {
        testz.assert_true(struct_type.is_struct)
    }
    
    damn based
}

fr fr ===== TYPE CHECKING TESTS =====

slay test_type_assignment_compatibility() lit {
    testz.test_start("Type assignment compatibility")
    
    sus tea_type typez.TypeInfo = typez.get_type_by_name("tea")
    sus normie_type typez.TypeInfo = typez.get_type_by_name("normie")
    sus thicc_type typez.TypeInfo = typez.get_type_by_name("thicc")
    sus meal_type typez.TypeInfo = typez.get_type_by_name("meal")
    
    fr fr Same type assignment
    testz.assert_true(typez.is_assignable(tea_type, tea_type))
    testz.assert_true(typez.is_assignable(normie_type, normie_type))
    
    fr fr Numeric conversions
    testz.assert_true(typez.is_assignable(normie_type, thicc_type))
    testz.assert_true(typez.is_assignable(thicc_type, normie_type))
    testz.assert_true(typez.is_assignable(normie_type, meal_type))
    
    fr fr Non-compatible assignments
    testz.assert_false(typez.is_assignable(tea_type, normie_type))
    testz.assert_false(typez.is_assignable(normie_type, tea_type))
    
    damn based
}

slay test_numeric_convertibility() lit {
    testz.test_start("Numeric type convertibility")
    
    sus normie_type typez.TypeInfo = typez.get_type_by_name("normie")
    sus thicc_type typez.TypeInfo = typez.get_type_by_name("thicc")
    sus meal_type typez.TypeInfo = typez.get_type_by_name("meal")
    sus smol_type typez.TypeInfo = typez.get_type_by_name("smol")
    sus tea_type typez.TypeInfo = typez.get_type_by_name("tea")
    
    fr fr Numeric types should be convertible
    testz.assert_true(typez.is_numeric_convertible(normie_type, thicc_type))
    testz.assert_true(typez.is_numeric_convertible(smol_type, normie_type))
    testz.assert_true(typez.is_numeric_convertible(normie_type, meal_type))
    
    fr fr Non-numeric types should not be convertible
    testz.assert_false(typez.is_numeric_convertible(tea_type, normie_type))
    testz.assert_false(typez.is_numeric_convertible(normie_type, tea_type))
    
    damn based
}

fr fr ===== TYPE CONVERSION TESTS =====

slay test_value_conversion() lit {
    testz.test_start("Value type conversion")
    
    sus normie_type typez.TypeInfo = typez.get_type_by_name("normie")
    sus thicc_type typez.TypeInfo = typez.get_type_by_name("thicc")
    
    fr fr Test conversion between same types
    sus value normie = 42
    sus converted normie = typez.convert_value(value, normie_type, normie_type)
    testz.assert_eq_int(converted, 42)
    
    fr fr Test numeric conversion
    sus converted_thicc normie = typez.convert_value(value, normie_type, thicc_type)
    testz.assert_eq_int(converted_thicc, 42)  fr fr Should preserve value
    
    damn based
}

slay test_safe_conversion() lit {
    testz.test_start("Safe type conversion")
    
    sus normie_type typez.TypeInfo = typez.get_type_by_name("normie")
    sus thicc_type typez.TypeInfo = typez.get_type_by_name("thicc")
    sus tea_type typez.TypeInfo = typez.get_type_by_name("tea")
    
    fr fr Valid conversion
    sus (converted_value, success) = typez.safe_convert(42, normie_type, thicc_type)
    testz.assert_true(success)
    testz.assert_eq_int(converted_value, 42)
    
    fr fr Invalid conversion
    sus (invalid_value, failed) = typez.safe_convert(42, normie_type, tea_type)
    testz.assert_false(failed)
    testz.assert_eq_int(invalid_value, 0)
    
    damn based
}

fr fr ===== STRUCT CALCULATION TESTS =====

slay test_struct_size_calculation() lit {
    testz.test_start("Struct size and alignment calculation")
    
    fr fr Create fields with different alignments
    sus byte_field typez.FieldInfo = typez.FieldInfo{
        name: "byte_val",
        type_info: typez.get_type_by_name("smol"),  fr fr 1 byte, 1-byte aligned
        offset: 0,
        tag: ""
    }
    
    sus int_field typez.FieldInfo = typez.FieldInfo{
        name: "int_val", 
        type_info: typez.get_type_by_name("normie"),  fr fr 4 bytes, 4-byte aligned
        offset: 0,
        tag: ""
    }
    
    sus string_field typez.FieldInfo = typez.FieldInfo{
        name: "string_val",
        type_info: typez.get_type_by_name("tea"),  fr fr 8 bytes, 8-byte aligned
        offset: 0,
        tag: ""
    }
    
    sus fields typez[value].FieldInfo = [byte_field, int_field, string_field]
    
    fr fr Test size calculation with padding
    sus total_size normie = typez.calculate_struct_size(fields)
    testz.assert_true(total_size >= 13)  fr fr 1 + 4 + 8 = 13, but with padding
    testz.assert_eq_int(total_size % 8, 0)  fr fr Should be 8-byte aligned
    
    fr fr Test alignment calculation
    sus alignment normie = typez.calculate_struct_alignment(fields)
    testz.assert_eq_int(alignment, 8)  fr fr Largest field alignment
    
    damn based
}

slay test_alignment_functions() lit {
    testz.test_start("Alignment utility functions")
    
    fr fr Test align_to_boundary
    testz.assert_eq_int(typez.align_to_boundary(7, 8), 8)
    testz.assert_eq_int(typez.align_to_boundary(8, 8), 8)
    testz.assert_eq_int(typez.align_to_boundary(9, 8), 16)
    testz.assert_eq_int(typez.align_to_boundary(15, 4), 16)
    testz.assert_eq_int(typez.align_to_boundary(16, 4), 16)
    
    fr fr Test edge cases
    testz.assert_eq_int(typez.align_to_boundary(10, 1), 10)  fr fr No alignment needed
    testz.assert_eq_int(typez.align_to_boundary(0, 8), 0)    fr fr Zero value
    
    damn based
}

fr fr ===== REFLECTION TESTS =====

slay test_field_operations() lit {
    testz.test_start("Field reflection operations")
    
    fr fr Register a test struct first
    sus name_field typez.FieldInfo = typez.FieldInfo{
        name: "name",
        type_info: typez.get_type_by_name("tea"),
        offset: 0,
        tag: "json:\"name\""
    }
    
    sus score_field typez.FieldInfo = typez.FieldInfo{
        name: "score",
        type_info: typez.get_type_by_name("normie"),
        offset: 8,
        tag: "json:\"score\""
    }
    
    sus fields typez[value].FieldInfo = [name_field, score_field]
    typez.register_struct_type("Player", fields, [])
    
    fr fr Test field info retrieval
    sus player_type typez.TypeInfo = typez.get_type_by_name("Player")
    sus name_info typez.FieldInfo = typez.get_field_info(player_type, "name")
    testz.assert_eq_string(name_info.name, "name")
    testz.assert_eq_string(name_info.type_info.name, "tea")
    testz.assert_eq_int(name_info.offset, 0)
    testz.assert_eq_string(name_info.tag, "json:\"name\"")
    
    sus score_info typez.FieldInfo = typez.get_field_info(player_type, "score")
    testz.assert_eq_string(score_info.name, "score")
    testz.assert_eq_string(score_info.type_info.name, "normie")
    testz.assert_eq_int(score_info.offset, 8)
    
    fr fr Test unknown field
    sus unknown_field typez.FieldInfo = typez.get_field_info(player_type, "nonexistent")
    testz.assert_eq_string(unknown_field.name, "unknown")
    
    damn based
}

fr fr ===== TYPE UTILITY TESTS =====

slay test_type_utility_functions() lit {
    testz.test_start("Type utility functions")
    
    sus tea_type typez.TypeInfo = typez.get_type_by_name("tea")
    sus normie_type typez.TypeInfo = typez.get_type_by_name("normie")
    
    fr fr Test size and alignment getters
    testz.assert_eq_int(typez.get_type_size(tea_type), 8)
    testz.assert_eq_int(typez.get_type_alignment(tea_type), 8)
    testz.assert_eq_int(typez.get_type_size(normie_type), 4)
    testz.assert_eq_int(typez.get_type_alignment(normie_type), 4)
    
    fr fr Test type category checks
    testz.assert_true(typez.is_type_primitive(tea_type))
    testz.assert_false(typez.is_type_struct(tea_type))
    testz.assert_false(typez.is_type_interface(tea_type))
    testz.assert_false(typez.is_type_pointer(tea_type))
    
    damn based
}

fr fr ===== TYPE INTROSPECTION TESTS =====

slay test_type_printing() lit {
    testz.test_start("Type information printing")
    
    sus tea_type typez.TypeInfo = typez.get_type_by_name("tea")
    
    fr fr These should not crash
    typez.print_type_info(tea_type)
    typez.print_all_types()
    
    fr fr Test struct printing if we have structs registered
    sus all_types typez[value].TypeInfo = typez.get_all_types()
    bestie type_info in all_types {
        lowkey type_info.is_struct {
            sus struct_info typez.StructInfo = typez.get_struct_info(type_info.name)
            typez.print_struct_info(struct_info)
            ghosted  fr fr Only test one struct
        }
    }
    
    damn based
}

fr fr ===== ERROR HANDLING TESTS =====

slay test_invalid_type_operations() lit {
    testz.test_start("Invalid type operations")
    
    sus unknown_type typez.TypeInfo = typez.get_type_by_id(99999)  fr fr Non-existent ID
    testz.assert_eq_string(unknown_type.name, "unknown")
    testz.assert_eq_int(unknown_type.type_id, 0)
    
    sus unknown_struct typez.StructInfo = typez.get_struct_info("NonExistentStruct")
    testz.assert_eq_string(unknown_struct.name, "unknown")
    testz.assert_eq_int(unknown_struct.fields.len(), 0)
    
    sus unknown_interface typez.InterfaceInfo = typez.get_interface_info("NonExistentInterface")
    testz.assert_eq_string(unknown_interface.name, "unknown")
    testz.assert_eq_int(unknown_interface.methods.len(), 0)
    
    damn based
}

fr fr ===== INTEGRATION TESTS =====

slay test_complex_type_system() lit {
    testz.test_start("Complex type system integration")
    
    fr fr Create a complex struct with nested types
    sus id_field typez.FieldInfo = typez.FieldInfo{
        name: "id",
        type_info: typez.get_type_by_name("normie"),
        offset: 0,
        tag: "primary_key"
    }
    
    sus name_field typez.FieldInfo = typez.FieldInfo{
        name: "name", 
        type_info: typez.get_type_by_name("tea"),
        offset: 4,
        tag: "required"
    }
    
    sus active_field typez.FieldInfo = typez.FieldInfo{
        name: "active",
        type_info: typez.get_type_by_name("lit"),
        offset: 12,
        tag: "default:true"
    }
    
    sus fields typez[value].FieldInfo = [id_field, name_field, active_field]
    
    fr fr Register complex struct
    sus user_type_id normie = typez.register_struct_type("User", fields, [])
    testz.assert_true(user_type_id > 0)
    
    fr fr Test the registered type
    sus user_type typez.TypeInfo = typez.get_type_by_id(user_type_id)
    testz.assert_eq_string(user_type.name, "User")
    testz.assert_true(user_type.is_struct)
    testz.assert_true(user_type.size >= 13)  fr fr With padding
    
    fr fr Test field access
    sus user_struct typez.StructInfo = typez.get_struct_info("User")
    testz.assert_eq_int(user_struct.fields.len(), 3)
    
    fr fr Test field lookups
    sus id_info typez.FieldInfo = typez.get_field_info(user_type, "id")
    testz.assert_eq_string(id_info.name, "id")
    testz.assert_eq_string(id_info.tag, "primary_key")
    
    damn based
}

fr fr ===== RUN ALL TESTS =====

slay run_all_typez_tests() lit {
    testz.test_group_start("typez module tests")
    
    test_primitive_types()
    test_struct_type_registration()
    test_interface_type_registration()
    test_array_type_registration()
    test_type_lookup_operations()
    test_types_by_kind()
    test_type_assignment_compatibility()
    test_numeric_convertibility()
    test_value_conversion()
    test_safe_conversion()
    test_struct_size_calculation()
    test_alignment_functions()
    test_field_operations()
    test_type_utility_functions()
    test_type_printing()
    test_invalid_type_operations()
    test_complex_type_system()
    
    testz.test_group_end()
    testz.print_test_summary()
    damn based
}

fr fr Run the tests
run_all_typez_tests()
