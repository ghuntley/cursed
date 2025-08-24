/**
 * CURSED WebAssembly Runtime
 * Provides minimal runtime support for CURSED programs compiled to WebAssembly
 */

#include <stdint.h>
#include <stdlib.h>
#include <string.h>

// WebAssembly memory management
extern unsigned char __heap_base;
static uint32_t heap_top = (uint32_t)&__heap_base;

/**
 * WebAssembly memory allocation
 */
void* __wasm_malloc(uint32_t size) {
    void* ptr = (void*)heap_top;
    heap_top += size;
    // Align to 8 bytes
    heap_top = (heap_top + 7) & ~7;
    return ptr;
}

/**
 * WebAssembly memory deallocation (no-op for simplicity)
 */
void __wasm_free(void* ptr) {
    // Simple heap implementation - no actual freeing
    (void)ptr;
}

/**
 * String output for WebAssembly (export to host)
 */
extern void wasm_print_string(const char* str);

/**
 * Print function for CURSED vibez.spill()
 */
void cursed_print(const char* str) {
    wasm_print_string(str);
}

/**
 * Print integer
 */
void cursed_print_int(int32_t value) {
    // Simple integer to string conversion
    char buffer[16];
    int32_t temp = value;
    int pos = 0;
    
    if (temp == 0) {
        buffer[pos++] = '0';
    } else {
        if (temp < 0) {
            buffer[pos++] = '-';
            temp = -temp;
        }
        
        // Convert to string (reverse order)
        char digits[16];
        int digit_count = 0;
        while (temp > 0) {
            digits[digit_count++] = '0' + (temp % 10);
            temp /= 10;
        }
        
        // Reverse digits into buffer
        for (int i = digit_count - 1; i >= 0; i--) {
            buffer[pos++] = digits[i];
        }
    }
    
    buffer[pos] = '\0';
    cursed_print(buffer);
}

/**
 * Print float
 */
void cursed_print_float(float value) {
    // Simplified float printing
    if (value == 0.0f) {
        cursed_print("0.0");
        return;
    }
    
    char buffer[32];
    // Simple float conversion (basic implementation)
    int32_t integer_part = (int32_t)value;
    cursed_print_int(integer_part);
    cursed_print(".");
    
    float fractional = value - integer_part;
    if (fractional < 0) fractional = -fractional;
    
    // Print first few decimal places
    for (int i = 0; i < 3; i++) {
        fractional *= 10;
        int digit = (int)fractional;
        buffer[0] = '0' + digit;
        buffer[1] = '\0';
        cursed_print(buffer);
        fractional -= digit;
    }
}

/**
 * WebAssembly exports
 */
__attribute__((export_name("malloc")))
void* exported_malloc(uint32_t size) {
    return __wasm_malloc(size);
}

__attribute__((export_name("free")))
void exported_free(void* ptr) {
    __wasm_free(ptr);
}

__attribute__((export_name("cursed_print")))
void exported_cursed_print(const char* str) {
    cursed_print(str);
}

__attribute__((export_name("cursed_print_int")))
void exported_cursed_print_int(int32_t value) {
    cursed_print_int(value);
}

__attribute__((export_name("cursed_print_float")))
void exported_cursed_print_float(float value) {
    cursed_print_float(value);
}
