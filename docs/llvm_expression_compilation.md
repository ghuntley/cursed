# LLVM Expression Compilation for CURSED Language

## Overview

The LLVM expression compilation system provides comprehensive support for compiling all AST expression types to efficient LLVM IR. This system handles the Gen Z slang syntax while generating standard LLVM operations, ensuring both language authenticity and runtime performance.

## Architecture

### Core Components

1. **LlvmExpressionCompiler**: Main compilation engine
2. **LlvmType**: Type system mapping to LLVM types
3. **LlvmValue**: Runtime value representation
4. **ExpressionContext**: Compilation state management

### Expression Types Supported

#### Literal Expressions
- **Integers**: Compiled to `i64` LLVM type
- **Floats**: Compiled to `double` LLVM type  
- **Strings**: Compiled to `i8*` with proper escaping
- **Booleans**: Gen Z slang (`based`/`cap`) compiled to `i1`
- **Nil**: Compiled to null pointer (`i8*`)
- **Characters**: Compiled to `i8` then promoted to `i32`

#### Binary Expressions

**Arithmetic Operators**:
- `+`, `-`, `*`, `/`, `%` - Support both integer and floating-point operations
- Automatic type promotion (int + float = float)
- Proper LLVM instruction selection (`add`/`fadd`, `sub`/`fsub`, etc.)

**Comparison Operators**:
- `==`, `!=`, `<`, `>`, `<=`, `>=` - Generate appropriate `icmp`/`fcmp` instructions
- Always return `i1` (boolean) type
- Support for both signed integer and floating-point comparisons

**Logical Operators**:
- `&&`/`and`, `||`/`or` - Gen Z slang support with LLVM `and`/`or` instructions
- Short-circuit evaluation potential (future optimization)

**Bitwise Operators**:
- `&`, `|`, `^`, `<<`, `>>` - Integer-only operations
- Proper signed/unsigned handling for shifts

#### Unary Expressions

- **Negation** (`-`): `sub` from zero for integers, `fsub` for floats
- **Logical NOT** (`!`/`not`): `xor` with true for booleans
- **Bitwise NOT** (`~`): `xor` with -1 for integers

#### Variable Operations

**Access**: Load operations with proper type tracking
**Assignment**: Store operations with context updates
**Scoping**: Context-based variable resolution

#### Function Calls

- Dynamic function resolution
- Argument type checking and marshaling
- Return type inference
- Proper calling convention handling

#### Advanced Features

- **Type Assertions**: Runtime type checking support
- **Index Access**: Array/slice access with bounds checking potential
- **Parenthesized Expressions**: Transparent compilation
- **Error Propagation**: `?` operator support

## Type System

### LLVM Type Mapping

| CURSED Type | LLVM Type | Description |
|-------------|-----------|-------------|
| `int` | `i64` | 64-bit signed integer |
| `float` | `double` | 64-bit floating point |
| `bool` | `i1` | Single bit boolean |
| `string` | `i8*` | Pointer to UTF-8 string |
| `nil` | `i8*` | Null pointer |
| `char` | `i32` | Unicode codepoint (promoted) |

### Type Coercion Rules

1. **Integer + Float** → Float (promotion)
2. **Comparison operations** → Boolean (always)
3. **Logical operations** → Boolean (input must be boolean)
4. **Bitwise operations** → Integer (integer operands only)

## Code Generation

### IR Generation Patterns

#### Literals
```llvm
; Integer literal: 42
%temp_1 = add i64 0, 42

; String literal: "hello"
@.str_1 = private unnamed_addr constant [6 x i8] c"hello\00", align 1
%temp_2 = getelementptr inbounds [6 x i8], [6 x i8]* @.str_1, i64 0, i64 0
```

#### Binary Operations
```llvm
; Addition: a + b
%temp_1 = add i64 0, 10    ; a = 10
%temp_2 = add i64 0, 20    ; b = 20  
%temp_3 = add i64 %temp_1, %temp_2

; Comparison: a > b
%temp_4 = icmp sgt i64 %temp_1, %temp_2
```

#### Variable Operations
```llvm
; Assignment: x = 42
%temp_1 = add i64 0, 42
store i64 %temp_1, i64* @x

; Access: load x
%temp_2 = load i64, i64* @x
```

