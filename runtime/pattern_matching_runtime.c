/*
 * Pattern Matching Runtime Bridge for CURSED
 * 
 * This C library provides runtime support for pattern matching operations
 * including array pattern destructuring and struct field extraction.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include <assert.h>

// CURSED runtime value structure (simplified for pattern matching)
typedef struct CursedValue {
    int32_t type_tag;    // Type identifier
    int32_t size;        // Size for arrays/structs
    void* data;          // Actual data
    char* type_name;     // Type name for struct matching
} CursedValue;

// Type tags for pattern matching
#define TYPE_ARRAY 1
#define TYPE_STRUCT 2
#define TYPE_TUPLE 3
#define TYPE_INTEGER 4
#define TYPE_STRING 5
#define TYPE_BOOLEAN 6

// Array runtime functions for pattern matching
/**
 * Get the length of an array for pattern matching
 * @param array_ptr Pointer to CURSED array value
 * @return Length of the array
 */
int32_t get_array_length(void* array_ptr) {
    if (!array_ptr) return 0;
    
    CursedValue* value = (CursedValue*)array_ptr;
    if (value->type_tag != TYPE_ARRAY && value->type_tag != TYPE_TUPLE) {
        return 0;
    }
    
    return value->size;
}

/**
 * Get an element from an array by index for pattern matching
 * @param array_ptr Pointer to CURSED array value  
 * @param index Zero-based index of element to extract
 * @return Pointer to the element value
 */
void* get_array_element(void* array_ptr, int32_t index) {
    if (!array_ptr) return NULL;
    
    CursedValue* value = (CursedValue*)array_ptr;
    if (value->type_tag != TYPE_ARRAY && value->type_tag != TYPE_TUPLE) {
        return NULL;
    }
    
    if (index < 0 || index >= value->size) {
        return NULL;
    }
    
    // Array data is stored as array of pointers to CursedValue
    CursedValue** elements = (CursedValue**)value->data;
    return elements[index];
}

/**
 * Get the rest of an array starting from a given index (for rest patterns)
 * @param array_ptr Pointer to CURSED array value
 * @param start_index Index to start from (inclusive)
 * @return Pointer to new array containing remaining elements
 */
void* get_array_rest(void* array_ptr, int32_t start_index) {
    if (!array_ptr) return NULL;
    
    CursedValue* value = (CursedValue*)array_ptr;
    if (value->type_tag != TYPE_ARRAY && value->type_tag != TYPE_TUPLE) {
        return NULL;
    }
    
    if (start_index < 0 || start_index >= value->size) {
        // Return empty array if start_index is out of bounds
        CursedValue* empty_array = malloc(sizeof(CursedValue));
        empty_array->type_tag = TYPE_ARRAY;
        empty_array->size = 0;
        empty_array->data = NULL;
        empty_array->type_name = NULL;
        return empty_array;
    }
    
    int32_t rest_size = value->size - start_index;
    CursedValue** original_elements = (CursedValue**)value->data;
    
    // Create new array for rest elements
    CursedValue* rest_array = malloc(sizeof(CursedValue));
    rest_array->type_tag = TYPE_ARRAY;
    rest_array->size = rest_size;
    rest_array->type_name = NULL;
    
    if (rest_size > 0) {
        CursedValue** rest_elements = malloc(sizeof(CursedValue*) * rest_size);
        for (int32_t i = 0; i < rest_size; i++) {
            rest_elements[i] = original_elements[start_index + i];
        }
        rest_array->data = rest_elements;
    } else {
        rest_array->data = NULL;
    }
    
    return rest_array;
}

// Struct runtime functions for pattern matching
/**
 * Check if a value matches a specific struct type
 * @param value_ptr Pointer to value to check
 * @param type_name_ptr Pointer to expected type name string
 * @return true if types match, false otherwise
 */
bool check_struct_type(void* value_ptr, void* type_name_ptr) {
    if (!value_ptr || !type_name_ptr) return false;
    
    CursedValue* value = (CursedValue*)value_ptr;
    char* expected_type = (char*)type_name_ptr;
    
    if (value->type_tag != TYPE_STRUCT) {
        return false;
    }
    
    if (!value->type_name || !expected_type) {
        return false;
    }
    
    return strcmp(value->type_name, expected_type) == 0;
}

/**
 * Get a field value from a struct by field name
 * @param struct_ptr Pointer to CURSED struct value
 * @param field_name_ptr Pointer to field name string
 * @return Pointer to field value, or NULL if not found
 */
