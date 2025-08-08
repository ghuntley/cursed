#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

/// Complete CURSED Defer Runtime Implementation
/// Provides LIFO (Last In, First Out) defer execution
/// Integrated with LLVM-generated code for proper cleanup semantics

#define MAX_DEFER_STACK_SIZE 1000
#define MAX_SCOPE_DEPTH 100

/// Function pointer type for cleanup functions
typedef void (*CleanupFuncPtr)(void);

/// Defer stack entry
typedef struct {
    CleanupFuncPtr cleanup_func;
    uint32_t scope_id;
    bool is_error_safe;
} DeferEntry;

/// Scope management
typedef struct {
    uint32_t scope_id;
    size_t defer_count_start;
    bool is_function_scope;
} ScopeInfo;

/// Global defer state
static DeferEntry defer_stack[MAX_DEFER_STACK_SIZE];
static ScopeInfo scope_stack[MAX_SCOPE_DEPTH];
static size_t defer_stack_count = 0;
static size_t scope_stack_count = 0;
static uint32_t current_scope_id = 0;
static bool defer_runtime_initialized = false;

/// Initialize defer runtime
void cursed_defer_init(void) {
    defer_stack_count = 0;
    scope_stack_count = 0;
    current_scope_id = 0;
    defer_runtime_initialized = true;
    printf("✅ CURSED defer runtime initialized\n");
}

/// Cleanup defer runtime
void cursed_defer_cleanup(void) {
    if (defer_runtime_initialized) {
        cursed_defer_execute_all();
        defer_runtime_initialized = false;
        printf("✅ CURSED defer runtime cleanup completed\n");
    }
}

/// Push cleanup function onto defer stack
void cursed_defer_push(void* cleanup_func) {
    if (!defer_runtime_initialized) {
        cursed_defer_init();
    }
    
    if (defer_stack_count >= MAX_DEFER_STACK_SIZE) {
        fprintf(stderr, "❌ Error: Defer stack overflow (max %d entries)\n", MAX_DEFER_STACK_SIZE);
        return;
    }
    
    if (cleanup_func == NULL) {
        fprintf(stderr, "❌ Error: NULL cleanup function passed to defer_push\n");
        return;
    }
    
    defer_stack[defer_stack_count] = (DeferEntry){
        .cleanup_func = (CleanupFuncPtr)cleanup_func,
        .scope_id = current_scope_id,
        .is_error_safe = true
    };
    defer_stack_count++;
    
    printf("📌 Defer pushed: stack size now %zu\n", defer_stack_count);
}

/// Execute all defer functions in LIFO order
void cursed_defer_execute_all(void) {
    printf("🔄 Executing %zu defer functions in LIFO order\n", defer_stack_count);
    
    // Execute in reverse order (LIFO - Last In, First Out)
    while (defer_stack_count > 0) {
        defer_stack_count--;
        DeferEntry entry = defer_stack[defer_stack_count];
        
        if (entry.cleanup_func != NULL) {
            printf("⚡ Executing defer function (scope: %u)\n", entry.scope_id);
            entry.cleanup_func();
        } else {
            printf("⚠️ Warning: NULL cleanup function at stack position %zu\n", defer_stack_count);
        }
    }
    
    printf("✅ All defer functions executed\n");
}

/// Execute defer functions up to a specific count (for scoped cleanup)
void cursed_defer_execute_to_count(size_t target_count) {
    if (target_count > defer_stack_count) {
        printf("⚠️ Warning: target count %zu > current stack size %zu\n", target_count, defer_stack_count);
        return;
    }
    
    printf("🔄 Executing defer functions from %zu down to %zu\n", defer_stack_count, target_count);
    
    while (defer_stack_count > target_count) {
        defer_stack_count--;
        DeferEntry entry = defer_stack[defer_stack_count];
        
        if (entry.cleanup_func != NULL) {
            printf("⚡ Executing scoped defer function (scope: %u)\n", entry.scope_id);
            entry.cleanup_func();
        }
    }
    
    printf("✅ Scoped defer execution completed\n");
}

/// Get current defer stack size
size_t cursed_defer_get_stack_size(void) {
    return defer_stack_count;
}

