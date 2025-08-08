#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <setjmp.h>

// CURSED Exception Handling Runtime Support
// Provides C runtime functions for LLVM-compiled CURSED error handling

// Error object structure
typedef struct CursedError {
    char* message;
    int code;
    char* details;
    struct CursedError* inner;
} CursedError;

// Exception handling context
typedef struct ExceptionContext {
    jmp_buf jump_buffer;
    CursedError* current_exception;
    struct ExceptionContext* previous;
} ExceptionContext;

// Global exception context stack
static ExceptionContext* exception_stack = NULL;

// Create a new error object
CursedError* cursed_error_create(const char* message, int code) {
    CursedError* error = (CursedError*)malloc(sizeof(CursedError));
    if (!error) return NULL;
    
    error->message = strdup(message ? message : "Unknown error");
    error->code = code;
    error->details = NULL;
    error->inner = NULL;
    
    return error;
}

// Check if a value represents an error
int cursed_error_check(void* value) {
    // In a real implementation, this would check type tags or metadata
    // For now, we use a simple heuristic based on pointer values
    if (!value) return 1; // NULL is an error
    
    // Check if the value looks like an error object
    CursedError* potential_error = (CursedError*)value;
    if (potential_error && potential_error->message) {
        return 1; // Looks like an error
    }
    
    return 0; // Not an error
}

// Propagate an error up the call stack
void cursed_error_propagate(CursedError* error) {
    if (!error) return;
    
    printf("Error propagated: %s (code: %d)\n", error->message, error->code);
    
    // In a real implementation, this would handle stack unwinding
    // For now, we just print and continue
}

// Allocate memory for exception handling
void* cursed_exception_alloc(int size) {
    return malloc(size);
}

// Begin exception handling frame
void cursed_exception_begin(void) {
    ExceptionContext* ctx = (ExceptionContext*)malloc(sizeof(ExceptionContext));
    if (!ctx) return;
    
    ctx->current_exception = NULL;
    ctx->previous = exception_stack;
    exception_stack = ctx;
    
    printf("Exception handling frame begun\n");
}

// Throw an exception
void cursed_exception_throw(void* exception) {
    if (!exception_stack) {
        printf("Unhandled exception: no exception context\n");
        exit(1);
    }
    
    exception_stack->current_exception = (CursedError*)exception;
    printf("Exception thrown: %s\n", 
           exception_stack->current_exception ? 
           exception_stack->current_exception->message : "Unknown");
    
    longjmp(exception_stack->jump_buffer, 1);
}

// Catch an exception
CursedError* cursed_exception_catch(void) {
    if (!exception_stack) return NULL;
    
    CursedError* caught = exception_stack->current_exception;
    exception_stack->current_exception = NULL;
    
    printf("Exception caught: %s\n", 
           caught ? caught->message : "No exception");
    
    return caught;
}

// Rethrow an exception
void cursed_exception_rethrow(CursedError* exception) {
    cursed_exception_throw(exception);
}

// Execute finally block
void cursed_exception_finally(void) {
    printf("Finally block executed\n");
}

// Perform stack unwinding
void cursed_stack_unwind(void) {
    if (exception_stack) {
        ExceptionContext* prev = exception_stack->previous;
        free(exception_stack);
        exception_stack = prev;
    }
    
    printf("Stack unwinding performed\n");
}

// Create a panic object
CursedError* cursed_panic_create(const char* message) {
    printf("PANIC: %s\n", message ? message : "Unknown panic");
    return cursed_error_create(message, 999); // Special panic code
}

// Recover from panic
CursedError* cursed_panic_recover(void) {
    if (exception_stack && exception_stack->current_exception) {
        CursedError* recovered = exception_stack->current_exception;
        exception_stack->current_exception = NULL;
        printf("Panic recovered: %s\n", recovered->message);
        return recovered;
    }
    
    printf("No panic to recover from\n");
    return NULL;
}

// Cleanup error object
void cursed_error_free(CursedError* error) {
    if (!error) return;
    
    if (error->message) free(error->message);
    if (error->details) free(error->details);
    if (error->inner) cursed_error_free(error->inner);
    free(error);
}

// Print error information
void cursed_error_print(CursedError* error) {
    if (!error) {
        printf("No error\n");
        return;
    }
    
    printf("Error %d: %s\n", error->code, error->message);
    if (error->details) {
        printf("  Details: %s\n", error->details);
    }
    if (error->inner) {
        printf("  Caused by: ");
        cursed_error_print(error->inner);
    }
}

// Test the runtime functions
void test_exception_runtime(void) {
    printf("=== Testing CURSED Exception Runtime ===\n");
    
    // Test error creation
    CursedError* error = cursed_error_create("Test error", 42);
    cursed_error_print(error);
    
    // Test error checking
    printf("Error check: %d\n", cursed_error_check(error));
    printf("Non-error check: %d\n", cursed_error_check("not an error"));
    
    // Test exception handling
    cursed_exception_begin();
    
    if (setjmp(exception_stack->jump_buffer) == 0) {
        printf("About to throw exception...\n");
        cursed_exception_throw(error);
        printf("This should not be reached\n");
    } else {
        printf("Exception caught in setjmp handler\n");
        CursedError* caught = cursed_exception_catch();
        cursed_error_print(caught);
    }
    
    cursed_stack_unwind();
    cursed_error_free(error);
    
    printf("=== Exception runtime test completed ===\n");
}

// Main function for standalone testing
#ifdef CURSED_RUNTIME_TEST
int main(void) {
    test_exception_runtime();
    return 0;
}
#endif
