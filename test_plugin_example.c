/*
 * Test Plugin Implementation
 * Demonstrates real plugin functionality for the CURSED plugin system
 * Compile as a shared library: gcc -shared -fPIC -o test_plugin.so test_plugin_example.c
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Plugin API version
#define PLUGIN_API_VERSION 1

// Plugin capability flags
#define CAPABILITY_MATH     (1 << 0)
#define CAPABILITY_STRING   (1 << 1)
#define CAPABILITY_IO       (1 << 2)

// Plugin metadata structure
typedef struct {
    const char* name;
    const char* version;
    const char* author;
    const char* description;
    unsigned int api_version;
    unsigned int capabilities;
} PluginMetadata;

// Global plugin metadata
static PluginMetadata plugin_metadata = {
    .name = "test_plugin",
    .version = "1.0.0",
    .author = "CURSED Plugin Developer",
    .description = "Test plugin demonstrating math and string operations",
    .api_version = PLUGIN_API_VERSION,
    .capabilities = CAPABILITY_MATH | CAPABILITY_STRING | CAPABILITY_IO
};

// Plugin state
static int plugin_initialized = 0;
static int call_count = 0;

// Required plugin initialization function
int plugin_init(void) {
    if (plugin_initialized) {
        return -1; // Already initialized
    }
    
    printf("Test plugin initializing...\n");
    plugin_initialized = 1;
    call_count = 0;
    
    return 0; // Success
}

// Optional plugin cleanup function
void plugin_cleanup(void) {
    if (!plugin_initialized) {
        return;
    }
    
    printf("Test plugin cleaning up... (handled %d calls)\n", call_count);
    plugin_initialized = 0;
}

// Get plugin metadata
const PluginMetadata* plugin_get_metadata(void) {
    return &plugin_metadata;
}

// Math capability functions
int add_numbers(int a, int b) {
    call_count++;
    return a + b;
}

int multiply_numbers(int a, int b) {
    call_count++;
    return a * b;
}

double calculate_pi_estimate(int iterations) {
    call_count++;
    double pi = 0.0;
    int sign = 1;
    
    for (int i = 0; i < iterations; i++) {
        pi += sign * (4.0 / (2 * i + 1));
        sign *= -1;
    }
    
    return pi;
}

// String capability functions
char* reverse_string(const char* input) {
    call_count++;
    
    if (!input) return NULL;
    
    int len = strlen(input);
    char* result = malloc(len + 1);
    if (!result) return NULL;
    
    for (int i = 0; i < len; i++) {
        result[i] = input[len - 1 - i];
    }
    result[len] = '\0';
    
    return result;
}

char* uppercase_string(const char* input) {
    call_count++;
    
    if (!input) return NULL;
    
    int len = strlen(input);
    char* result = malloc(len + 1);
    if (!result) return NULL;
    
    for (int i = 0; i <= len; i++) {
        if (input[i] >= 'a' && input[i] <= 'z') {
            result[i] = input[i] - 32;
        } else {
            result[i] = input[i];
        }
    }
    
    return result;
}

int count_vowels(const char* input) {
    call_count++;
    
    if (!input) return 0;
    
    int count = 0;
    for (const char* p = input; *p; p++) {
        switch (*p) {
            case 'a': case 'A':
            case 'e': case 'E':
            case 'i': case 'I':
            case 'o': case 'O':
            case 'u': case 'U':
                count++;
                break;
        }
    }
    
    return count;
}

// IO capability functions
int write_to_file(const char* filename, const char* content) {
    call_count++;
    
    if (!filename || !content) return -1;
    
    FILE* file = fopen(filename, "w");
    if (!file) return -1;
    
    int result = fprintf(file, "%s", content);
    fclose(file);
    
    return result >= 0 ? 0 : -1;
}

char* read_from_file(const char* filename) {
    call_count++;
    
    if (!filename) return NULL;
    
    FILE* file = fopen(filename, "r");
    if (!file) return NULL;
    
    // Get file size
    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);
    
    if (size < 0) {
        fclose(file);
        return NULL;
    }
    
    // Allocate buffer
    char* buffer = malloc(size + 1);
    if (!buffer) {
        fclose(file);
        return NULL;
    }
    
    // Read file content
    size_t read_size = fread(buffer, 1, size, file);
    buffer[read_size] = '\0';
    
    fclose(file);
    return buffer;
}

// Generic extension handler for extension points
const char* extension_handler(const char* data) {
    call_count++;
    
    static char result_buffer[1024];
    
    if (!data) {
        strncpy(result_buffer, "no_data_provided", sizeof(result_buffer) - 1);
    } else {
        snprintf(result_buffer, sizeof(result_buffer), "processed_%s", data);
    }
    result_buffer[sizeof(result_buffer) - 1] = '\0';
    
    return result_buffer;
}

// Plugin status and statistics
int get_call_count(void) {
    return call_count;
}

int is_plugin_ready(void) {
    return plugin_initialized;
}

const char* get_plugin_status(void) {
    if (plugin_initialized) {
        return "ready";
    } else {
        return "uninitialized";
    }
}

// Test function for basic functionality verification
const char* test_basic_functionality(void) {
    call_count++;
    
    // Test math
    int math_result = add_numbers(5, 3);
    
    // Test string
    char* reversed = reverse_string("hello");
    
    static char test_result[256];
    snprintf(test_result, sizeof(test_result), 
             "math_test_5+3=%d,string_test_reverse_hello=%s", 
             math_result, reversed ? reversed : "NULL");
    
    if (reversed) {
        free(reversed);
    }
    
    return test_result;
}

// Dynamic capability checking
int has_capability(unsigned int capability) {
    return (plugin_metadata.capabilities & capability) != 0;
}

// Version compatibility checking
int is_compatible_with_api(unsigned int api_version) {
    return plugin_metadata.api_version == api_version;
}

// Memory usage estimation
unsigned int estimate_memory_usage(void) {
    return sizeof(plugin_metadata) + (call_count * 16); // Rough estimate
}

// Free memory allocated by plugin functions
void plugin_free(void* ptr) {
    if (ptr) {
        free(ptr);
    }
}

// Bulk operation example
int* generate_fibonacci_sequence(int count) {
    call_count++;
    
    if (count <= 0) return NULL;
    
    int* sequence = malloc(count * sizeof(int));
    if (!sequence) return NULL;
    
    if (count >= 1) sequence[0] = 0;
    if (count >= 2) sequence[1] = 1;
    
    for (int i = 2; i < count; i++) {
        sequence[i] = sequence[i-1] + sequence[i-2];
    }
    
    return sequence;
}

// Plugin information for debugging
void print_plugin_info(void) {
    printf("Plugin: %s v%s\n", plugin_metadata.name, plugin_metadata.version);
    printf("Author: %s\n", plugin_metadata.author);
    printf("Description: %s\n", plugin_metadata.description);
    printf("API Version: %u\n", plugin_metadata.api_version);
    printf("Capabilities: %u\n", plugin_metadata.capabilities);
    printf("Status: %s\n", get_plugin_status());
    printf("Call Count: %d\n", call_count);
}
