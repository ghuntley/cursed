# CURSED LLVM Backend Design: CURSED Stdlib Compilation

## Executive Summary

This document outlines the architectural design for enabling direct CURSED→LLVM compilation, allowing CURSED stdlib functions to be compiled to native code instead of relying on Zig runtime functions. This represents a critical step toward CURSED self-hosting.

## Current Architecture Analysis

### LLVM Backend Status: ✅ FUNCTIONAL
- **File:** `/home/ghuntley/cursed/src-zig/llvm_ir_pipeline.zig` (1000+ lines)
- **Capability:** Full CURSED→LLVM IR→Native Binary compilation pipeline
- **Method Call Translation:** Pattern-matching system for `mathz.add()` → `mathz_add()` runtime calls
- **Integration:** Links with `runtime/libcursed_stdlib.a` containing Zig implementations

### CURSED Stdlib Status: ✅ FUNCTIONAL  
- **Interpreter Loading:** 100% functional with 280+ modules loaded
- **File:** `/home/ghuntley/cursed/src-zig/interpreter.zig` - `loadCursedStdlibModule()` function
- **Coverage:** Full stdlib including mathz, stringz, vibez, collections, json, regex, etc.
- **Implementation:** Pure CURSED code with 850+ lines in stringz alone

### Current Compilation Flow
```
CURSED Program → AST → LLVM IR → [CALLS] Zig Runtime Functions → Native Binary
                                      ↓
                               runtime/cursed_stdlib.zig
                               (mathz_add, mathz_sub, etc.)
```

### Target Architecture
```
CURSED Program → AST → [INTEGRATE] CURSED Stdlib AST → LLVM IR → Native Binary
                           ↓
                      stdlib/mathz/mod.💀  
                      (CURSED implementations)
```

## Design Strategy

### Phase 1: CURSED Function Compilation Infrastructure

**Objective:** Enable LLVM backend to compile individual CURSED functions to IR

**Key Components:**

1. **CURSED Function Compiler** (`src-zig/cursed_function_compiler.zig`)
   ```zig
   pub const CursedFunctionCompiler = struct {
       llvm_pipeline: *LLVMIRPipeline,
       
       pub fn compileCursedFunction(
           self: *CursedFunctionCompiler,
           func_ast: ast.FunctionStatement,
           module_name: []const u8
       ) !c.LLVMValueRef
   };
   ```

2. **Hybrid Compilation Strategy**
   - **Priority 1:** Attempt to compile CURSED implementation
   - **Priority 2:** Fall back to Zig runtime function on failure
   - **Validation:** Compare outputs between CURSED and Zig implementations

3. **Integration Points:**
   - Modify `generateMethodCall()` in `llvm_ir_pipeline.zig`
   - Add CURSED stdlib loading to LLVM backend initialization
   - Create function registry for CURSED vs Zig function resolution

### Phase 2: Stdlib Module Integration  

**Objective:** Load and compile entire CURSED stdlib modules during LLVM compilation

**Architecture:**

1. **Module Loading System**
   ```zig
   pub const CursedStdlibCompiler = struct {
       modules: HashMap([]const u8, *CompiledModule),
       
       pub fn loadModule(self: *Self, name: []const u8) !*CompiledModule {
           // Load stdlib/{name}/mod.💀
           // Parse to AST
           // Compile all functions to LLVM IR
           // Register in function table
       }
   };
   ```

2. **Function Resolution Strategy**
   ```zig
   fn resolveStdlibCall(self: *LLVMIRPipeline, 
                       module: []const u8, 
                       function: []const u8) !c.LLVMValueRef {
       // Check if CURSED implementation exists and is compiled
       if (self.cursed_functions.get(module, function)) |cursed_fn| {
           return cursed_fn;
       }
       
       // Fall back to Zig runtime
       return self.getOrDeclareRuntimeFunction(...);
   }
   ```

### Phase 3: Advanced Code Generation

**Objective:** Optimize CURSED stdlib function compilation

**Features:**

1. **Inlining Strategy**
   - Simple functions (< 10 IR instructions) → Always inline
   - Complex functions → Compile as separate functions
   - Recursive functions → Never inline

2. **Type Optimization**
   - CURSED `drip` (i64) → LLVM i64
   - CURSED `meal` (f64) → LLVM double
   - CURSED `lit` (bool) → LLVM i1
   - CURSED `tea` (string) → LLVM i8*

3. **Error Handling**
   - CURSED `yikes<T>` → LLVM struct { i1 is_error, T value }
   - Automatic error propagation in calling functions

## Implementation Plan

### Phase 1 Tasks (Week 1-2)

1. **✅ Create CursedFunctionCompiler**
   - File: `src-zig/cursed_function_compiler.zig`
   - Basic CURSED function → LLVM IR compilation
   - Handle simple arithmetic functions first

2. **✅ Modify Method Call Generation**
   - Update `generateMethodCall()` in `llvm_ir_pipeline.zig`
   - Add CURSED function resolution before Zig runtime fallback
   - Implement hybrid compilation strategy

