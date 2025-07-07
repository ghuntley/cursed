/*
 * Minimal C Bootstrap for CURSED Self-Hosting
 * This provides the minimal 12 functions needed for a completely self-hosted compiler
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// External function declarations for CURSED runtime
extern void cursed_main(int argc, char** argv);
extern void cursed_runtime_init(void);
extern void cursed_runtime_shutdown(void);

// Minimal C functions required for bootstrap
void* cursed_malloc(size_t size) {
    return malloc(size);
}

void cursed_free(void* ptr) {
    free(ptr);
}

int cursed_printf(const char* format, ...) {
    va_list args;
    va_start(args, format);
    int result = vprintf(format, args);
    va_end(args);
    return result;
}

int cursed_puts(const char* str) {
    return puts(str);
}

FILE* cursed_fopen(const char* filename, const char* mode) {
    return fopen(filename, mode);
}

int cursed_fclose(FILE* file) {
    return fclose(file);
}

size_t cursed_fread(void* ptr, size_t size, size_t count, FILE* file) {
    return fread(ptr, size, count, file);
}

size_t cursed_fwrite(const void* ptr, size_t size, size_t count, FILE* file) {
    return fwrite(ptr, size, count, file);
}

long cursed_ftell(FILE* file) {
    return ftell(file);
}

int cursed_fseek(FILE* file, long offset, int whence) {
    return fseek(file, offset, whence);
}

void cursed_exit(int status) {
    exit(status);
}

int cursed_system(const char* command) {
    return system(command);
}

// Main bootstrap entry point
int main(int argc, char** argv) {
    printf("🚀 CURSED Self-Hosting Bootstrap\n");
    
    // Initialize the CURSED runtime
    cursed_runtime_init();
    
    // Execute the main CURSED program
    cursed_main(argc, argv);
    
    // Cleanup and shutdown
    cursed_runtime_shutdown();
    
    printf("✅ CURSED bootstrap complete\n");
    return 0;
}
