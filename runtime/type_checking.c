/*
 * Runtime type checking functions for CURSED type switches
 * 
 * Provides runtime type information and type checking capabilities
 * for dynamic type dispatch and type switches.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

// C11 alignment support
#if __STDC_VERSION__ >= 201112L
#include <stdalign.h>
#else
#define alignof _Alignof
#endif

// Runtime type information structure
typedef struct runtime_type_info {
    uint32_t type_id;
    const char* type_name;
    uint64_t size;
    uint64_t alignment;
} runtime_type_info_t;

// Type tag enumeration for CURSED values
typedef enum cursed_type_tag {
    CURSED_TYPE_NORMIE = 1,
    CURSED_TYPE_SMOL = 2,
    CURSED_TYPE_MID = 3,
    CURSED_TYPE_THICC = 4,
    CURSED_TYPE_DRIP = 5,
    CURSED_TYPE_SNACK = 6,
    CURSED_TYPE_MEAL = 7,
    CURSED_TYPE_TEA = 8,
    CURSED_TYPE_LIT = 9,
    CURSED_TYPE_SIP = 10,
    CURSED_TYPE_BYTE = 11,
    CURSED_TYPE_RUNE = 12,
    CURSED_TYPE_EXTRA = 13,
    CURSED_TYPE_ARRAY = 100,
    CURSED_TYPE_SLICE = 101,
    CURSED_TYPE_POINTER = 102,
    CURSED_TYPE_INTERFACE = 103,
    CURSED_TYPE_STRUCT = 104,
    CURSED_TYPE_FUNCTION = 105,
    CURSED_TYPE_GENERIC = 106,
    CURSED_TYPE_CHANNEL = 107,
    CURSED_TYPE_MAP = 108,
    CURSED_TYPE_UNKNOWN = 999
} cursed_type_tag_t;

// CURSED value structure with type tag
typedef struct cursed_value {
    cursed_type_tag_t type_tag;
    void* data;
    size_t size;
} cursed_value_t;

// Static type information table
static runtime_type_info_t type_info_table[] = {
    {CURSED_TYPE_NORMIE, "normie", sizeof(int32_t), alignof(int32_t)},
    {CURSED_TYPE_SMOL, "smol", sizeof(int8_t), alignof(int8_t)},
    {CURSED_TYPE_MID, "mid", sizeof(int16_t), alignof(int16_t)},
    {CURSED_TYPE_THICC, "thicc", sizeof(int64_t), alignof(int64_t)},
    {CURSED_TYPE_DRIP, "drip", sizeof(float), alignof(float)},
    {CURSED_TYPE_SNACK, "snack", sizeof(float), alignof(float)},
    {CURSED_TYPE_MEAL, "meal", sizeof(double), alignof(double)},
    {CURSED_TYPE_TEA, "tea", sizeof(char*), alignof(char*)},
    {CURSED_TYPE_LIT, "lit", sizeof(uint8_t), alignof(uint8_t)},
    {CURSED_TYPE_SIP, "sip", sizeof(char), alignof(char)},
    {CURSED_TYPE_BYTE, "byte", sizeof(uint8_t), alignof(uint8_t)},
    {CURSED_TYPE_RUNE, "rune", sizeof(int32_t), alignof(int32_t)},
    {CURSED_TYPE_EXTRA, "extra", sizeof(double) * 2, alignof(double)},
    {0, NULL, 0, 0} // Sentinel
};

// Get runtime type information for a value
runtime_type_info_t* cursed_get_runtime_type_info(void* value) {
    if (!value) {
        return NULL;
    }
    
    // Cast to CURSED value to extract type tag
    cursed_value_t* cursed_val = (cursed_value_t*)value;
    cursed_type_tag_t type_tag = cursed_val->type_tag;
    
    // Look up type information in table
    for (int i = 0; type_info_table[i].type_name != NULL; i++) {
        if (type_info_table[i].type_id == type_tag) {
            return &type_info_table[i];
        }
    }
    
    // Return unknown type
    static runtime_type_info_t unknown_type = {
        CURSED_TYPE_UNKNOWN, "unknown", 0, 0
    };
    return &unknown_type;
}

// Check if value is of integer type
uint8_t cursed_is_integer_type(void* value) {
    if (!value) return 0;
    
    cursed_value_t* cursed_val = (cursed_value_t*)value;
    cursed_type_tag_t type_tag = cursed_val->type_tag;
    
    return (type_tag == CURSED_TYPE_NORMIE || 
            type_tag == CURSED_TYPE_SMOL ||
            type_tag == CURSED_TYPE_MID ||
            type_tag == CURSED_TYPE_THICC ||
            type_tag == CURSED_TYPE_BYTE ||
            type_tag == CURSED_TYPE_RUNE) ? 1 : 0;
}

// Check if value is of float type
uint8_t cursed_is_float_type(void* value) {
    if (!value) return 0;
    
    cursed_value_t* cursed_val = (cursed_value_t*)value;
    cursed_type_tag_t type_tag = cursed_val->type_tag;
    
    return (type_tag == CURSED_TYPE_DRIP ||
            type_tag == CURSED_TYPE_SNACK ||
            type_tag == CURSED_TYPE_MEAL ||
            type_tag == CURSED_TYPE_EXTRA) ? 1 : 0;
}

// Check if value is of string type
uint8_t cursed_is_string_type(void* value) {
    if (!value) return 0;
    
    cursed_value_t* cursed_val = (cursed_value_t*)value;
    return (cursed_val->type_tag == CURSED_TYPE_TEA) ? 1 : 0;
}

// Check if value is of boolean type
uint8_t cursed_is_boolean_type(void* value) {
    if (!value) return 0;
    
    cursed_value_t* cursed_val = (cursed_value_t*)value;
    return (cursed_val->type_tag == CURSED_TYPE_LIT) ? 1 : 0;
}

// Check if value implements an interface
uint8_t cursed_implements_interface(void* type_info, const char* interface_name) {
    if (!type_info || !interface_name) {
        return 0;
    }
    
    runtime_type_info_t* info = (runtime_type_info_t*)type_info;
    
    // For now, simple string comparison
    // In a full implementation, we'd have a proper interface registry
    if (strcmp(interface_name, "Stringer") == 0) {
        // TEA type implements Stringer interface
        return (info->type_id == CURSED_TYPE_TEA) ? 1 : 0;
    }
    
    if (strcmp(interface_name, "Numeric") == 0) {
        // Integer and float types implement Numeric interface
        return (info->type_id == CURSED_TYPE_NORMIE ||
                info->type_id == CURSED_TYPE_SMOL ||
                info->type_id == CURSED_TYPE_MID ||
                info->type_id == CURSED_TYPE_THICC ||
                info->type_id == CURSED_TYPE_DRIP ||
                info->type_id == CURSED_TYPE_SNACK ||
                info->type_id == CURSED_TYPE_MEAL) ? 1 : 0;
    }
    
    // Default: interface not implemented
    return 0;
}

// Panic function for unhandled type switch cases
void cursed_panic(const char* message) {
    fprintf(stderr, "CURSED Runtime Panic: %s\n", message);
    exit(1);
}

// Create a CURSED value with type tag
cursed_value_t* cursed_create_value(cursed_type_tag_t type_tag, void* data, size_t size) {
    cursed_value_t* value = malloc(sizeof(cursed_value_t));
    if (!value) {
        cursed_panic("Failed to allocate memory for CURSED value");
    }
    
    value->type_tag = type_tag;
    value->size = size;
    
    // Copy data
    value->data = malloc(size);
    if (!value->data) {
        free(value);
        cursed_panic("Failed to allocate memory for CURSED value data");
    }
    memcpy(value->data, data, size);
    
    return value;
}

// Free a CURSED value
void cursed_free_value(cursed_value_t* value) {
    if (value) {
        if (value->data) {
            free(value->data);
        }
        free(value);
    }
}

// Get type name from type tag
const char* cursed_get_type_name(cursed_type_tag_t type_tag) {
    for (int i = 0; type_info_table[i].type_name != NULL; i++) {
        if (type_info_table[i].type_id == type_tag) {
            return type_info_table[i].type_name;
        }
    }
    return "unknown";
}

// Type switch runtime helper functions
uint8_t cursed_type_switch_check_type(void* value, uint32_t expected_type_id) {
    if (!value) return 0;
    
    cursed_value_t* cursed_val = (cursed_value_t*)value;
    return (cursed_val->type_tag == expected_type_id) ? 1 : 0;
}

// Declare runtime function for LLVM linkage
uint8_t cursed_type_switch_check_type(void* value, uint32_t expected_type_id);

// Cast value to specific type (with runtime checking)
void* cursed_type_cast(void* value, uint32_t target_type_id) {
    if (!value) return NULL;
    
    cursed_value_t* cursed_val = (cursed_value_t*)value;
    
    // Check if cast is valid
    if (cursed_val->type_tag != target_type_id) {
        char error_msg[256];
        snprintf(error_msg, sizeof(error_msg), 
                "Invalid type cast from %s to %s", 
                cursed_get_type_name(cursed_val->type_tag),
                cursed_get_type_name(target_type_id));
        cursed_panic(error_msg);
    }
    
    return cursed_val->data;
}