### Optimization Opportunities

1. **Constant Folding**: Compile-time evaluation of constant expressions
2. **Dead Code Elimination**: Remove unused temporaries
3. **Common Subexpression Elimination**: Reuse computed values
4. **Type-specific Optimizations**: Use optimal instructions per type

## Gen Z Slang Support

### Syntax Mapping

| Gen Z Slang | Standard | LLVM Operation |
|-------------|----------|----------------|
| `based` | `true` | `i1 1` |
| `cap` | `false` | `i1 0` |
| `and` | `&&` | `and i1` |
| `or` | `\|\|` | `or i1` |
| `not` | `!` | `xor i1 ..., true` |

### Implementation Notes

The compiler recognizes both traditional and slang operators, maintaining full compatibility while providing authentic Gen Z programming experience.

## Error Handling

### Compile-Time Errors

1. **Type Mismatches**: Incompatible operand types
2. **Undefined Variables**: Variable access before declaration
3. **Invalid Operations**: Unsupported operator/type combinations
4. **Syntax Errors**: Malformed expressions

### Error Messages

Errors include:
- Source location information (when available)
- Type information for operands
- Suggested corrections
- Context about the failing operation

## Testing Strategy

### Why Comprehensive Testing is Critical

Expression compilation testing is essential because:

1. **Operator Precedence**: Ensures mathematical expressions evaluate correctly
2. **Type Safety**: Prevents runtime type errors and crashes
3. **Performance**: Verifies efficient LLVM IR generation
4. **Compatibility**: Ensures Gen Z slang works alongside standard syntax
5. **Correctness**: Validates that compiled code produces expected results

### Test Categories

#### Unit Tests
- Individual expression type compilation
- Type resolution and coercion
- Error condition handling
- IR quality validation

#### Integration Tests  
- Complete compilation pipeline
- Context persistence
- Debug information integration
- Performance characteristics

#### Stress Tests
- Large expression trees
- Deep nesting scenarios
- Memory usage validation
- Compilation time benchmarks

## Performance Characteristics

### Compilation Speed
- Linear complexity for expression depth
- Efficient temporary variable management
- Minimal memory allocation per expression

### Generated Code Quality
- SSA form output for optimization readiness
- Type-specific instruction selection
- Minimal runtime overhead
- Standards-compliant LLVM IR

## Future Enhancements

### Planned Features

1. **Advanced Optimizations**:
   - Constant folding at compile time
   - Dead code elimination
   - Common subexpression elimination

2. **Enhanced Type System**:
   - Generic type support
   - Union types for flexible programming
   - Interface type assertions

3. **Debug Information**:
   - Source location mapping
   - Variable lifetime tracking
   - Expression evaluation debugging

4. **Performance Improvements**:
   - Vectorization hints
   - Profile-guided optimization hooks
   - Architecture-specific optimizations

### Research Areas

- **JIT Compilation**: Runtime expression evaluation
- **Partial Evaluation**: Compile-time specialization
- **Automatic Parallelization**: SIMD optimization
- **Memory Layout**: Cache-friendly data structures

## Usage Examples

### Basic Arithmetic
```cursed
sus result = 10 + 20 * 3  // Generates proper precedence IR
```

### Gen Z Boolean Logic
```cursed
facts condition = based and not cap  // Compiles to: true && !false
```

### Mixed Type Operations
```cursed
sus mixed = 42 + 3.14  // Automatic int->float promotion
```

### Complex Expressions
```cursed
sus complex = ((a + b) * c) > (d && e)  // Proper nesting and types
```

## Integration with CURSED Compiler

The expression compiler integrates seamlessly with:

- **Parser**: Consumes AST expression nodes
- **Type Checker**: Provides type resolution services  
- **Optimizer**: Produces optimization-ready IR
- **Code Generator**: Embedded in main compilation pipeline
- **Debug System**: Provides source location tracking
- **Error Reporter**: Delivers comprehensive error information

This expression compilation system forms the foundation for efficient, type-safe, and performant CURSED language execution while maintaining the authentic Gen Z programming experience.
