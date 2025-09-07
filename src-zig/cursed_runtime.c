#include <stdio.h>
#include <stdint.h>

// CURSED Runtime Library - Provides built-in functions for compiled CURSED programs

// vibez.spill() string implementation
void cursed_runtime_spill_string(const char* str) {
    printf("%s\n", str);
    fflush(stdout);
}

// vibez.spill() integer implementation  
void cursed_runtime_spill_int(int64_t value) {
    printf("%ld\n", (long)value);
    fflush(stdout);
}

// vibez.spill() float implementation
void cursed_runtime_spill_float(double value) {
    printf("%f\n", value);
    fflush(stdout);
}

// vibez.spill() boolean implementation
void cursed_runtime_spill_bool(int64_t value) {
    printf("%s\n", value ? "true" : "false");
    fflush(stdout);
}
