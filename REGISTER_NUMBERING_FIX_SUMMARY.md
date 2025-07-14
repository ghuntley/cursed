# LLVM IR Register Numbering Fix Summary

## Issues Identified

1. **Multiple Register Counters**: Different LLVM codegen modules (`main.rs`, `expression_compiler.rs`, `function_compilation.rs`) had separate `variable_counter` fields causing register numbering conflicts.

2. **Missing Method Access**: The `next_register()` method was private, preventing other modules from accessing synchronized register allocation.

3. **Goroutine/Channel Register Conflicts**: Goroutine and channel operations were using separate register numbering causing conflicts.

## Solution Implemented

### 1. Global Register Tracker
- Created `register_tracker.rs` with a global synchronized counter
- Uses thread-safe `Mutex` and static variables for global synchronization
- All register allocation goes through `RegisterTracker::allocate_register()`

### 2. Updated Main Generator
- Replaced `variable_counter: usize` with `register_tracker: RegisterTracker`
- Made `next_register()` method public for external access
- Reset global counter on each compilation via `RegisterTracker::set_global_counter(1)`

### 3. Updated Expression Compiler
- Replaced `variable_counter` with `register_tracker: RegisterTracker`
- Synchronized with global counter on initialization
- Uses global register allocation for consistent numbering

### 4. Updated Function Compiler
- Replaced `variable_counter` with `register_tracker: RegisterTracker` 
- Automatic synchronization with global counter
- Consistent register numbering across function boundaries

## Current Status

**Partially Fixed**: The register tracking infrastructure is implemented but compilation has additional issues that need to be resolved:

1. Missing methods in `LlvmCodeGenerator` (package integration, optimization methods)
2. Runtime GC issues with type mismatches
3. Missing `next_label()` method implementation

## Next Steps

1. **Implement Missing Methods**: Add package integration and optimization methods to `LlvmCodeGenerator`
2. **Fix Runtime Issues**: Resolve GC type mismatches by dereferencing pointers correctly
3. **Add Label Management**: Implement synchronized label tracking similar to register tracking
4. **Test Validation**: Run comprehensive tests to verify register numbering consistency

## Benefits of the Fix

- **Consistent Numbering**: All LLVM modules use the same register counter
- **Thread Safety**: Global synchronization prevents race conditions
- **Scalability**: Easy to extend to new codegen modules
- **Debugging**: Clear register allocation sequence for debugging

## Files Modified

- `src/codegen/llvm/register_tracker.rs` (new)
- `src/codegen/llvm/main.rs` (updated)  
- `src/codegen/llvm/expression_compiler.rs` (updated)
- `src/codegen/llvm/function_compilation.rs` (updated)
- `src/codegen/llvm/goroutine.rs` (updated)
- `src/codegen/llvm/mod.rs` (updated to export register_tracker)

## Testing Commands

Once compilation issues are resolved:

```bash
# Test LLVM compilation
cargo test llvm --lib

# Test register numbering
cargo run --bin cursed -- compile test_register_numbering.csd

# Verify LLVM IR consistency
cat test_register_numbering.ll | grep '%[0-9]' | sort -n
```
