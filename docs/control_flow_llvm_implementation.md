# LLVM Control Flow Implementation for CURSED

## Overview

This document describes the implementation of LLVM code generation for control flow constructs in the CURSED programming language. The implementation handles the compilation of Gen Z slang keywords into standard LLVM IR while preserving semantic correctness.

## Implemented Control Flow Constructs

### 1. If/Else Statements (`lowkey`/`highkey`)

**CURSED Syntax:**
```cursed
lowkey condition {
    // then branch
} highkey {
    // else branch
}
```

**LLVM IR Generation:**
- Creates three basic blocks: `lowkey_then`, `highkey_else`, `if_merge`
- Compiles condition expression and converts to i1 boolean
- Uses `br i1` for conditional branching
- Merges control flow in the merge block

**Key Features:**
- Proper condition type handling (converts integers to boolean)
- Handles optional else clause
- Prevents double termination of basic blocks

### 2. While Loops (`periodt`)

**CURSED Syntax:**
```cursed
periodt condition {
    // loop body
}
```

**LLVM IR Generation:**
- Creates three basic blocks: `periodt_condition`, `periodt_body`, `periodt_exit`
- Sets up loop context for break/continue handling
- Uses phi nodes implicitly through LLVM's SSA form
- Proper branch structure for loop entry and continuation

**Key Features:**
- Loop context management for nested constructs
- Break/continue target tracking
- Condition re-evaluation on each iteration

### 3. For Loops (`bestie`)

**CURSED Syntax:**
```cursed
bestie init; condition; post {
    // loop body
}
```

**LLVM IR Generation:**
- Creates four basic blocks: `bestie_condition`, `bestie_body`, `bestie_post`, `bestie_exit`
- Handles optional initialization, condition, and post statements
- Supports infinite loops (no condition)
- Variable scoping for loop-local variables

**Key Features:**
- Proper execution order: init → condition → body → post → condition
- Continue jumps to post block, break jumps to exit
- Scope management for loop variables

### 4. Range-Based For Loops (`bestie` ... `flex`)

**CURSED Syntax:**
```cursed
bestie key, value := flex iterable {
    // loop body
}
```

**LLVM IR Generation:**
- Creates five basic blocks: `flex_setup`, `flex_condition`, `flex_body`, `flex_post`, `flex_exit`
- Implements iterator protocol (simplified for integers currently)
- Manages loop variables in separate scope
- Index-based iteration with bounds checking

**Key Features:**
- Iterator setup and cleanup
- Variable binding for key/value pairs
- Automatic index management

### 5. Switch Statements (`vibe_check`)

**CURSED Syntax:**
```cursed
vibe_check value {
    mood case1, case2:
        // statements
    mood case3:
        // statements
    basic:
        // default case
}
```

**LLVM IR Generation:**
- Uses LLVM's `switch` instruction
- Creates separate basic blocks for each case
- Handles multiple values per case
- Implements default case (`basic`)

**Key Features:**
- Efficient jump table generation
- Multiple case values support
- Fallthrough behavior (simplified)

### 6. Break/Continue Statements (`ghosted`/`simp`)

**CURSED Syntax:**
```cursed
ghosted    // break
simp       // continue
```

**LLVM IR Generation:**
- Uses unconditional branches to appropriate targets
- Validates context (must be inside loop)
- Proper basic block termination

**Key Features:**
- Context validation prevents usage outside loops
- Proper target resolution from loop stack
- Immediate termination of current basic block

## Implementation Architecture

### Control Flow Context

The `ControlFlowContext` manages compilation state:

```rust
pub struct ControlFlowContext {
    pub loop_stack: Vec<LoopContext>,
    pub current_function: Option<FunctionValue<'static>>,
    pub variable_scopes: Vec<HashMap<String, PointerValue<'static>>>,
}
```

**Features:**
- Loop stack for nested loop handling
- Variable scope management
- Function context tracking

### Loop Context

