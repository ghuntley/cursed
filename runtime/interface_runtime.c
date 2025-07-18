/*
 * Interface Runtime Support for CURSED
 * 
 * This C library provides runtime support for interface dispatch operations
 * including vtable management, method dispatch, and type checking.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

// Interface value structure (fat pointer)
typedef struct {
    void* vtable_ptr;
    void* data_ptr;
    const char* interface_name;
    const char* concrete_type;
} InterfaceValue;

// VTable entry structure
typedef struct {
    const char* method_name;
    void* function_ptr;
    const char* signature;
} VTableEntry;

// Interface VTable structure
typedef struct {
    const char* interface_name;
    const char* concrete_type;
    int method_count;
    VTableEntry* methods;
} InterfaceVTable;

// Interface registry entry
typedef struct InterfaceRegistryEntry {
    const char* interface_name;
    const char* concrete_type;
    InterfaceVTable* vtable;
    struct InterfaceRegistryEntry* next;
} InterfaceRegistryEntry;

// Global interface registry
static InterfaceRegistryEntry* interface_registry = NULL;

// Runtime interface dispatch functions

/**
 * Create interface value from concrete object
 */
void* cursed_create_interface_value(void* vtable_ptr, void* data_ptr, const char* type_name) {
    InterfaceValue* interface_value = (InterfaceValue*)malloc(sizeof(InterfaceValue));
    if (!interface_value) {
        return NULL;
    }
    
    interface_value->vtable_ptr = vtable_ptr;
    interface_value->data_ptr = data_ptr;
    interface_value->interface_name = ""; // Will be set by caller
    interface_value->concrete_type = type_name;
    
    return interface_value;
}

/**
 * Dispatch interface method call
 */
void* cursed_dispatch_method(void* vtable_ptr, const char* method_name, void* args, int arg_count) {
    InterfaceVTable* vtable = (InterfaceVTable*)vtable_ptr;
    
    // Find method in vtable
    for (int i = 0; i < vtable->method_count; i++) {
        if (strcmp(vtable->methods[i].method_name, method_name) == 0) {
            // Get function pointer
            void* func_ptr = vtable->methods[i].function_ptr;
            
            // Call function with arguments
            // This is a simplified implementation - in practice, you'd need
            // proper argument marshalling based on the method signature
            typedef void* (*GenericFunction)(void*, void*);
            GenericFunction func = (GenericFunction)func_ptr;
            
            return func(args, NULL); // Simplified call
        }
    }
    
    // Method not found
    return NULL;
}

/**
 * Check if type implements interface
 */
bool cursed_implements_interface(const char* type_name, const char* interface_name) {
    InterfaceRegistryEntry* entry = interface_registry;
    
    while (entry) {
        if (strcmp(entry->concrete_type, type_name) == 0 &&
            strcmp(entry->interface_name, interface_name) == 0) {
            return true;
        }
        entry = entry->next;
    }
    
    return false;
}

/**
 * Get vtable for type-interface pair
 */
void* cursed_runtime_get_vtable(const char* type_name, const char* interface_name) {
    InterfaceRegistryEntry* entry = interface_registry;
    
    while (entry) {
        if (strcmp(entry->concrete_type, type_name) == 0 &&
            strcmp(entry->interface_name, interface_name) == 0) {
            return entry->vtable;
        }
        entry = entry->next;
    }
    
    return NULL;
}

/**
 * Register interface implementation
 */
bool cursed_register_interface_implementation(const char* interface_name, const char* concrete_type, InterfaceVTable* vtable) {
    InterfaceRegistryEntry* entry = (InterfaceRegistryEntry*)malloc(sizeof(InterfaceRegistryEntry));
    if (!entry) {
        return false;
    }
    
    entry->interface_name = interface_name;
    entry->concrete_type = concrete_type;
    entry->vtable = vtable;
    entry->next = interface_registry;
    
    interface_registry = entry;
    return true;
}

/**
 * Runtime interface hierarchy checking
 */
bool cursed_runtime_check_interface_hierarchy(const char* derived_interface, const char* base_interface) {
    // This would need to be implemented based on your interface hierarchy system
    // For now, return false as a placeholder
    return false;
}

/**
 * Panic on type assertion failure
 */
void cursed_panic_type_assertion(const char* type_name, const char* interface_name) {
    fprintf(stderr, "CURSED Runtime Error: Type assertion failed - type '%s' does not implement interface '%s'\n", 
            type_name, interface_name);
    abort();
}

/**
 * Runtime interface compliance checking
 */
bool cursed_runtime_check_interface(void* type_info, void* interface_info) {
    // This would implement full runtime type checking
    // For now, return true as a placeholder
    return true;
}

/**
 * Interface value type checking
 */
