# CURSED Compiler Architectural Plan
## Systematic Resolution of Core Issues to Achieve 90%+ Pass Rate

### Current State Analysis (71% Pass Rate - 318/444 tests)
**Strengths Achieved:**
- ✅ Complete stdlib function implementation (mathz, stringz, vibez)
- ✅ Control flow working (if/else, while, for loops)
- ✅ Arithmetic expressions and variable resolution
- ✅ LLVM function call infrastructure implemented
- ✅ Return type mapping and return statement compilation

**Critical Issues Identified:**
1. **Parser Memory Corruption** (73 interpreter errors)
2. **LLVM Runtime Integration Gap** (user functions return 0)  
3. **Compile-time vs Runtime Architecture Conflict**

---

## Phase 1: Parser Foundation Stabilization
**Target: 80% Pass Rate - Eliminate Interpreter Errors**

### Issue 1.1: Parser Memory Corruption
**Root Cause:** ArrayList initialization with `{}` causing undefined memory access
**Impact:** Function names corrupted during parsing (test_return → spill)
**Current Status:** 3 critical sites fixed, ~15 sites remaining

**Implementation Plan:**
```zig
// Current (corrupted):
var arguments = ArrayList(*Expression){};

// Fixed (safe):  
var arguments = std.ArrayList(*Expression){ .items = &.{}, .capacity = 0 };
```

**Action Items:**
- [ ] Audit all ArrayList(`{}`} usage in parser.zig (15+ instances found)
- [ ] Replace with proper zero-initialization pattern  
- [ ] Add compiler flag `-fsanitize=memory` for detection
- [ ] Verify with complex function parsing tests

**Estimated Impact:** +50 tests (resolve 73 interpreter errors)

### Issue 1.2: Function Registration & Forward Declarations
**Root Cause:** Parser state confusion between method calls and function definitions
**Impact:** Functions with return types not properly registered

**Implementation Plan:**
- [ ] Complete `.Spill` keyword removal from remaining parser sites
- [ ] Enhance function registration debug logging
- [ ] Implement robust parser state recovery mechanisms
- [ ] Add function signature validation during registration

**Estimated Impact:** +15 tests (complex function definitions)

---

## Phase 2: LLVM Runtime Integration
**Target: 85% Pass Rate - User Functions Working**

### Issue 2.1: Function Parameter Handling
**Root Cause:** Function parameters not available in LLVM compilation context
**Current:** `⚠️ Variable a not found, returning 0`

**Implementation Plan:**
```zig
// Current (broken):
fn declareCursedFunction(...) {
    var param_types: [0]llvm.Builder.Type = .{}; // No parameters
}

// Fixed (working):
fn declareCursedFunction(...) {
    var param_types = ArrayList(llvm.Builder.Type){};
    for (func_stmt.parameters.items) |param| {
        const param_type = mapCursedTypeToLLVM(param.type);
        try param_types.append(param_type);
    }
}
```

**Action Items:**
- [ ] Implement parameter type parsing in `declareCursedFunction`
- [ ] Add parameter variable registration in `implementCursedFunction`  
- [ ] Map CURSED parameter types to LLVM types consistently
- [ ] Test with functions containing multiple typed parameters

### Issue 2.2: Runtime Result Capture Architecture
**Root Cause:** Compile-time evaluation can't handle runtime function calls
**Current:** User function calls evaluate to 0 at compile-time

**Two Strategic Options:**

#### Option A: Dynamic Variable Resolution (Recommended)
Implement runtime variable loading in LLVM IR instead of compile-time capture:

```zig
// Instead of compile-time: result = 0
// Generate LLVM IR: %result = call @add_numbers(i64 10, i64 5)  
// When vibez.spill(result), load %result at runtime
```

#### Option B: Function Call Result Caching
Extend the current static analysis to cache function call results:

```zig
// Cache runtime results from LLVM execution
// Map function_call_signature -> computed_result
// Reuse in subsequent compile-time evaluations
```

**Recommended Approach:** Option A (Dynamic Variable Resolution)

**Implementation Plan:**
- [ ] Implement LLVM variable loading for function call results
- [ ] Extend `compileIdentifierLoad` for runtime-computed variables
- [ ] Add variable type tracking in LLVM IR generation
- [ ] Remove forced compile-time evaluation for `.Call` expressions

