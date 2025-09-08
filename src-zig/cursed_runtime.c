#include <stdio.h>
#include <stdint.h>
#include <math.h>

// CURSED Runtime Library - Provides built-in functions for compiled CURSED programs

// vibez.spill() string implementation - match interpreter spacing  
void cursed_runtime_spill_string(const char* str) {
    printf("%s", str);
    fflush(stdout);
}

// vibez.spill() integer implementation - NO automatic newlines to match interpreter
void cursed_runtime_spill_int(int64_t value) {
    printf("%ld", (long)value);
    fflush(stdout);
}

// vibez.spill() float implementation - match interpreter precision exactly
void cursed_runtime_spill_float(double value) {
    // Match interpreter precision exactly
    if (value == (long)value) {
        // If it's a whole number, print as integer
        printf("%ld", (long)value);
    } else {
        // For scientific notation, use .5g to match interpreter format
        // But for very small numbers, use .5e format to match "1.00000e-6"
        if (fabs(value) < 1e-4 || fabs(value) > 1e6) {
            printf("%.5e", value);
        } else {
            printf("%.5g", value);
        }
    }
    fflush(stdout);
}

// vibez.spill() boolean implementation - NO automatic newlines to match interpreter
void cursed_runtime_spill_bool(int64_t value) {
    // Print CURSED boolean keywords
    printf("%s", value ? "based" : "cringe");
    fflush(stdout);
}
