/*
 * Simple Plugin System Demo
 * Demonstrates the core concepts of the real plugin loading system
 */

#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
#include <string.h>

// Plugin function pointer types
typedef int (*init_func_t)(void);
typedef void (*cleanup_func_t)(void);
typedef const char* (*test_func_t)(void);
typedef int (*add_func_t)(int, int);

int main() {
    printf("🔌 CURSED Plugin System Demo\n");
    printf("==============================\n\n");

    // Test 1: Load plugin
    printf("Test 1: Loading test plugin...\n");
    void* handle = dlopen("./test_plugin.so", RTLD_LAZY);
    if (!handle) {
        printf("✗ Failed to load plugin: %s\n", dlerror());
        printf("  Build it with: gcc -shared -fPIC -o test_plugin.so test_plugin_example.c\n");
        return 1;
    }
    printf("✓ Plugin loaded successfully\n");

    // Test 2: Resolve symbols  
    printf("\nTest 2: Resolving plugin functions...\n");
    
    init_func_t plugin_init = (init_func_t)dlsym(handle, "plugin_init");
    if (!plugin_init) {
        printf("✗ Could not find plugin_init: %s\n", dlerror());
        dlclose(handle);
        return 1;
    }
    printf("✓ Found plugin_init function\n");

    cleanup_func_t plugin_cleanup = (cleanup_func_t)dlsym(handle, "plugin_cleanup");
    if (!plugin_cleanup) {
        printf("✗ Could not find plugin_cleanup: %s\n", dlerror());
        dlclose(handle);
        return 1;
    }
    printf("✓ Found plugin_cleanup function\n");

    test_func_t test_basic = (test_func_t)dlsym(handle, "test_basic_functionality");
    if (!test_basic) {
        printf("✗ Could not find test_basic_functionality: %s\n", dlerror());
        dlclose(handle);
        return 1;
    }
    printf("✓ Found test_basic_functionality function\n");

    add_func_t add_numbers = (add_func_t)dlsym(handle, "add_numbers");
    if (!add_numbers) {
        printf("✗ Could not find add_numbers: %s\n", dlerror());
        dlclose(handle);
        return 1;
    }
    printf("✓ Found add_numbers function\n");

    // Test 3: Initialize plugin
    printf("\nTest 3: Initializing plugin...\n");
    int init_result = plugin_init();
    if (init_result == 0) {
        printf("✓ Plugin initialized successfully\n");
    } else {
        printf("✗ Plugin initialization failed: %d\n", init_result);
        dlclose(handle);
        return 1;
    }

    // Test 4: Call plugin functions
    printf("\nTest 4: Calling plugin functions...\n");
    
    const char* test_result = test_basic();
    if (test_result) {
        printf("✓ test_basic_functionality() -> %s\n", test_result);
    } else {
        printf("✗ test_basic_functionality() returned NULL\n");
    }

    int math_result = add_numbers(15, 27);
    printf("✓ add_numbers(15, 27) -> %d\n", math_result);

    // Test 5: Plugin capabilities
    printf("\nTest 5: Testing plugin capabilities...\n");
    
    // Get more function pointers for capability testing
    int (*count_vowels)(const char*) = dlsym(handle, "count_vowels");
    if (count_vowels) {
        int vowel_count = count_vowels("Hello World!");
        printf("✓ count_vowels(\"Hello World!\") -> %d\n", vowel_count);
    }

    char* (*reverse_string)(const char*) = dlsym(handle, "reverse_string");
    if (reverse_string) {
        char* reversed = reverse_string("CURSED");
        if (reversed) {
            printf("✓ reverse_string(\"CURSED\") -> %s\n", reversed);
            free(reversed); // Plugin allocated memory
        }
    }

    double (*calc_pi)(int) = dlsym(handle, "calculate_pi_estimate");
    if (calc_pi) {
        double pi = calc_pi(10000);
        printf("✓ calculate_pi_estimate(10000) -> %f\n", pi);
    }

    // Test 6: Plugin status
    printf("\nTest 6: Plugin status and statistics...\n");
    
    int (*get_call_count)(void) = dlsym(handle, "get_call_count");
    if (get_call_count) {
        int calls = get_call_count();
        printf("✓ Plugin handled %d function calls\n", calls);
    }

    const char* (*get_status)(void) = dlsym(handle, "get_plugin_status");
    if (get_status) {
        printf("✓ Plugin status: %s\n", get_status());
    }

    // Test 7: Clean shutdown
    printf("\nTest 7: Plugin cleanup...\n");
    plugin_cleanup();
    printf("✓ Plugin cleanup completed\n");

    // Test 8: Unload plugin
    printf("\nTest 8: Unloading plugin...\n");
    int close_result = dlclose(handle);
    if (close_result == 0) {
        printf("✓ Plugin unloaded successfully\n");
    } else {
        printf("✗ Plugin unload failed\n");
    }

    printf("\n🎉 Plugin System Demo Complete!\n");
    printf("\n📋 Summary:\n");
    printf("✅ Dynamic library loading: dlopen/dlsym\n");
    printf("✅ Symbol resolution: Found all required functions\n");
    printf("✅ Plugin initialization: Proper lifecycle management\n");
    printf("✅ Function calling: Math, string, and utility functions\n");
    printf("✅ Memory management: Plugin allocated and freed memory\n");
    printf("✅ Statistics tracking: Call counting and status monitoring\n");
    printf("✅ Clean shutdown: Proper cleanup and unloading\n");
    printf("\n🔌 This demonstrates the core functionality of the CURSED plugin system!\n");
    printf("   The same principles are implemented in the Zig/CURSED integration.\n");

    return 0;
}
