# CURSED Optimization Passes Documentation

This document describes the critical LLVM optimization passes implemented for the CURSED compiler. These passes form the core of the optimization infrastructure and are essential for generating high-performance code.

## Overview

The CURSED compiler implements 7 critical optimization passes that work together to transform LLVM IR into highly optimized code:

1. **SCCP** (Sparse Conditional Constant Propagation)
2. **LICM** (Loop Invariant Code Motion)  
3. **GVN** (Global Value Numbering)
4. **SROA** (Scalar Replacement of Aggregates)
5. **Mem2Reg** (Memory to Register Promotion)
6. **Tail Call Optimization**
7. **Jump Threading**

## Optimization Pass Details

### 1. SCCP (Sparse Conditional Constant Propagation)

**Purpose**: More aggressive constant propagation than basic constant propagation by tracking values through control flow.

**What it does**:
- Tracks constant values across conditional branches
- Eliminates unreachable code paths
- Simplifies conditional expressions with known outcomes
- Uses lattice-based analysis for precise value tracking

**Example transformation**:
```llvm
; Before SCCP
%cond = icmp eq i32 %x, 5
br i1 %cond, label %then, label %else

then:
  %result = add i32 %x, 10  ; %x is known to be 5 here
  br label %merge

; After SCCP
br i1 %cond, label %then, label %else

then:
  %result = i32 15  ; Constant folded: 5 + 10 = 15
  br label %merge
```

**Implementation**: `src/codegen/llvm/passes/sccp.rs`

### 2. LICM (Loop Invariant Code Motion)

**Purpose**: Moves computations that don't change inside loops to outside the loop.

**What it does**:
- Identifies loop-invariant expressions
- Hoists invariant computations to loop preheaders
- Reduces redundant calculations in loops
- Improves cache locality by reducing instruction count in loops

**Example transformation**:
```llvm
; Before LICM
loop:
  %invariant = add i32 %a, %b  ; %a and %b don't change in loop
  %i = phi i32 [0, %entry], [%next_i, %loop]
  %result = add i32 %invariant, %i
  %next_i = add i32 %i, 1
  %cond = icmp slt i32 %next_i, 100
  br i1 %cond, label %loop, label %exit

; After LICM
preheader:
  %invariant = add i32 %a, %b  ; Hoisted out of loop
  br label %loop

loop:
  %i = phi i32 [0, %preheader], [%next_i, %loop]
  %result = add i32 %invariant, %i
  %next_i = add i32 %i, 1
  %cond = icmp slt i32 %next_i, 100
  br i1 %cond, label %loop, label %exit
```

**Implementation**: `src/codegen/llvm/passes/licm.rs`

### 3. GVN (Global Value Numbering)

**Purpose**: Eliminates redundant computations by identifying expressions that compute the same value.

**What it does**:
- Assigns value numbers to expressions
- Identifies and eliminates redundant calculations
- Works across basic block boundaries
- Enables load elimination and common subexpression elimination

**Example transformation**:
```llvm
; Before GVN
%1 = add i32 %a, %b
%2 = mul i32 %1, 2
; ... some other code ...
%3 = add i32 %a, %b  ; Redundant calculation
%4 = mul i32 %3, 2   ; Redundant calculation

; After GVN
%1 = add i32 %a, %b
%2 = mul i32 %1, 2
; ... some other code ...
; %3 and %4 are replaced with %1 and %2
```

**Implementation**: `src/codegen/llvm/passes/gvn.rs`

### 4. SROA (Scalar Replacement of Aggregates)

**Purpose**: Replaces struct and array allocations with individual scalar variables when beneficial.

**What it does**:
- Breaks down aggregate types into scalars
- Eliminates struct/array allocations where possible
- Enables better register allocation
- Exposes optimization opportunities for other passes

**Example transformation**:
```llvm
; Before SROA
%struct = alloca { i32, i32 }
%field1_ptr = getelementptr { i32, i32 }, { i32, i32 }* %struct, i32 0, i32 0
%field2_ptr = getelementptr { i32, i32 }, { i32, i32 }* %struct, i32 0, i32 1
store i32 %val1, i32* %field1_ptr
store i32 %val2, i32* %field2_ptr
%load1 = load i32, i32* %field1_ptr
%load2 = load i32, i32* %field2_ptr

; After SROA
; %struct allocation eliminated, replaced with scalars
; %load1 = %val1 (direct use)
; %load2 = %val2 (direct use)
```

**Implementation**: `src/codegen/llvm/passes/sroa.rs`

### 5. Mem2Reg (Memory to Register Promotion)

**Purpose**: Converts stack-allocated variables to SSA register values.

**What it does**:
- Promotes alloca/load/store patterns to direct value usage
- Inserts PHI nodes for values that change across control flow
- Essential foundation pass for other optimizations
- Converts imperative code to functional SSA form

**Example transformation**:
```llvm
; Before Mem2Reg
%var = alloca i32
store i32 %input, i32* %var
%temp = load i32, i32* %var
%result = add i32 %temp, 1
store i32 %result, i32* %var
%final = load i32, i32* %var

; After Mem2Reg
%temp = %input
%result = add i32 %temp, 1
%final = %result
```

**Implementation**: `src/codegen/llvm/passes/mem2reg.rs`

### 6. Tail Call Optimization

**Purpose**: Converts tail calls to jumps, eliminating stack frame overhead.

**What it does**:
- Identifies tail recursive calls
- Converts recursive calls to loops where possible
- Marks non-recursive tail calls for LLVM optimization
- Prevents stack overflow in deeply recursive functions

