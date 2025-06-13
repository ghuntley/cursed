# CURSED Goroutine LLVM Integration

This document explains the real LLVM integration for CURSED goroutines, which enables the compilation of cooperative concurrency primitives (`stan`, `yolo`) into executable machine code.

## Overview

The goroutine LLVM integration replaces placeholder implementations with real inkwell-based code generation that:

1. **Compiles `stan` keyword** - Spawns goroutines by calling the runtime scheduler
2. **Generates `yolo` yield points** - Inserts cooperative scheduling points in loops
3. **Creates GC safe points** - Enables garbage collection coordination
4. **Integrates with runtime** - Connects compiled code with the goroutine scheduler

## Architecture

### Components

1. **`src/codegen/llvm/goroutine.rs`** - Main LLVM integration module
2. **`src/runtime/goroutine.rs`** - Runtime goroutine scheduler and FFI functions
3. **`src/codegen/llvm.rs`** - Main LLVM code generator integration

### Key Traits and Types

```rust
/// Main trait for goroutine compilation
pub trait GoroutineCompiler<'ctx> {
    fn compile_goroutine_spawn(&mut self, spawn: &GoroutineSpawn) -> Result<BasicValueEnum<'ctx>, Error>;
    fn generate_yield_point(&mut self, location: &str) -> Result<(), Error>;
    fn generate_safe_point(&mut self, location: &str) -> Result<(), Error>;
    fn setup_goroutine_runtime(&mut self) -> Result<PointerValue<'ctx>, Error>;
    fn declare_goroutine_runtime_functions(&mut self) -> Result<(), Error>;
}
```

## LLVM Code Generation

### Function Declarations

The integration automatically declares these runtime functions in LLVM modules:

```llvm
; Spawn a new goroutine
declare i64 @cursed_spawn_goroutine(i8*, i8*)

; Yield current goroutine
declare void @cursed_yield_goroutine(i8*)

; Signal GC safe point
declare void @cursed_safe_point(i8*, i8*)

; Check if GC coordination requested
declare i1 @cursed_gc_requested(i8*)
```

### Goroutine Spawn (`stan` keyword)

When compiling `stan function_name`, the integration:

1. **Resolves the target function** in the LLVM module
2. **Creates a function pointer** by casting to `i8*`
3. **Calls the runtime spawn function** with scheduler and function pointers
4. **Returns goroutine ID** as an `i64` value

Example generated IR:
```llvm
%target_fn_ptr = bitcast void ()* @background_task to i8*
%goroutine_id = call i64 @cursed_spawn_goroutine(i8* %scheduler_ptr, i8* %target_fn_ptr)
```

### Yield Points (`yolo` keyword)

When compiling `yolo` in loops, the integration:

1. **Checks if GC coordination is requested** using `cursed_gc_requested`
2. **Creates conditional branch** to yield or continue
3. **Calls yield function** if GC coordination needed
4. **Continues execution** after yield completes

Example generated IR:
```llvm
%gc_requested = call i1 @cursed_gc_requested(i8* %scheduler_ptr)
br i1 %gc_requested, label %yield_block, label %continue_block

yield_block:
  call void @cursed_yield_goroutine(i8* %scheduler_ptr)
  br label %continue_block

continue_block:
  ; Continue loop execution
```

### Safe Points

Safe points are inserted at strategic locations for GC coordination:

1. **Function entry/exit**
2. **Loop headers**
3. **Allocation sites**
4. **Manual `yolo` statements**

Example generated IR:
```llvm
%location_str = getelementptr inbounds [13 x i8], [13 x i8]* @str_const, i32 0, i32 0
call void @cursed_safe_point(i8* %scheduler_ptr, i8* %location_str)
```

## Runtime Integration

### Scheduler Management

The integration uses a global scheduler pointer for coordination:

```rust
// Set global scheduler (typically at program startup)
set_runtime_scheduler(scheduler_ptr);

// Get scheduler in compiled code
let scheduler = get_runtime_scheduler().expect("No scheduler available");
```

### FFI Functions

The runtime provides C-compatible functions for LLVM-generated code:

```rust
/// Spawn goroutine from function pointer
#[no_mangle]
pub extern "C" fn cursed_spawn_goroutine(
    scheduler_ptr: *mut GoroutineScheduler,
    function_ptr: *const u8,
) -> u64;

/// Yield current goroutine for cooperative scheduling
#[no_mangle]
pub extern "C" fn cursed_yield_goroutine(scheduler_ptr: *mut GoroutineScheduler);

/// Signal safe point for GC coordination
#[no_mangle]
pub extern "C" fn cursed_safe_point(
    scheduler_ptr: *mut GoroutineScheduler,
    location: *const std::os::raw::c_char,
);

/// Check if GC coordination is requested
#[no_mangle]
pub extern "C" fn cursed_gc_requested(scheduler_ptr: *mut GoroutineScheduler) -> bool;
```

## Usage Example

### CURSED Code

