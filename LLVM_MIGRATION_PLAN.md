# CURSED LLVM Migration Plan: String-based IR to Inkwell API

## Current State Analysis

### String-based IR Generation Areas Identified:
1. **Main LLVM Code Generator** (`src/codegen/llvm/main.rs`)
   - Uses `ir_code: String` field and `push_str`/`format!` for IR generation
   - Manual register numbering with `variable_counter`
   - String-based function, statement, and expression generation

2. **Expression Compiler** (`src/codegen/llvm/expression_compiler.rs`) 
   - Uses `ir_buffer: String` for IR output
   - String-based binary/unary operations
   - Manual register management

3. **Function Compilation** (`src/codegen/llvm/function_compilation.rs`)
   - String-based function IR generation
   - Manual parameter handling

### Inkwell Usage Examples Found:
- `enhanced_codegen.rs.full` shows proper inkwell usage patterns
- Several optimization passes use inkwell correctly
- Error handling patterns for inkwell operations

## Migration Strategy

### Phase 1: Create Inkwell-based Foundation
1. Create new inkwell-based code generator structure
2. Implement proper LLVM context, module, and builder management
3. Add type conversion utilities (CURSED types → LLVM types)

### Phase 2: Migrate Core Expression Compilation
1. Replace string-based arithmetic operations with inkwell builder calls
2. Migrate literal value generation
3. Convert variable access and storage operations
4. Handle function calls with proper typing

### Phase 3: Migrate Statement Generation
1. Convert control flow (if, while, for) to proper basic blocks
2. Migrate variable declarations and assignments
3. Handle function definitions with proper signatures

### Phase 4: Integration and Testing
1. Update main code generator to use inkwell backend
2. Ensure all existing tests pass
3. Performance comparison and optimization

## Implementation Plan

### Step 1: Create Inkwell-based Expression Compiler
- Replace `ExpressionCompiler` with proper inkwell usage
- Use `Builder` for instruction generation instead of string formatting
- Implement proper register/value management through LLVM values

### Step 2: Create Inkwell-based Statement Generator  
- Replace string-based statement generation
- Use proper basic block creation for control flow
- Implement function generation with inkwell

### Step 3: Refactor Main Generator
- Replace `LlvmCodeGenerator` string fields with inkwell structures
- Update API to return LLVM Module instead of string IR
- Maintain backward compatibility where needed

## Benefits of Migration

1. **Type Safety**: Inkwell provides compile-time type checking for LLVM operations
2. **Performance**: Direct LLVM API calls are more efficient than string parsing
3. **Maintainability**: Structured IR generation instead of string concatenation
4. **Register Management**: LLVM handles register numbering automatically
5. **Error Handling**: Better error reporting from LLVM operations
6. **Optimization**: Access to LLVM's built-in optimization passes

## Risk Mitigation

1. **Incremental Migration**: Migrate one component at a time
2. **Test Coverage**: Ensure all existing functionality is preserved
3. **Backward Compatibility**: Keep string-based output for testing
4. **Fallback Option**: Maintain ability to revert if needed

## Key Findings and Challenges

### Issues Discovered:
1. **Lifetime Management**: Circular dependencies between expression compiler and statement generator cause borrowing issues
2. **API Differences**: Inkwell 0.4 has different API than expected:
   - `build_load` requires type parameter first
   - `get_element_type()` doesn't exist on PointerType
   - `TargetTriple` creation requires different approach
   - `OptimizationLevel::O2` doesn't exist (use `OptimizationLevel::None`)

### Solutions Implemented:
1. **Build Load Fix**: Use `build_load(element_type, pointer, name)` instead of `build_load(pointer, name)`
2. **Element Type Access**: Need to track types separately or use different approach
3. **Separate Builders**: Create separate builder instances to avoid borrowing conflicts
4. **Simplified Architecture**: Avoid circular dependencies between components

## Migration Status

### Phase 1: Foundation ✅ (Partial)
- [x] Created inkwell-based expression compiler structure
- [x] Created inkwell-based statement generator structure  
- [x] Created main inkwell code generator
- [x] Added proper imports and module declarations
- [ ] Fix lifetime and API compatibility issues

### Phase 2: Core Expression Compilation 🚧 (In Progress)
- [x] Basic literal compilation (integers, floats, booleans, strings)
- [x] Binary operations with type promotion
- [x] Unary operations  
- [x] Variable access and storage operations
- [ ] Fix `get_element_type()` API issue
- [ ] Fix function calls with proper typing
- [ ] Handle increment/decrement expressions

### Phase 3: Statement Generation 🚧 (In Progress)
- [x] Let statements and variable declarations
- [x] Assignment statements
- [x] Short declarations (:= operator)
- [x] Return statements
- [x] If statements with proper basic blocks
- [x] While loops with proper loop structure
- [x] For loops (C-style)
- [x] Break/continue statements
- [ ] Fix AST field name mismatches
- [ ] Fix lifetime issues between components

### Phase 4: Integration and Testing ❌ (Not Started)
- [ ] Update main code generator to use inkwell backend  
- [ ] Ensure all existing tests pass
- [ ] Performance comparison and optimization
- [ ] Backward compatibility with string-based API

## Success Criteria

- [ ] All existing CURSED tests pass with inkwell backend
- [ ] Generated IR quality is equivalent or better
- [ ] Compilation performance improves  
- [ ] Code maintainability increases
- [ ] Memory usage is optimized

## Next Steps

1. **Fix API Compatibility**: Address inkwell 0.4 API differences
2. **Resolve Lifetime Issues**: Redesign component relationships to avoid circular dependencies
3. **Element Type Tracking**: Implement proper type tracking for pointer operations
4. **Integration Testing**: Create simple test cases to verify functionality
5. **Performance Benchmarking**: Compare string-based vs inkwell-based approaches