void* get_struct_field(void* struct_ptr, void* field_name_ptr) {
    if (!struct_ptr || !field_name_ptr) return NULL;
    
    CursedValue* value = (CursedValue*)struct_ptr;
    char* field_name = (char*)field_name_ptr;
    
    if (value->type_tag != TYPE_STRUCT) {
        return NULL;
    }
    
    // Struct data is stored as array of field name-value pairs
    // Format: [field_count, name1, value1, name2, value2, ...]
    if (!value->data) return NULL;
    
    void** struct_data = (void**)value->data;
    int32_t field_count = *(int32_t*)struct_data[0];
    
    for (int32_t i = 0; i < field_count; i++) {
        char* current_field_name = (char*)struct_data[1 + i * 2];
        void* current_field_value = struct_data[2 + i * 2];
        
        if (current_field_name && strcmp(current_field_name, field_name) == 0) {
            return current_field_value;
        }
    }
    
    return NULL;
}

// Guard expression evaluation for pattern matching
/**
 * Evaluate guard expressions in pattern matching
 * This is a placeholder - actual implementation would be generated by compiler
 * @return Result of guard expression evaluation
 */
bool evaluate_guard_expression(void) {
    // This function is typically generated by the compiler for each guard expression
    // For now, return true as a safe default
    return true;
}

// Pattern matching utility functions
/**
 * Create a new CURSED value for runtime use
 */
CursedValue* create_cursed_value(int32_t type_tag, int32_t size, void* data, const char* type_name) {
    CursedValue* value = malloc(sizeof(CursedValue));
    value->type_tag = type_tag;
    value->size = size;
    value->data = data;
    
    if (type_name) {
        value->type_name = malloc(strlen(type_name) + 1);
        strcpy(value->type_name, type_name);
    } else {
        value->type_name = NULL;
    }
    
    return value;
}

/**
 * Create an array value for pattern matching tests
 */
CursedValue* create_array_value(CursedValue** elements, int32_t count) {
    CursedValue** array_data = malloc(sizeof(CursedValue*) * count);
    for (int32_t i = 0; i < count; i++) {
        array_data[i] = elements[i];
    }
    
    return create_cursed_value(TYPE_ARRAY, count, array_data, NULL);
}

/**
 * Create a struct value for pattern matching tests
 */
CursedValue* create_struct_value(const char* type_name, const char** field_names, CursedValue** field_values, int32_t field_count) {
    // Allocate struct data: [count, name1, value1, name2, value2, ...]
    void** struct_data = malloc(sizeof(void*) * (1 + field_count * 2));
    int32_t* count_ptr = malloc(sizeof(int32_t));
    *count_ptr = field_count;
    struct_data[0] = count_ptr;
    
    for (int32_t i = 0; i < field_count; i++) {
        // Allocate and copy field name
        char* field_name_copy = malloc(strlen(field_names[i]) + 1);
        strcpy(field_name_copy, field_names[i]);
        
        struct_data[1 + i * 2] = field_name_copy;
        struct_data[2 + i * 2] = field_values[i];
    }
    
    return create_cursed_value(TYPE_STRUCT, field_count, struct_data, type_name);
}

/**
 * Create an integer value for pattern matching tests
 */
CursedValue* create_integer_value(int32_t value) {
    int32_t* data = malloc(sizeof(int32_t));
    *data = value;
    return create_cursed_value(TYPE_INTEGER, sizeof(int32_t), data, NULL);
}

/**
 * Create a string value for pattern matching tests
 */
CursedValue* create_string_value(const char* value) {
    char* data = malloc(strlen(value) + 1);
    strcpy(data, value);
    return create_cursed_value(TYPE_STRING, strlen(value), data, NULL);
}

/**
 * Free a CURSED value and its data
 */
void free_cursed_value(CursedValue* value) {
    if (!value) return;
    
    if (value->type_name) {
        free(value->type_name);
    }
    
    if (value->data) {
        if (value->type_tag == TYPE_STRUCT) {
            void** struct_data = (void**)value->data;
            int32_t field_count = *(int32_t*)struct_data[0];
            
            // Free field names and count
            free(struct_data[0]);
            for (int32_t i = 0; i < field_count; i++) {
                free(struct_data[1 + i * 2]); // Free field name copy
            }
        }
        free(value->data);
    }
    
    free(value);
}