**Example transformation**:
```llvm
; Before TCO (recursive factorial)
define i32 @factorial(i32 %n, i32 %acc) {
  %cond = icmp eq i32 %n, 0
  br i1 %cond, label %base, label %recursive

base:
  ret i32 %acc

recursive:
  %n_minus_1 = sub i32 %n, 1
  %new_acc = mul i32 %acc, %n
  %result = call i32 @factorial(i32 %n_minus_1, i32 %new_acc)
  ret i32 %result
}

; After TCO (converted to loop)
define i32 @factorial(i32 %n, i32 %acc) {
entry:
  br label %loop

loop:
  %n_phi = phi i32 [%n, %entry], [%n_minus_1, %loop]
  %acc_phi = phi i32 [%acc, %entry], [%new_acc, %loop]
  %cond = icmp eq i32 %n_phi, 0
  br i1 %cond, label %exit, label %continue

continue:
  %n_minus_1 = sub i32 %n_phi, 1
  %new_acc = mul i32 %acc_phi, %n_phi
  br label %loop

exit:
  ret i32 %acc_phi
}
```

**Implementation**: `src/codegen/llvm/passes/tail_call.rs`

### 7. Jump Threading

**Purpose**: Eliminates redundant conditional branches by threading jumps through intermediate blocks.

**What it does**:
- Analyzes conditions that can be determined along specific paths
- Creates direct jumps when branch outcomes are known
- Eliminates unnecessary intermediate basic blocks
- Simplifies control flow graphs

**Example transformation**:
```llvm
; Before Jump Threading
block1:
  %cond = icmp eq i32 %x, 5
  br i1 %cond, label %block2, label %block3

block2:
  ; %x is known to be 5 here
  %cond2 = icmp sgt i32 %x, 0  ; Always true since %x = 5
  br i1 %cond2, label %target_true, label %target_false

; After Jump Threading
block1:
  %cond = icmp eq i32 %x, 5
  br i1 %cond, label %target_true, label %block3  ; Direct jump

; block2 eliminated or simplified
```

**Implementation**: `src/codegen/llvm/passes/jump_threading.rs`

## Pass Pipeline Integration

The optimization passes are integrated into the CURSED compiler's optimization pipeline with careful ordering to maximize effectiveness:

### O1 (Basic Optimization)
1. Memory promotion (Mem2Reg, SROA)
2. Basic cleanup (dead code elimination, constant propagation)

### O2 (Default Optimization)
1. Memory promotion (Mem2Reg, SROA)
2. Early optimization (SCCP, dead code elimination, GVN)
3. Loop optimization (LICM, loop unrolling)
4. Function optimization (inlining, tail call, jump threading)

### O3 (Aggressive Optimization)
1. Memory promotion (Mem2Reg, SROA)
2. Early optimization (SCCP, dead code elimination, GVN)
3. Aggressive loop optimization (LICM, loop unrolling, SCCP again)
4. Aggressive function optimization (inlining, tail call, GVN again)
5. Final optimization (jump threading, SCCP, final cleanup)

## Performance Characteristics

| Pass | Typical Runtime | Memory Usage | Optimization Impact |
|------|----------------|--------------|-------------------|
| Mem2Reg | 100-200ms | Low | High (enables other passes) |
| SROA | 200-300ms | Medium | High (memory to register) |
| SCCP | 250-400ms | Medium | High (constant propagation) |
| GVN | 400-600ms | High | High (redundancy elimination) |
| LICM | 300-500ms | Medium | Medium-High (loop optimization) |
| Tail Call | 200-350ms | Low | Medium (recursive functions) |
| Jump Threading | 300-450ms | Medium | Medium (control flow) |

## Testing and Validation

### Running Tests
```bash
# Run optimization pass integration tests
make test-optimization-passes

# Run with verbose output
make test-optimization-passes-verbose

# Run performance benchmarks
make bench-optimization-passes
```

### Test Coverage
- **Unit tests**: Each pass has comprehensive unit tests
- **Integration tests**: Full pipeline testing with real LLVM IR
- **Performance tests**: Benchmarks for scalability validation
- **Correctness tests**: Verification that optimizations preserve semantics

### Benchmark Results
The optimization passes are designed to meet these performance targets:
- **Individual pass runtime**: < 500ms for typical functions
- **Pipeline runtime**: < 2s for complete O3 optimization
- **Memory usage**: < 100MB peak for large functions
- **Optimization effectiveness**: 10-30% improvement in generated code

## Future Enhancements

Planned improvements to the optimization pass infrastructure:

1. **Interprocedural Analysis**: Cross-function optimization
2. **Profile-Guided Optimization**: Runtime data-driven optimization
3. **Vectorization**: Auto-vectorization for SIMD operations
4. **Alias Analysis**: More precise memory dependence analysis
5. **Polyhedral Optimization**: Advanced loop nest optimization

## Debugging and Profiling

### Pass-Level Debugging
```bash
# Enable tracing for specific passes
RUST_LOG=cursed::codegen::llvm::passes=debug make test-optimization-passes

# Generate optimization reports
CURSED_OPTIMIZATION_REPORT=1 make build
```

### Performance Profiling
```bash
# Profile pass execution time
make bench-optimization-passes

# Generate detailed performance report
cargo bench --bench optimization_passes_bench -- --save-baseline current
```

### Pass Interaction Analysis
The passes are designed with careful dependency management:
- Mem2Reg enables most other passes by promoting memory to registers
- SROA works best after Mem2Reg to expose scalar replacement opportunities
- SCCP benefits from multiple runs as other passes expose new constants
- GVN should run after other passes to clean up redundancies they create

This optimization infrastructure provides the foundation for high-performance code generation in the CURSED compiler while maintaining correctness and reasonable compilation times.
