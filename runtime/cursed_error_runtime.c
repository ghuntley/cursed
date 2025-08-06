#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include <setjmp.h>
#include <unistd.h>

// CURSED Error Propagation Runtime
// Provides complete error handling with yikes/shook/fam support

typedef struct CursedError {
    char* message;
    int32_t code;
    char* source_location;
    struct CursedError* inner_error;
    uint64_t error_id;
} CursedError;

typedef struct CursedDeferContext {
    void (**cleanup_funcs)(void*);
    void** cleanup_args;
    size_t count;
    size_t capacity;
} CursedDeferContext;

// Global error context per thread
static __thread CursedError* current_error = NULL;
static __thread CursedDeferContext* current_defer_context = NULL;
static uint64_t error_counter = 0;

// Forward declarations
void cursed_defer_execute_all(CursedDeferContext* ctx);

// Error creation and management
CursedError* cursed_create_error(const char* message, int32_t code, const char* source_location) {
    CursedError* error = malloc(sizeof(CursedError));
    if (!error) {
        fprintf(stderr, "CURSED PANIC: Out of memory creating error\n");
        abort();
    }
    
    error->message = strdup(message ? message : "Unknown error");
    error->code = code;
    error->source_location = strdup(source_location ? source_location : "unknown:0:0");
    error->inner_error = NULL;
    error->error_id = __atomic_fetch_add(&error_counter, 1, __ATOMIC_SEQ_CST);
    
    return error;
}

void cursed_free_error(CursedError* error) {
    if (!error) return;
    
    free(error->message);
    free(error->source_location);
    if (error->inner_error) {
        cursed_free_error(error->inner_error);
    }
    free(error);
}

bool cursed_is_error(void* value) {
    // Simple error detection: NULL pointer indicates error
    // In full implementation, would check error union types
    return value == NULL || (uintptr_t)value == 0xDEADBEEF; // Error sentinel
}

// Error propagation (shook operator)
void* cursed_propagate_error(void* result, CursedError* error) {
    if (cursed_is_error(result) && error) {
        current_error = error;
        return result; // Propagate error up the stack
    }
    return result;
}

// Defer system for cleanup
CursedDeferContext* cursed_defer_init(void) {
    CursedDeferContext* ctx = malloc(sizeof(CursedDeferContext));
    if (!ctx) {
        fprintf(stderr, "CURSED PANIC: Out of memory initializing defer context\n");
        abort();
    }
    
    ctx->capacity = 16;
    ctx->cleanup_funcs = malloc(sizeof(void*) * ctx->capacity);
    ctx->cleanup_args = malloc(sizeof(void*) * ctx->capacity);
    ctx->count = 0;
    
    return ctx;
}

void cursed_defer_push(CursedDeferContext* ctx, void (*cleanup_func)(void*), void* arg) {
    if (!ctx) return;
    
    if (ctx->count >= ctx->capacity) {
        ctx->capacity *= 2;
        ctx->cleanup_funcs = realloc(ctx->cleanup_funcs, sizeof(void*) * ctx->capacity);
        ctx->cleanup_args = realloc(ctx->cleanup_args, sizeof(void*) * ctx->capacity);
    }
    
    ctx->cleanup_funcs[ctx->count] = cleanup_func;
    ctx->cleanup_args[ctx->count] = arg;
    ctx->count++;
}

void cursed_defer_execute_all(CursedDeferContext* ctx) {
    if (!ctx) return;
    
    // Execute cleanup functions in LIFO order
    for (ssize_t i = ctx->count - 1; i >= 0; i--) {
        if (ctx->cleanup_funcs[i]) {
            ctx->cleanup_funcs[i](ctx->cleanup_args[i]);
        }
    }
    
    // Free defer context
    free(ctx->cleanup_funcs);
    free(ctx->cleanup_args);
    free(ctx);
}

// Panic and recovery system (fam blocks)
typedef struct CursedPanicContext {
    jmp_buf* recovery_point;
    CursedError* panic_error;
    CursedDeferContext* defer_context;
} CursedPanicContext;

static __thread CursedPanicContext* panic_stack = NULL;
static __thread size_t panic_stack_size = 0;
static __thread size_t panic_stack_capacity = 0;

