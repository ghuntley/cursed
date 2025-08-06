#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

// CURSED Defer Runtime System
// Implements LIFO (Last In, First Out) defer execution

#define MAX_DEFER_STACK_SIZE 1000

typedef void (*cleanup_func_t)(void);

typedef struct {
    cleanup_func_t functions[MAX_DEFER_STACK_SIZE];
    size_t count;
} defer_stack_t;

// Global defer stack for runtime
static defer_stack_t global_defer_stack = {0};

// Thread-local defer stacks would go here for full concurrency support
// For now, we use a global stack

// Push a cleanup function onto the defer stack
void cursed_defer_push(void* cleanup_func) {
    if (global_defer_stack.count >= MAX_DEFER_STACK_SIZE) {
        fprintf(stderr, "Error: Defer stack overflow\n");
        exit(1);
    }
    
    global_defer_stack.functions[global_defer_stack.count] = (cleanup_func_t)cleanup_func;
    global_defer_stack.count++;
    
    #ifdef DEBUG_DEFER
    printf("Defer pushed: function at %p, stack size: %zu\n", cleanup_func, global_defer_stack.count);
    #endif
}

// Pop the most recent defer function (used for early cleanup)
void cursed_defer_pop(void) {
    if (global_defer_stack.count == 0) {
        return;  // Nothing to pop
    }
    
    global_defer_stack.count--;
    
    #ifdef DEBUG_DEFER
    printf("Defer popped: stack size: %zu\n", global_defer_stack.count);
    #endif
}

// Execute all defer functions in LIFO order
void cursed_defer_execute_all(void) {
    #ifdef DEBUG_DEFER
    printf("Executing %zu defer functions\n", global_defer_stack.count);
    #endif
    
    // Execute in reverse order (LIFO)
    while (global_defer_stack.count > 0) {
        global_defer_stack.count--;
        cleanup_func_t func = global_defer_stack.functions[global_defer_stack.count];
        
        if (func != NULL) {
            #ifdef DEBUG_DEFER
            printf("Executing defer function at %p\n", (void*)func);
            #endif
            func();
        }
    }
}

// Execute defer functions up to a specific count (for scoped cleanup)
void cursed_defer_execute_to_count(size_t target_count) {
    #ifdef DEBUG_DEFER
    printf("Executing defer functions from %zu to %zu\n", global_defer_stack.count, target_count);
    #endif
    
    while (global_defer_stack.count > target_count) {
        global_defer_stack.count--;
        cleanup_func_t func = global_defer_stack.functions[global_defer_stack.count];
        
        if (func != NULL) {
            #ifdef DEBUG_DEFER
            printf("Executing scoped defer function at %p\n", (void*)func);
            #endif
            func();
        }
    }
}

// Get current defer stack size (for scope management)
size_t cursed_defer_get_stack_size(void) {
    return global_defer_stack.count;
}

// Clear all defer functions (emergency cleanup)
void cursed_defer_clear_all(void) {
    global_defer_stack.count = 0;
    
    #ifdef DEBUG_DEFER
    printf("Defer stack cleared\n");
    #endif
}

// Initialize defer runtime (called at program start)
void cursed_defer_init(void) {
    global_defer_stack.count = 0;
    
    #ifdef DEBUG_DEFER
    printf("Defer runtime initialized\n");
    #endif
}

// Cleanup defer runtime (called at program exit)
void cursed_defer_cleanup(void) {
    // Execute any remaining defers
    cursed_defer_execute_all();
    
    #ifdef DEBUG_DEFER
    printf("Defer runtime cleanup complete\n");
    #endif
}