/// Enter a new scope
uint32_t cursed_defer_enter_scope(void) {
    if (scope_stack_count >= MAX_SCOPE_DEPTH) {
        fprintf(stderr, "❌ Error: Scope stack overflow (max %d scopes)\n", MAX_SCOPE_DEPTH);
        return current_scope_id;
    }
    
    current_scope_id++;
    
    scope_stack[scope_stack_count] = (ScopeInfo){
        .scope_id = current_scope_id,
        .defer_count_start = defer_stack_count,
        .is_function_scope = false
    };
    scope_stack_count++;
    
    printf("📍 Entered scope %u (total scopes: %zu)\n", current_scope_id, scope_stack_count);
    return current_scope_id;
}

/// Exit a scope and execute its defers
void cursed_defer_exit_scope(uint32_t scope_id) {
    if (scope_stack_count == 0) {
        printf("⚠️ Warning: No scopes to exit\n");
        return;
    }
    
    printf("📍 Exiting scope %u\n", scope_id);
    
    // Find and remove the scope
    bool scope_found = false;
    size_t scope_index = 0;
    
    for (size_t i = 0; i < scope_stack_count; i++) {
        if (scope_stack[i].scope_id == scope_id) {
            scope_found = true;
            scope_index = i;
            break;
        }
    }
    
    if (!scope_found) {
        printf("⚠️ Warning: Scope %u not found\n", scope_id);
        return;
    }
    
    ScopeInfo scope_info = scope_stack[scope_index];
    
    // Execute defers for this scope in LIFO order
    size_t executed_count = 0;
    for (size_t i = defer_stack_count; i > 0; i--) {
        size_t index = i - 1;
        DeferEntry* entry = &defer_stack[index];
        
        if (entry->scope_id == scope_id) {
            printf("⚡ Executing defer for scope %u\n", scope_id);
            if (entry->cleanup_func != NULL) {
                entry->cleanup_func();
            }
            executed_count++;
            
            // Shift remaining entries down
            for (size_t j = index; j < defer_stack_count - 1; j++) {
                defer_stack[j] = defer_stack[j + 1];
            }
            defer_stack_count--;
        }
    }
    
    // Remove scope from stack
    for (size_t i = scope_index; i < scope_stack_count - 1; i++) {
        scope_stack[i] = scope_stack[i + 1];
    }
    scope_stack_count--;
    
    printf("✅ Scope %u exited, executed %zu defer functions\n", scope_id, executed_count);
}

/// Execute error-safe defers during error unwinding
void cursed_defer_execute_on_error(void) {
    printf("💥 Executing error-safe defer functions due to error unwinding\n");
    
    size_t executed_count = 0;
    
    // Execute only error-safe defers in LIFO order
    for (size_t i = defer_stack_count; i > 0; i--) {
        size_t index = i - 1;
        DeferEntry entry = defer_stack[index];
        
        if (entry.is_error_safe && entry.cleanup_func != NULL) {
            printf("⚡ Executing error-safe defer function (scope: %u)\n", entry.scope_id);
            entry.cleanup_func();
            executed_count++;
        }
    }
    
    // Clear the stack after error unwinding
    defer_stack_count = 0;
    
    printf("✅ Error unwinding completed, executed %zu error-safe defer functions\n", executed_count);
}

/// Clear all defer functions (emergency cleanup)
void cursed_defer_clear_all(void) {
    defer_stack_count = 0;
    scope_stack_count = 0;
    printf("🧹 All defer functions cleared\n");
}

/// Mark current scope as function scope
void cursed_defer_enter_function_scope(void) {
    if (scope_stack_count > 0) {
        scope_stack[scope_stack_count - 1].is_function_scope = true;
        printf("🏢 Current scope marked as function scope\n");
    }
}

/// Print defer runtime status (for debugging)
void cursed_defer_print_status(void) {
    printf("🔍 Defer Runtime Status:\n");
    printf("   - Defer stack: %zu/%d entries\n", defer_stack_count, MAX_DEFER_STACK_SIZE);
    printf("   - Scope stack: %zu/%d entries\n", scope_stack_count, MAX_SCOPE_DEPTH);
    printf("   - Current scope ID: %u\n", current_scope_id);
    printf("   - Initialized: %s\n", defer_runtime_initialized ? "true" : "false");
    
    if (defer_stack_count > 0) {
        printf("   - Defer entries:\n");
        for (size_t i = 0; i < defer_stack_count; i++) {
            printf("     [%zu] scope: %u, error_safe: %s\n", 
                   i, defer_stack[i].scope_id, 
                   defer_stack[i].is_error_safe ? "true" : "false");
        }
    }
}