void cursed_push_panic_context(CursedPanicContext* ctx) {
    if (panic_stack_size >= panic_stack_capacity) {
        panic_stack_capacity = panic_stack_capacity ? panic_stack_capacity * 2 : 8;
        panic_stack = realloc(panic_stack, sizeof(CursedPanicContext) * panic_stack_capacity);
    }
    
    panic_stack[panic_stack_size++] = *ctx;
}

CursedPanicContext* cursed_pop_panic_context(void) {
    if (panic_stack_size == 0) return NULL;
    return &panic_stack[--panic_stack_size];
}

void cursed_panic_with_error(CursedError* error) {
    // Execute defer cleanup before panicking
    if (current_defer_context) {
        cursed_defer_execute_all(current_defer_context);
        current_defer_context = NULL;
    }
    
    // Look for panic recovery context
    CursedPanicContext* recovery = cursed_pop_panic_context();
    if (recovery && recovery->recovery_point) {
        recovery->panic_error = error;
        longjmp(*recovery->recovery_point, 1);
    }
    
    // No recovery context, terminate with error
    fprintf(stderr, "CURSED PANIC: %s (code: %d) at %s\n", 
            error->message, error->code, error->source_location);
    cursed_free_error(error);
    abort();
}

// Error context management
void cursed_set_error_context(CursedError* error) {
    if (current_error) {
        cursed_free_error(current_error);
    }
    current_error = error;
}

CursedError* cursed_get_error_context(void) {
    return current_error;
}

void cursed_clear_error_context(void) {
    if (current_error) {
        cursed_free_error(current_error);
        current_error = NULL;
    }
}

// Error wrapping for context preservation
CursedError* cursed_wrap_error(CursedError* inner, const char* context_message) {
    if (!inner) return NULL;
    
    CursedError* wrapper = cursed_create_error(context_message, inner->code, inner->source_location);
    wrapper->inner_error = inner;
    return wrapper;
}

// Error chain traversal
void cursed_print_error_chain(CursedError* error) {
    if (!error) return;
    
    CursedError* current = error;
    int depth = 0;
    
    while (current) {
        for (int i = 0; i < depth; i++) printf("  ");
        printf("Error #%lu: %s (code: %d) at %s\n", 
               current->error_id, current->message, current->code, current->source_location);
        current = current->inner_error;
        depth++;
    }
}

// Runtime initialization and cleanup
void cursed_runtime_init(void) {
    current_error = NULL;
    current_defer_context = NULL;
    panic_stack = NULL;
    panic_stack_size = 0;
    panic_stack_capacity = 0;
}

void cursed_runtime_cleanup(void) {
    cursed_clear_error_context();
    
    if (current_defer_context) {
        cursed_defer_execute_all(current_defer_context);
        current_defer_context = NULL;
    }
    
    if (panic_stack) {
        free(panic_stack);
        panic_stack = NULL;
    }
}

// Error statistics and monitoring
typedef struct ErrorStats {
    uint64_t total_errors;
    uint64_t total_panics;
    uint64_t total_recoveries;
    uint64_t defer_cleanups;
} ErrorStats;

static ErrorStats global_stats = {0};

ErrorStats cursed_get_error_stats(void) {
    return global_stats;
}

void cursed_reset_error_stats(void) {
    memset(&global_stats, 0, sizeof(ErrorStats));
}

// Function result helpers for yikes/shook integration
typedef struct CursedResult {
    void* value;
    CursedError* error;
} CursedResult;

CursedResult cursed_ok(void* value) {
    return (CursedResult){.value = value, .error = NULL};
}

CursedResult cursed_error(CursedError* error) {
    return (CursedResult){.value = NULL, .error = error};
}

bool cursed_result_is_error(CursedResult result) {
    return result.error != NULL;
}

void* cursed_result_unwrap(CursedResult result) {
    if (result.error) {
        cursed_panic_with_error(result.error);
    }
    return result.value;
}

void* cursed_result_unwrap_or(CursedResult result, void* default_value) {
    if (result.error) {
        cursed_free_error(result.error);
        return default_value;
    }
    return result.value;
}
