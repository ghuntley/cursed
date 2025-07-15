/*
 * Type Assertion Runtime Support for CURSED
 * 
 * This C library provides runtime support for type assertion operations
 * including type checking, casting, and panic handling.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include <assert.h>

// Type IDs for runtime type checking
typedef enum {
    CURSED_TYPE_UNKNOWN = 0,
    CURSED_TYPE_INTEGER = 1,
    CURSED_TYPE_FLOAT = 2,
    CURSED_TYPE_STRING = 3,
    CURSED_TYPE_BOOLEAN = 4,
    CURSED_TYPE_BYTE = 5,
    CURSED_TYPE_CHARACTER = 6,
    CURSED_TYPE_ARRAY = 100,
    CURSED_TYPE_SLICE = 101,
    CURSED_TYPE_REFERENCE = 200,
    CURSED_TYPE_FUNCTION = 300,
    CURSED_TYPE_INTERFACE = 400,
    CURSED_TYPE_GENERIC = 500,
    CURSED_TYPE_MAP = 600,
    CURSED_TYPE_CHANNEL = 700,
} CursedTypeId;

// Runtime type information structure
typedef struct {
    CursedTypeId type_id;
    const char* type_name;
    size_t size;
    void* metadata;
} CursedTypeInfo;

// Value with type information
typedef struct {
    void* data;
    CursedTypeInfo* type_info;
} CursedTypedValue;

// Type names for error messages
static const char* type_names[] = {
    [CURSED_TYPE_UNKNOWN] = "unknown",
    [CURSED_TYPE_INTEGER] = "normie",
    [CURSED_TYPE_FLOAT] = "drip",
    [CURSED_TYPE_STRING] = "tea",
    [CURSED_TYPE_BOOLEAN] = "lit",
    [CURSED_TYPE_BYTE] = "byte",
    [CURSED_TYPE_CHARACTER] = "sip",
    [CURSED_TYPE_ARRAY] = "array",
    [CURSED_TYPE_SLICE] = "slice",
    [CURSED_TYPE_REFERENCE] = "reference",
    [CURSED_TYPE_FUNCTION] = "function",
    [CURSED_TYPE_INTERFACE] = "interface",
    [CURSED_TYPE_GENERIC] = "generic",
    [CURSED_TYPE_MAP] = "map",
    [CURSED_TYPE_CHANNEL] = "channel",
};

// Global type registry
static CursedTypeInfo type_registry[1000];
static int registry_size = 0;

// Forward declarations
static const char* get_type_name(CursedTypeId type_id);
static bool is_compatible_type(CursedTypeId source, CursedTypeId target);
static void* cast_value(void* value, CursedTypeId source, CursedTypeId target);
static void* get_default_value(CursedTypeId type_id);

/**
 * Check if a type conversion is valid at runtime
 */
bool cursed_check_type_compatibility(void* value, int32_t source_type_id, int32_t target_type_id) {
    CursedTypeId source = (CursedTypeId)source_type_id;
    CursedTypeId target = (CursedTypeId)target_type_id;
    
    // Same type is always compatible
    if (source == target) {
        return true;
    }
    
    return is_compatible_type(source, target);
}

/**
 * Check interface type compatibility
 */
bool cursed_check_interface_type(void* value) {
    // For now, simplified - would check vtable compatibility
    return value != NULL;
}

/**
 * Check generic type compatibility
 */
bool cursed_check_generic_type(void* value) {
    // For now, simplified - would check type parameters
    return value != NULL;
}

/**
 * Check array type compatibility
 */
bool cursed_check_array_type(void* value) {
    // For now, simplified - would check element types and dimensions
    return value != NULL;
}

/**
 * Check function type compatibility
 */
bool cursed_check_function_type(void* value) {
    // For now, simplified - would check signature compatibility
    return value != NULL;
}

/**
 * Cast a value from one type to another
 */
void* cursed_cast_type(void* value, int32_t source_type_id, int32_t target_type_id) {
    CursedTypeId source = (CursedTypeId)source_type_id;
    CursedTypeId target = (CursedTypeId)target_type_id;
    
    // Same type - no cast needed
    if (source == target) {
        return value;
    }
    
    return cast_value(value, source, target);
}

/**
 * Get an empty string for default values
 */
char* cursed_empty_string() {
    static char empty[] = "";
    return empty;
}

/**
 * Get a null value for default values
 */
void* cursed_null_value() {
    return NULL;
}

/**
 * Panic with type assertion error
 */
void cursed_panic_type_assertion(int32_t source_type_id, int32_t target_type_id) {
    CursedTypeId source = (CursedTypeId)source_type_id;
    CursedTypeId target = (CursedTypeId)target_type_id;
    
    const char* source_name = get_type_name(source);
    const char* target_name = get_type_name(target);
    
    fprintf(stderr, "CURSED PANIC: Type assertion failed - cannot convert %s to %s\n", 
            source_name, target_name);
    fprintf(stderr, "This is a type assertion panic in CURSED runtime\n");
    
    // Print stack trace if available
    // (This would integrate with the panic runtime for full stack traces)
    
    abort();
}

/**
 * Register a type in the global registry
 */
void cursed_register_type(int32_t type_id, const char* type_name, size_t size, void* metadata) {
    if (registry_size >= 1000) {
        fprintf(stderr, "Type registry full\n");
        return;
    }
    
    CursedTypeInfo* info = &type_registry[registry_size++];
    info->type_id = (CursedTypeId)type_id;
    info->type_name = type_name;
    info->size = size;
    info->metadata = metadata;
}

/**
 * Get type information from registry
 */