Each loop maintains its execution context:

```rust
pub struct LoopContext {
    pub continue_block: BasicBlock<'static>,
    pub break_block: BasicBlock<'static>,
    pub condition_block: Option<BasicBlock<'static>>,
    pub loop_type: String,
}
```

**Features:**
- Break/continue target blocks
- Loop type identification for debugging
- Condition block for complex loops

### Compilation Trait

The `ControlFlowCompilation` trait defines the interface:

```rust
pub trait ControlFlowCompilation<'ctx> {
    fn compile_if_statement(&self, ...) -> Result<(), Error>;
    fn compile_while_statement(&self, ...) -> Result<(), Error>;
    // ... other methods
}
```

## Key Implementation Details

### Basic Block Management

1. **Block Creation**: Each control flow construct creates appropriately named blocks
2. **Termination Checking**: Prevents adding instructions to already terminated blocks
3. **Merge Points**: Proper convergence of control flow paths

### Variable Scoping

1. **Scope Stack**: Maintains hierarchy of variable scopes
2. **Loop Variables**: Special handling for for-loop variables
3. **Lifetime Management**: Variables automatically cleaned up on scope exit

### Error Handling

1. **Context Validation**: Break/continue must be inside loops
2. **Type Checking**: Conditions must be boolean-convertible
3. **LLVM Validation**: Generated IR is verified for correctness

### Expression Integration

1. **Condition Compilation**: Handles various expression types in conditions
2. **Type Conversion**: Automatic conversion to boolean for conditions
3. **Fallback Values**: Reasonable defaults for unknown expressions

## Testing Strategy

### Unit Tests
- Individual control flow construct compilation
- Context management operations
- Error condition handling

### Integration Tests
- Complete function compilation with control flow
- Nested control flow structures
- Variable scoping validation

### Execution Tests
- JIT execution of compiled control flow
- Semantic correctness verification
- Performance validation

### Edge Case Tests
- Empty loops and conditions
- Break/continue in nested structures
- Invalid usage patterns

## Generated LLVM IR Examples

### Simple If Statement
```llvm
define i32 @main() {
entry:
  br i1 true, label %lowkey_then, label %highkey_else

lowkey_then:
  br label %if_merge

highkey_else:
  br label %if_merge

if_merge:
  ret i32 0
}
```

### While Loop with Break
```llvm
define i32 @main() {
entry:
  br label %periodt_condition

periodt_condition:
  br i1 true, label %periodt_body, label %periodt_exit

periodt_body:
  br label %periodt_exit  ; break statement

periodt_exit:
  ret i32 0
}
```

## Performance Considerations

1. **Optimal Branching**: Uses LLVM's efficient branching instructions
2. **Phi Node Elimination**: Relies on LLVM's SSA optimizations
3. **Dead Code Elimination**: Unreachable code after breaks is handled
4. **Loop Optimization**: Structure enables LLVM loop optimizations

## Future Enhancements

1. **Iterator Protocol**: Full implementation for range-based loops
2. **Loop Annotations**: LLVM metadata for optimization hints
3. **Exception Handling**: Integration with error propagation
4. **Debug Information**: Line number and scope debugging
5. **Optimization Passes**: Custom passes for CURSED-specific patterns

## Security Considerations

1. **Bounds Checking**: Range loops include bounds validation
2. **Stack Overflow Protection**: Prevents unbounded recursion
3. **Memory Safety**: Proper scope management prevents use-after-free
4. **Termination Guarantees**: Break/continue validation prevents infinite loops

## Integration with CURSED Ecosystem

1. **AST Compatibility**: Works with existing AST structures
2. **Error System Integration**: Uses CURSED error types
3. **Debug Support**: Compatible with debug information generation
4. **Type System**: Integrates with CURSED type checking

This implementation provides a solid foundation for control flow compilation while maintaining the semantic richness of CURSED's Gen Z slang syntax and ensuring the generated LLVM IR is efficient and correct.