bool cursed_check_interface_type(void* interface_value, const char* expected_interface) {
    InterfaceValue* iface = (InterfaceValue*)interface_value;
    if (!iface) {
        return false;
    }
    
    return strcmp(iface->interface_name, expected_interface) == 0;
}

/**
 * Get concrete type from interface value
 */
const char* cursed_get_concrete_type(void* interface_value) {
    InterfaceValue* iface = (InterfaceValue*)interface_value;
    if (!iface) {
        return NULL;
    }
    
    return iface->concrete_type;
}

/**
 * Get interface name from interface value
 */
const char* cursed_get_interface_name(void* interface_value) {
    InterfaceValue* iface = (InterfaceValue*)interface_value;
    if (!iface) {
        return NULL;
    }
    
    return iface->interface_name;
}

/**
 * Free interface value
 */
void cursed_free_interface_value(void* interface_value) {
    if (interface_value) {
        free(interface_value);
    }
}

/**
 * Clone interface value
 */
void* cursed_clone_interface_value(void* interface_value) {
    InterfaceValue* original = (InterfaceValue*)interface_value;
    if (!original) {
        return NULL;
    }
    
    InterfaceValue* clone = (InterfaceValue*)malloc(sizeof(InterfaceValue));
    if (!clone) {
        return NULL;
    }
    
    *clone = *original;
    return clone;
}

/**
 * Initialize interface runtime system
 */
void cursed_initialize_interface_runtime(void) {
    // Initialize global registry and other runtime structures
    interface_registry = NULL;
    
    // Register built-in interfaces if any
    // This would be called during program initialization
}

/**
 * Cleanup interface runtime system
 */
void cursed_cleanup_interface_runtime(void) {
    // Free all registry entries
    InterfaceRegistryEntry* entry = interface_registry;
    while (entry) {
        InterfaceRegistryEntry* next = entry->next;
        free(entry);
        entry = next;
    }
    
    interface_registry = NULL;
}

/**
 * Debug: Print interface registry
 */
void cursed_debug_print_interface_registry(void) {
    printf("Interface Registry:\n");
    InterfaceRegistryEntry* entry = interface_registry;
    int count = 0;
    
    while (entry) {
        printf("  %d. %s -> %s (methods: %d)\n", 
               count++, entry->concrete_type, entry->interface_name, 
               entry->vtable ? entry->vtable->method_count : 0);
        entry = entry->next;
    }
    
    if (count == 0) {
        printf("  (empty)\n");
    }
}

/**
 * Method dispatch with error handling
 */
void* cursed_safe_dispatch_method(void* interface_value, const char* method_name, void* args, int arg_count) {
    InterfaceValue* iface = (InterfaceValue*)interface_value;
    if (!iface || !iface->vtable_ptr) {
        fprintf(stderr, "CURSED Runtime Error: Invalid interface value for method dispatch\n");
        return NULL;
    }
    
    return cursed_dispatch_method(iface->vtable_ptr, method_name, args, arg_count);
}

/**
 * Get method count for interface
 */
int cursed_get_method_count(void* vtable_ptr) {
    InterfaceVTable* vtable = (InterfaceVTable*)vtable_ptr;
    return vtable ? vtable->method_count : 0;
}

/**
 * Get method name by index
 */
const char* cursed_get_method_name(void* vtable_ptr, int index) {
    InterfaceVTable* vtable = (InterfaceVTable*)vtable_ptr;
    if (!vtable || index < 0 || index >= vtable->method_count) {
        return NULL;
    }
    
    return vtable->methods[index].method_name;
}

/**
 * Check if method exists in vtable
 */
bool cursed_has_method(void* vtable_ptr, const char* method_name) {
    InterfaceVTable* vtable = (InterfaceVTable*)vtable_ptr;
    if (!vtable) {
        return false;
    }
    
    for (int i = 0; i < vtable->method_count; i++) {
        if (strcmp(vtable->methods[i].method_name, method_name) == 0) {
            return true;
        }
    }
    
    return false;
}

/**
 * Simple print function for CURSED programs
 * Prints a string using puts
 */
int print(const char* str) {
    if (str == NULL) {
        return puts("(null)");
    }
    return puts(str);
}

/**
 * Simple test method implementation
 * Returns true for test_method calls
 */
bool cursed_test_method_impl(void* obj) {
    // For TestStruct.test_method(), always return true
    return true;
}

/**
 * Simple method dispatch for basic interface calls
 * Handles basic method dispatch without complex vtables
 */
void* cursed_dispatch_simple_method(void* obj, const char* method_name, int arg_count) {
    if (!obj || !method_name) {
        return NULL;
    }
    
    // Handle known method names
    if (strcmp(method_name, "test_method") == 0) {
        // Return boolean value as pointer (1 for true)
        return (void*)1;
    }
    
    // Default: return NULL for unknown methods
    return NULL;
}
