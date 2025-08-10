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

// Defer stack for cleanup during unwinding
typedef struct DeferAction {
    void (*cleanup_fn)(void* context);
    void* context;
    char* description;
    int scope_level;
    struct DeferAction* next;
} DeferAction;

static DeferAction* defer_stack = NULL;
static int current_scope_level = 0;

// Goroutine panic context
typedef struct GoroutinePanicContext {
    int goroutine_id;
    CursedError* panic_error;
    char* stack_trace;
    struct GoroutinePanicContext* next;
} GoroutinePanicContext;

static GoroutinePanicContext* goroutine_panics = NULL;

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

// Push a defer action onto the stack
void cursed_defer_push(void (*cleanup_fn)(void*), void* context, const char* description) {
    DeferAction* action = (DeferAction*)malloc(sizeof(DeferAction));
    if (!action) return;
    
    action->cleanup_fn = cleanup_fn;
    action->context = context;
    action->description = strdup(description ? description : "defer cleanup");
    action->scope_level = current_scope_level;
    action->next = defer_stack;
    defer_stack = action;
    
    printf("Defer action pushed: %s (scope %d)\n", action->description, action->scope_level);
}

// Execute defer actions up to a specific scope level
void cursed_defer_cleanup_scope(int target_scope) {
    printf("Cleaning up defer actions to scope %d\n", target_scope);
    
    while (defer_stack && defer_stack->scope_level >= target_scope) {
        DeferAction* action = defer_stack;
        defer_stack = action->next;
        
        printf("Executing defer: %s\n", action->description);
        if (action->cleanup_fn) {
            action->cleanup_fn(action->context);
        }
        
        free(action->description);
        free(action);
    }
}

// Unwind stack to a specific scope level
void cursed_unwind_to_scope(int target_scope) {
    printf("Unwinding to scope level %d\n", target_scope);
    
    // Execute all defer actions from current scope down to target scope
    cursed_defer_cleanup_scope(target_scope);
    
    // Update current scope level
    current_scope_level = target_scope;
}

// Enter a new scope
void cursed_enter_scope(void) {
    current_scope_level++;
    printf("Entered scope level %d\n", current_scope_level);
}

// Exit current scope
void cursed_exit_scope(void) {
    if (current_scope_level > 0) {
        cursed_defer_cleanup_scope(current_scope_level);
        current_scope_level--;
        printf("Exited to scope level %d\n", current_scope_level);
    }
}

// Propagate panic through goroutine call stack
void cursed_goroutine_panic_propagate(CursedError* panic_error) {
    if (!panic_error) return;
    
    // Get current goroutine ID (simplified - would use actual goroutine context)
    int current_goroutine = 1; // Placeholder
    
    printf("Panic propagating in goroutine %d: %s\n", current_goroutine, panic_error->message);
    
    // Create goroutine panic context
    GoroutinePanicContext* panic_ctx = (GoroutinePanicContext*)malloc(sizeof(GoroutinePanicContext));
    if (panic_ctx) {
        panic_ctx->goroutine_id = current_goroutine;
        panic_ctx->panic_error = panic_error;
        panic_ctx->stack_trace = strdup("goroutine stack trace"); // Placeholder
        panic_ctx->next = goroutine_panics;
        goroutine_panics = panic_ctx;
    }
    
    // Unwind the entire goroutine stack
    cursed_unwind_to_scope(0);
    
    // If we reach here, the panic wasn't recovered, so terminate goroutine
    printf("Goroutine %d terminated due to unrecovered panic\n", current_goroutine);
}

// Enhanced panic recovery with goroutine support
CursedError* cursed_panic_recover_goroutine(int goroutine_id) {
    GoroutinePanicContext* prev = NULL;
    GoroutinePanicContext* current = goroutine_panics;
    
    while (current) {
        if (current->goroutine_id == goroutine_id) {
            // Remove from panic list
            if (prev) {
                prev->next = current->next;
            } else {
                goroutine_panics = current->next;
            }
            
            CursedError* recovered = current->panic_error;
            printf("Recovered panic in goroutine %d: %s\n", goroutine_id, recovered->message);
            
            free(current->stack_trace);
            free(current);
            
            return recovered;
        }
        prev = current;
        current = current->next;
    }
    
    printf("No panic to recover in goroutine %d\n", goroutine_id);
    return NULL;
}

// Check if a goroutine has a pending panic
int cursed_goroutine_has_panic(int goroutine_id) {
    GoroutinePanicContext* current = goroutine_panics;
    while (current) {
        if (current->goroutine_id == goroutine_id) {
            return 1;
        }
        current = current->next;
    }
    return 0;
}

// Test cleanup function for defer testing
void test_cleanup(void* context) {
    char* name = (char*)context;
    printf("Cleanup executed: %s\n", name ? name : "unknown");
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
    
    // Test defer and scope management
    printf("\n=== Testing Defer and Scope Management ===\n");
    cursed_enter_scope(); // Scope 1
    cursed_defer_push(test_cleanup, "resource1", "cleanup resource 1");
    
    cursed_enter_scope(); // Scope 2
    cursed_defer_push(test_cleanup, "resource2", "cleanup resource 2");
    cursed_defer_push(test_cleanup, "resource3", "cleanup resource 3");
    
    // Test unwinding to scope 1 (should cleanup resources 2 and 3)
    cursed_unwind_to_scope(1);
    
    // Test full scope exit (should cleanup resource 1)
    cursed_exit_scope();
    
    // Test goroutine panic propagation
    printf("\n=== Testing Goroutine Panic Propagation ===\n");
    CursedError* panic_error = cursed_panic_create("Goroutine panic test");
    cursed_goroutine_panic_propagate(panic_error);
    
    // Test panic recovery
    CursedError* recovered = cursed_panic_recover_goroutine(1);
    if (recovered) {
        printf("Successfully recovered panic: %s\n", recovered->message);
        cursed_error_free(recovered);
    }
    
    // Test exception handling
    printf("\n=== Testing Exception Handling ===\n");
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