3. **✅ Test with mathz Functions**
   - Compile `mathz.abs_normie()` from CURSED to IR
   - Compare output with Zig implementation
   - Validate correctness and performance

### Phase 2 Tasks (Week 3-4)

1. **✅ Module Loading Integration**
   - Integrate `loadCursedStdlibModule()` from interpreter
   - Create CURSED → LLVM compilation pipeline
   - Handle module dependencies and imports

2. **✅ Function Registry System**
   - Track compiled CURSED functions
   - Maintain mapping: module.function → LLVM IR
   - Handle function overloading and signatures

3. **✅ Full mathz Module Compilation**
   - Compile entire `stdlib/mathz/mod.💀` (516 lines)
   - Replace all `mathz_*` runtime calls with CURSED IR
   - Performance benchmarking vs Zig runtime

### Phase 3 Tasks (Week 5-6)

1. **✅ Advanced Features**
   - Error handling (`yikes<T>` types)
   - Complex control flow (`bestie` loops, `ready` conditionals)
   - Function calls between CURSED functions

2. **✅ Multi-Module Compilation**
   - stringz, vibez, collections modules
   - Cross-module function calls
   - Module initialization and dependencies

3. **✅ Optimization Pass**
   - Function inlining for simple functions
   - Dead code elimination
   - Constant folding for CURSED expressions

## Technical Challenges & Solutions

### Challenge 1: Type System Integration
**Problem:** CURSED types need accurate LLVM representation
**Solution:** Extend `cursedTypeToLLVM()` with comprehensive type mapping

### Challenge 2: Error Handling
**Problem:** CURSED `yikes<T>` error types don't map directly to LLVM
**Solution:** Use LLVM struct types for error unions, similar to Rust/Zig approach

### Challenge 3: Function Resolution
**Problem:** Determining when to use CURSED vs Zig implementation  
**Solution:** Priority-based resolution with fallback mechanism

### Challenge 4: Debugging & Validation
**Problem:** Ensuring CURSED compilation correctness vs Zig runtime
**Solution:** Dual compilation mode with output comparison testing

## Success Metrics

### Phase 1 Success Criteria
- [ ] `mathz.abs_normie()` compiles from CURSED to correct LLVM IR
- [ ] Binary output matches Zig runtime implementation
- [ ] Performance within 5% of Zig runtime

### Phase 2 Success Criteria  
- [ ] Full `mathz` module (50+ functions) compiles successfully
- [ ] All existing test cases pass with CURSED implementations
- [ ] Compilation time reasonable (< 2x slower than current)

### Phase 3 Success Criteria
- [ ] Multi-module compilation (mathz, stringz, vibez)
- [ ] Self-hosting capability demonstrated
- [ ] Performance parity or improvement vs Zig runtime

## Integration Strategy

### Minimal Disruption Approach
1. **Keep existing Zig runtime** as fallback during development
2. **Gradual migration** - enable CURSED compilation per function/module
3. **Validation testing** - compare outputs between implementations
4. **Performance monitoring** - ensure no regression

### Build System Changes
```toml
# CursedBuild.toml
[compilation]
stdlib_mode = "hybrid"  # "cursed", "zig", or "hybrid"
cursed_modules = ["mathz", "stringz"]  # Enable CURSED compilation
fallback_runtime = true  # Keep Zig runtime as fallback
```

## File Structure

```
src-zig/
├── llvm_ir_pipeline.zig          # Main LLVM backend (existing)
├── cursed_function_compiler.zig  # New: CURSED function compilation  
├── cursed_stdlib_compiler.zig    # New: Module-level compilation
└── hybrid_resolver.zig           # New: CURSED vs Zig resolution

stdlib/
├── mathz/mod.💀                 # CURSED math functions (existing)
├── stringz/mod.💀               # CURSED string functions (existing)
└── vibez/mod.💀                 # CURSED I/O functions (existing)

runtime/
└── cursed_stdlib.zig             # Zig runtime (existing, keep as fallback)
```

## Risk Assessment & Mitigation

| Risk | Impact | Mitigation |
|------|--------|------------|
| CURSED compilation bugs | High | Dual compilation with validation testing |
| Performance regression | Medium | Benchmarking and optimization passes |
| Increased compilation time | Low | Lazy compilation and caching strategies |
| Type system complexity | Medium | Gradual type system extension |

## Conclusion

This design enables a seamless transition from Zig runtime to CURSED native compilation while maintaining backward compatibility and validation. The hybrid approach ensures minimal risk while providing a clear path toward full CURSED self-hosting.

The implementation leverages existing, proven infrastructure (LLVM backend + CURSED stdlib loading) and extends it with targeted functionality for CURSED function compilation. This represents the final major component needed for CURSED self-hosting capabilities.

## Next Steps

1. **Create Phase 1 implementation files**
2. **Begin with simple mathz functions (abs_normie, max_normie)**  
3. **Validate against existing test suite**
4. **Expand to full mathz module**
5. **Scale to multi-module compilation**

This design document provides a comprehensive roadmap for achieving CURSED stdlib compilation while maintaining system stability and performance.
