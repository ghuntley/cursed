/*
 * Test for pattern matching runtime functions
 * This C test verifies the runtime functions work correctly
 */

#include "runtime/pattern_matching_runtime.h"
#include <stdio.h>
#include <assert.h>

int main() {
    printf("Testing pattern matching runtime functions...\n");
    
    // Test array functions
    printf("Testing array pattern matching functions...\n");
    
    // Create test array: [1, 2, 3, 4, 5]
    CursedValue* elements[5];
    elements[0] = create_integer_value(1);
    elements[1] = create_integer_value(2);
    elements[2] = create_integer_value(3);
    elements[3] = create_integer_value(4);
    elements[4] = create_integer_value(5);
    
    CursedValue* test_array = create_array_value(elements, 5);
    
    // Test get_array_length
    int32_t length = get_array_length(test_array);
    assert(length == 5);
    printf("✅ get_array_length: %d\n", length);
    
    // Test get_array_element
    CursedValue* first_element = (CursedValue*)get_array_element(test_array, 0);
    assert(first_element != NULL);
    assert(*(int32_t*)first_element->data == 1);
    printf("✅ get_array_element(0): %d\n", *(int32_t*)first_element->data);
    
    CursedValue* third_element = (CursedValue*)get_array_element(test_array, 2);
    assert(third_element != NULL);
    assert(*(int32_t*)third_element->data == 3);
    printf("✅ get_array_element(2): %d\n", *(int32_t*)third_element->data);
    
    // Test get_array_rest
    CursedValue* rest_array = (CursedValue*)get_array_rest(test_array, 2);
    assert(rest_array != NULL);
    assert(get_array_length(rest_array) == 3);
    printf("✅ get_array_rest(2): length = %d\n", get_array_length(rest_array));
    
    // Test struct functions
    printf("Testing struct pattern matching functions...\n");
    
    // Create test struct: { name: "John", age: 30 }
    const char* field_names[] = {"name", "age"};
    CursedValue* field_values[2];
    field_values[0] = create_string_value("John");
    field_values[1] = create_integer_value(30);
    
    CursedValue* test_struct = create_struct_value("Person", field_names, field_values, 2);
    
    // Test check_struct_type
    bool type_matches = check_struct_type(test_struct, (void*)"Person");
    assert(type_matches == true);
    printf("✅ check_struct_type: %s\n", type_matches ? "true" : "false");
    
    bool wrong_type = check_struct_type(test_struct, (void*)"Animal");
    assert(wrong_type == false);
    printf("✅ check_struct_type (wrong): %s\n", wrong_type ? "true" : "false");
    
    // Test get_struct_field
    CursedValue* name_field = (CursedValue*)get_struct_field(test_struct, (void*)"name");
    assert(name_field != NULL);
    printf("✅ get_struct_field(name): %s\n", (char*)name_field->data);
    
    CursedValue* age_field = (CursedValue*)get_struct_field(test_struct, (void*)"age");
    assert(age_field != NULL);
    assert(*(int32_t*)age_field->data == 30);
    printf("✅ get_struct_field(age): %d\n", *(int32_t*)age_field->data);
    
    // Test nonexistent field
    CursedValue* missing_field = (CursedValue*)get_struct_field(test_struct, (void*)"email");
    assert(missing_field == NULL);
    printf("✅ get_struct_field(missing): NULL\n");
    
    // Test evaluate_guard_expression
    bool guard_result = evaluate_guard_expression();
    printf("✅ evaluate_guard_expression: %s\n", guard_result ? "true" : "false");
    
    // Cleanup
    free_cursed_value(test_array);
    free_cursed_value(rest_array);
    free_cursed_value(test_struct);
    for (int i = 0; i < 5; i++) {
        free_cursed_value(elements[i]);
    }
    for (int i = 0; i < 2; i++) {
        free_cursed_value(field_values[i]);
    }
    
    printf("\n🎉 All pattern matching runtime tests passed!\n");
    return 0;
}
