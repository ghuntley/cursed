/*
 * Pattern Matching Runtime Bridge for CURSED
 * Header file for pattern matching runtime functions
 */

#ifndef PATTERN_MATCHING_RUNTIME_H
#define PATTERN_MATCHING_RUNTIME_H

#include <stdint.h>
#include <stdbool.h>

// CURSED runtime value structure
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

// Array pattern matching functions
int32_t get_array_length(void* array_ptr);
void* get_array_element(void* array_ptr, int32_t index);
void* get_array_rest(void* array_ptr, int32_t start_index);

// Struct pattern matching functions
bool check_struct_type(void* value_ptr, void* type_name_ptr);
void* get_struct_field(void* struct_ptr, void* field_name_ptr);

// Guard expression evaluation
bool evaluate_guard_expression(void);

// Utility functions for creating test values
CursedValue* create_cursed_value(int32_t type_tag, int32_t size, void* data, const char* type_name);
CursedValue* create_array_value(CursedValue** elements, int32_t count);
CursedValue* create_struct_value(const char* type_name, const char** field_names, CursedValue** field_values, int32_t field_count);
CursedValue* create_integer_value(int32_t value);
CursedValue* create_string_value(const char* value);
void free_cursed_value(CursedValue* value);

#endif // PATTERN_MATCHING_RUNTIME_H