**Estimated Impact:** +25 tests (user-defined function calls working)

---

## Phase 3: Architecture Unification  
**Target: 90%+ Pass Rate - Production Ready**

### Issue 3.1: Hybrid Compilation Model
**Goal:** Seamless integration of compile-time and runtime evaluation

**Architecture Design:**
```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ Static Analysis │────│ Hybrid Evaluator │────│ Runtime System  │
│                 │    │                  │    │                 │  
│ • Stdlib calls  │    │ • Smart dispatch │    │ • User functions│
│ • Arithmetic    │    │ • Value caching  │    │ • Complex expr  │
│ • Literals      │    │ • Type inference │    │ • Control flow  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

**Implementation Components:**
- [ ] **Smart Expression Evaluator**: Decides compile-time vs runtime per expression type
- [ ] **Value Type System**: Track static vs dynamic values consistently  
- [ ] **Unified Variable Resolution**: Single mechanism for both compilation paths
- [ ] **Result Propagation**: Proper value flow from runtime to compile-time contexts

### Issue 3.2: Advanced Language Features
**After core architecture stabilizes:**

**Control Flow Enhancements:**
- [ ] Fix if statement branch logic (both branches executing issue)
- [ ] Implement proper short-circuit evaluation (&& and ||)
- [ ] Add switch/vibe_check statement support

**Type System Completion:**  
- [ ] Array operations and indexing
- [ ] Pointer arithmetic and dereferencing
- [ ] Struct member access and manipulation
- [ ] Advanced expression parsing

**Error Handling & Edge Cases:**
- [ ] Proper division by zero handling parity  
- [ ] Float precision formatting consistency
- [ ] Memory safety improvements
- [ ] Overflow detection alignment

**Estimated Impact:** +20-30 tests (advanced feature coverage)

---

## Implementation Timeline & Prioritization

### Phase 1: Foundation (High-Impact, Immediate) - 1-2 days
**Priority 1:** Parser memory corruption fixes
- Highest impact/effort ratio
- Directly resolves 73 interpreter errors
- Essential foundation for other improvements

**Priority 2:** Function registration stability
- Enables proper interpreter vs compiler comparison
- Required for meaningful progress measurement

### Phase 2: Runtime Integration (Medium-Impact, Complex) - 3-4 days  
**Priority 1:** Function parameter handling in LLVM
- Unblocks user-defined function testing
- Required for language feature completeness

**Priority 2:** Dynamic variable resolution system
- Major architectural enhancement
- Enables runtime function call results

### Phase 3: Unification (Lower-Impact, Polish) - 2-3 days
**Priority 1:** Hybrid evaluation system
- Performance optimization and consistency
- Production-ready compiler quality

**Priority 2:** Advanced language features
- Completeness and edge case handling
- Path to 95%+ pass rates

---

## Success Metrics & Validation

### Phase 1 Success Criteria:
- [ ] Pass rate: 71% → 80%+ 
- [ ] Interpreter errors: 73 → <5
- [ ] Complex function definitions parse correctly
- [ ] Memory corruption eliminated (no more function name changes)

### Phase 2 Success Criteria:  
- [ ] Pass rate: 80% → 85%+
- [ ] User-defined functions return correct values
- [ ] Function parameters accessible in function bodies  
- [ ] Runtime function call results properly captured

### Phase 3 Success Criteria:
- [ ] Pass rate: 85% → 90%+
- [ ] Production-ready stability for all core language features
- [ ] Consistent behavior between interpreter and compiled modes
- [ ] Advanced language constructs working reliably

### Risk Mitigation:
- **Incremental Testing**: Validate improvements after each sub-phase
- **Rollback Capability**: Maintain working state at each phase boundary  
- **Progress Measurement**: Continuous pass rate tracking and regression detection

---

## Architectural Decision Summary

**Immediate Focus:** Parser foundation stabilization provides highest-impact improvements with manageable complexity.

**Strategic Direction:** Hybrid compile-time/runtime evaluation model enables both performance (compile-time when possible) and completeness (runtime when necessary).

**Long-term Vision:** Production-ready self-hosting compiler with 95%+ test coverage and robust language feature support.

This plan provides a systematic approach to resolving the identified architectural issues while maintaining the significant progress achieved in this session.