CursedTypeInfo* cursed_get_type_info(int32_t type_id) {
    for (int i = 0; i < registry_size; i++) {
        if (type_registry[i].type_id == (CursedTypeId)type_id) {
            return &type_registry[i];
        }
    }
    return NULL;
}

// Helper functions

static const char* get_type_name(CursedTypeId type_id) {
    if (type_id < sizeof(type_names) / sizeof(type_names[0])) {
        return type_names[type_id];
    }
    return "unknown";
}

static bool is_compatible_type(CursedTypeId source, CursedTypeId target) {
    // Define type compatibility rules
    switch (source) {
        case CURSED_TYPE_INTEGER:
            return target == CURSED_TYPE_FLOAT || 
                   target == CURSED_TYPE_BOOLEAN || 
                   target == CURSED_TYPE_BYTE;
        
        case CURSED_TYPE_FLOAT:
            return target == CURSED_TYPE_INTEGER;
        
        case CURSED_TYPE_BOOLEAN:
            return target == CURSED_TYPE_INTEGER;
        
        case CURSED_TYPE_BYTE:
            return target == CURSED_TYPE_INTEGER || 
                   target == CURSED_TYPE_CHARACTER;
        
        case CURSED_TYPE_CHARACTER:
            return target == CURSED_TYPE_BYTE || 
                   target == CURSED_TYPE_INTEGER;
        
        case CURSED_TYPE_STRING:
            return target == CURSED_TYPE_ARRAY; // String to []byte
        
        case CURSED_TYPE_ARRAY:
            return target == CURSED_TYPE_SLICE;
        
        case CURSED_TYPE_SLICE:
            return target == CURSED_TYPE_ARRAY;
        
        case CURSED_TYPE_REFERENCE:
            return true; // References can be cast to most types
        
        case CURSED_TYPE_INTERFACE:
            return true; // Interface can be cast to concrete types
        
        case CURSED_TYPE_GENERIC:
            return true; // Generic types can be cast based on constraints
        
        default:
            return false;
    }
}

static void* cast_value(void* value, CursedTypeId source, CursedTypeId target) {
    // Type casting implementations
    switch (source) {
        case CURSED_TYPE_INTEGER:
            if (target == CURSED_TYPE_FLOAT) {
                int32_t* int_val = (int32_t*)value;
                double* float_val = malloc(sizeof(double));
                *float_val = (double)(*int_val);
                return float_val;
            }
            if (target == CURSED_TYPE_BOOLEAN) {
                int32_t* int_val = (int32_t*)value;
                bool* bool_val = malloc(sizeof(bool));
                *bool_val = (*int_val != 0);
                return bool_val;
            }
            if (target == CURSED_TYPE_BYTE) {
                int32_t* int_val = (int32_t*)value;
                uint8_t* byte_val = malloc(sizeof(uint8_t));
                *byte_val = (uint8_t)(*int_val);
                return byte_val;
            }
            break;
        
        case CURSED_TYPE_FLOAT:
            if (target == CURSED_TYPE_INTEGER) {
                double* float_val = (double*)value;
                int32_t* int_val = malloc(sizeof(int32_t));
                *int_val = (int32_t)(*float_val);
                return int_val;
            }
            break;
        
        case CURSED_TYPE_BOOLEAN:
            if (target == CURSED_TYPE_INTEGER) {
                bool* bool_val = (bool*)value;
                int32_t* int_val = malloc(sizeof(int32_t));
                *int_val = *bool_val ? 1 : 0;
                return int_val;
            }
            break;
        
        case CURSED_TYPE_BYTE:
            if (target == CURSED_TYPE_INTEGER) {
                uint8_t* byte_val = (uint8_t*)value;
                int32_t* int_val = malloc(sizeof(int32_t));
                *int_val = (int32_t)(*byte_val);
                return int_val;
            }
            break;
        
        default:
            break;
    }
    
    // Default: return original value (no conversion)
    return value;
}

static void* get_default_value(CursedTypeId type_id) {
    switch (type_id) {
        case CURSED_TYPE_INTEGER: {
            int32_t* val = malloc(sizeof(int32_t));
            *val = 0;
            return val;
        }
        case CURSED_TYPE_FLOAT: {
            double* val = malloc(sizeof(double));
            *val = 0.0;
            return val;
        }
        case CURSED_TYPE_BOOLEAN: {
            bool* val = malloc(sizeof(bool));
            *val = false;
            return val;
        }
        case CURSED_TYPE_BYTE: {
            uint8_t* val = malloc(sizeof(uint8_t));
            *val = 0;
            return val;
        }
        case CURSED_TYPE_STRING: {
            return cursed_empty_string();
        }
        default:
            return NULL;
    }
}

/**
 * Initialize the type assertion runtime
 */
void cursed_init_type_assertion_runtime() {
    // Initialize built-in types
    cursed_register_type(CURSED_TYPE_INTEGER, "normie", sizeof(int32_t), NULL);
    cursed_register_type(CURSED_TYPE_FLOAT, "drip", sizeof(double), NULL);
    cursed_register_type(CURSED_TYPE_STRING, "tea", sizeof(char*), NULL);
    cursed_register_type(CURSED_TYPE_BOOLEAN, "lit", sizeof(bool), NULL);
    cursed_register_type(CURSED_TYPE_BYTE, "byte", sizeof(uint8_t), NULL);
    cursed_register_type(CURSED_TYPE_CHARACTER, "sip", sizeof(char), NULL);
}

/**
 * Cleanup the type assertion runtime
 */
void cursed_cleanup_type_assertion_runtime() {
    // Clean up any allocated resources
    registry_size = 0;
}