```cursed
// Background processing function
slay process_data() {
    lowkey (sus i = 0; i < 1000; i++) {
        // Process some data
        process_item(i);
        
        // Yield point for cooperative scheduling
        yolo;
    }
}

// Main function
slay main() {
    // Spawn goroutine to run background processing
    stan process_data();
    
    // Continue with main logic
    println("Background processing started");
}
```

### Generated LLVM IR

```llvm
define void @process_data() {
entry:
  ; Safe point at function entry
  call void @cursed_safe_point(i8* %scheduler_ptr, i8* getelementptr inbounds ([14 x i8], [14 x i8]* @.str.func_entry, i32 0, i32 0))
  
  br label %loop

loop:
  ; Loop body - process item
  call void @process_item(i32 %i)
  
  ; Yield point (yolo)
  %gc_requested = call i1 @cursed_gc_requested(i8* %scheduler_ptr)
  br i1 %gc_requested, label %yield, label %continue

yield:
  call void @cursed_yield_goroutine(i8* %scheduler_ptr)
  br label %continue

continue:
  ; Continue loop or exit
  ; ...
}

define void @main() {
entry:
  ; Spawn goroutine
  %target_fn = bitcast void ()* @process_data to i8*
  %goroutine_id = call i64 @cursed_spawn_goroutine(i8* %scheduler_ptr, i8* %target_fn)
  
  ; Continue with main logic
  call void @println(i8* getelementptr inbounds ([27 x i8], [27 x i8]* @.str.msg, i32 0, i32 0))
  ret void
}
```

## Memory Safety and Thread Safety

### Pointer Safety

- **Null pointer checks** in all FFI functions
- **Bounds checking** for string operations
- **Type safety** through inkwell's type system

### Scheduler Safety

- **Arc/Mutex synchronization** for shared scheduler state
- **Atomic operations** for coordination flags
- **Lock-free operations** where possible

### Stack Safety

- **Real stack allocation** with proper bounds
- **Stack scanning** for GC roots
- **Automatic cleanup** on goroutine completion

## Performance Characteristics

### Code Generation

- **Zero-cost abstractions** - goroutine operations compile to direct function calls
- **Minimal overhead** - yield points are simple conditional branches
- **Efficient coordination** - GC safe points use atomic flags

### Runtime Performance

- **Cooperative scheduling** - no preemption overhead
- **Work-stealing scheduler** - good load balancing
- **Stack management** - 64KB default stacks with configurable sizes

## Error Handling

### Compilation Errors

- **Missing scheduler** - Error if no global scheduler available
- **Invalid functions** - Error for non-existent spawn targets
- **Type mismatches** - Compile-time checking through inkwell

### Runtime Errors

- **Spawn failures** - Return 0 goroutine ID on failure
- **Yield failures** - Log error but continue execution
- **Null pointers** - Safe handling with error logging

## Testing

### Integration Tests

The `tests/goroutine_llvm_integration_test.rs` file provides comprehensive tests:

- **Function declaration** - Verify FFI functions are declared correctly
- **Runtime setup** - Test scheduler initialization and cleanup
- **Code generation** - Verify correct LLVM IR generation
- **Safe points** - Test GC coordination points
- **Full integration** - End-to-end compilation and execution

### Usage

```bash
# Run goroutine LLVM integration tests
cargo test --test goroutine_llvm_integration_test

# Run with verbose output
RUST_LOG=debug cargo test --test goroutine_llvm_integration_test
```

## Future Enhancements

### Planned Features

1. **Argument passing** - Support function arguments in goroutine spawn
2. **Return values** - Channel-based result communication
3. **Error propagation** - Panic handling across goroutine boundaries
4. **Optimization** - Inlining of yield points and safe points
5. **Debugging** - DWARF debug information for goroutines

### Advanced Integration

1. **Custom schedulers** - Support for different scheduling policies
2. **NUMA awareness** - Thread affinity and memory locality
3. **Async integration** - Bridge with async/await runtimes
4. **JIT compilation** - Dynamic function compilation for hot paths

## Troubleshooting

### Common Issues

1. **No scheduler available** - Ensure `initialize_scheduler()` is called
2. **Function not found** - Verify target function exists in module
3. **Null pointer errors** - Check FFI function arguments
4. **Module verification fails** - Ensure complete function definitions

### Debug Tips

1. **Enable tracing** - Set `RUST_LOG=debug` for detailed logs
2. **Print IR** - Use `module.print_to_string()` to inspect generated code
3. **Step through compilation** - Use debugger to trace code generation
4. **Verify scheduler state** - Check active goroutines and scheduler status

## Conclusion

The CURSED goroutine LLVM integration provides a production-ready implementation of cooperative concurrency that:

- **Compiles to efficient machine code** using real LLVM IR generation
- **Integrates seamlessly** with the runtime scheduler
- **Provides memory safety** through proper pointer management
- **Enables GC coordination** through safe points and yield points
- **Supports testing** with comprehensive integration tests

This implementation replaces the previous placeholder system with a fully functional goroutine compilation pipeline suitable for production use in the CURSED programming language.
