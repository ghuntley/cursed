# CURSED Type Conversion Specification

This document serves as the comprehensive implementation blueprint for type conversions in the CURSED programming language.

## Overview

CURSED requires explicit type conversions between different types, following the principle of explicit over implicit conversion. The language provides a C-style casting syntax for conversions between compatible types.

## Basic Type System

### Primitive Types and Zero Values

| Type Category | CURSED Type | Size | Zero Value | Description |
|---------------|-------------|------|------------|-------------|
| Boolean       | `lit`       | 1 bit | `sus` (false) | Boolean values |
| Integer       | `smol`      | 8-bit | `0` | Signed 8-bit integer |
|               | `mid`       | 16-bit | `0` | Signed 16-bit integer |
|               | `normie`    | 32-bit | `0` | Signed 32-bit integer |
|               | `thicc`     | 64-bit | `0` | Signed 64-bit integer |
|               | `byte`      | 8-bit | `0` | Unsigned 8-bit (alias for uint8) |
|               | `rune`      | 32-bit | `0` | Unicode code point (alias for int32) |
| Floating      | `snack`     | 32-bit | `0.0` | 32-bit floating point |
|               | `meal`      | 64-bit | `0.0` | 64-bit floating point |
| String        | `tea`       | variable | `""` | UTF-8 string |
| Character     | `sip`       | 32-bit | `\0` | Single Unicode character (rune) |
| Complex       | `extra`     | 128-bit | `0+0i` | Complex number |

### Reference Types and Zero Values

| Type Category | CURSED Type | Zero Value | Description |
|---------------|-------------|------------|-------------|
| Pointer       | `@T`        | `cap` | Pointer to type T |
| Array         | `[n]T`      | All elements zero | Fixed-size array |
| Slice         | `[]T`       | `cap` | Dynamic array |
| Map           | `tea[K]V`   | `cap` | Hash map |
| Channel       | `dm<T>`     | `cap` | Communication channel |
| Interface     | `collab`    | `cap` | Interface value |
| Function      | `slay`      | `cap` | Function pointer |

## Type Conversion Syntax

### Explicit Type Conversion

CURSED uses function-call syntax for type conversions:

```cursed
sus x normie = 42
sus y snack = snack(x)  // Convert int to float
sus z tea = tea(x)      // Convert int to string
```

### General Conversion Syntax

```cursed
target_type(source_expression)
```

## Numeric Type Conversions

### Integer to Integer Conversions

```cursed
// Conversion between integer types
sus small smol = 100
sus medium mid = mid(small)    // Widening conversion (safe)
sus large thicc = thicc(medium)   // Widening conversion (safe)
sus back normie = normie(large)   // Narrowing conversion (may truncate)

// Conversion with potential overflow
sus big thicc = 300
sus tiny smol = smol(big)      // Results in 44 (300 % 256)
```

**Behavior:**
- **Widening conversions** (smaller to larger): Always safe, value preserved
- **Narrowing conversions** (larger to smaller): May truncate, uses modulo arithmetic
- **Signed/unsigned conversions**: Reinterpret bit pattern

### Integer to Floating Point Conversions

```cursed
sus integer normie = 42
sus float32 snack = snack(integer)   // 42.0
sus float64 meal = meal(integer)     // 42.0

// Large integers may lose precision in floats
sus large thicc = 9223372036854775807  // max int64
sus imprecise snack = snack(large)     // Precision loss
```

**Behavior:**
- Always succeeds
- May lose precision for very large integers
- Exact representation for small integers

### Floating Point to Integer Conversions

```cursed
sus pi meal = 3.14159
sus truncated normie = normie(pi)    // Results in 3 (truncation)
sus negative meal = -2.8
sus neg_int normie = normie(negative) // Results in -2
```

**Behavior:**
- **Truncation**: Decimal part is discarded (not rounded)
- **Overflow**: Results in undefined behavior for values outside integer range
- **NaN/Infinity**: Results in 0 or implementation-defined value

### Floating Point to Floating Point Conversions

```cursed
sus double meal = 3.14159265359
sus single snack = snack(double)     // May lose precision
sus back meal = meal(single)         // Precision cannot be recovered
```

**Behavior:**
- `meal` to `snack`: May lose precision
- `snack` to `meal`: Extends precision with exact representation

## String Conversions

### Numeric to String Conversions

```cursed
// Integer to string
sus num normie = 42
sus str tea = tea(num)               // "42"

// Float to string  
sus pi meal = 3.14159
sus pi_str tea = tea(pi)             // "3.14159" (implementation-defined format)

// Boolean to string
sus flag lit = based
sus flag_str tea = tea(flag)         // "based"
```

### String to Numeric Conversions

```cursed
// String to integer (requires stdlib parsing functions)
sus num_str tea = "42"
sus num normie = normie(num_str)     // 42 (if valid), 0 (if invalid)

// String to float
sus pi_str tea = "3.14159"
sus pi meal = meal(pi_str)           // 3.14159 (if valid), 0.0 (if invalid)
```

**Error Handling:**
- Invalid strings convert to zero value
- Leading/trailing whitespace handling is implementation-defined
- Overflow results in maximum/minimum value for type

## Character Conversions

### Character to Integer Conversions

```cursed
sus letter sip = 'A'
sus code normie = normie(letter)     // 65 (ASCII/Unicode code point)
sus as_rune rune = rune(letter)      // 65 (explicit rune conversion)
```

### Integer to Character Conversions

```cursed
sus code normie = 65
sus letter sip = sip(code)           // 'A'
sus unicode normie = 8364            // Euro symbol code point
sus euro sip = sip(unicode)          // '€'
```

### Character to String Conversions

```cursed
sus letter sip = 'A'
sus str tea = tea(letter)            // "A"

// Character operations via stdlib
sus upper sip = letter.to_uppercase()
sus lower sip = letter.to_lowercase()
sus is_upper lit = letter.is_uppercase()
```

## Boolean Conversions

### Boolean to Integer/Float Conversions

```cursed
sus flag lit = based
sus as_int normie = normie(flag)     // 1
sus as_float snack = snack(flag)     // 1.0

sus false_flag lit = sus
sus zero normie = normie(false_flag) // 0
```

### Integer/Float to Boolean Conversions

```cursed
sus num normie = 42
sus flag lit = lit(num)              // based (non-zero is true)

sus zero normie = 0  
sus false_flag lit = lit(zero)       // sus (zero is false)

sus pi meal = 3.14
sus true_flag lit = lit(pi)          // based (non-zero is true)
```

**Behavior:**
- `0` converts to `sus` (false)
- Any non-zero value converts to `based` (true)
- `NaN` converts to `sus` (false)

## Pointer Conversions

### Nil Pointer Representation

```cursed
// Nil pointers use the 'cap' keyword
sus ptr @normie = cap               // Nil pointer
sus slice []normie = cap            // Nil slice  
sus map_val tea[normie]normie = cap // Nil map
sus chan dm<normie> = cap           // Nil channel
```

### Pointer Type Conversions

```cursed
// Conversion between compatible pointer types
sus int_ptr @normie = @some_int
sus void_ptr @byte = @byte(int_ptr)  // Unsafe conversion
sus back @normie = @normie(void_ptr) // Unsafe conversion back
```

**Safety:**
- Most pointer conversions are unsafe
- Converting through `@byte` (void pointer equivalent) loses type information
- Dereferencing incorrectly typed pointers causes undefined behavior

## Array and Slice Conversions

### Array to Slice Conversions

```cursed
sus arr [5]normie = [5]normie{1, 2, 3, 4, 5}
sus slice []normie = arr[:]          // Convert array to slice
```

### Slice Type Conversions

```cursed
// Converting between compatible slice types
sus bytes []byte = "hello"           // String to byte slice
sus str tea = tea(bytes)             // Byte slice to string

// Unsafe conversions between numeric slice types
sus ints []normie = []normie{1, 2, 3, 4}
sus bytes []byte = []byte(ints)      // Reinterpret as bytes (unsafe)
```

## Interface Conversions

### Type Assertions

```cursed
// Interface to concrete type conversion
sus val interface{} = 42
sus num, ok = val.(normie)           // Type assertion with success check

// Panic-causing type assertion
sus definite_num = val.(normie)      // Panics if not normie

// Type assertion with error propagation
sus safe_num = val.(normie)?         // Uses ? operator for error handling
```

### Interface Assignment

```cursed
// Any concrete type can be assigned to interface{}
sus num normie = 42
sus val interface{} = num            // Automatic conversion

// Specific interface implementations
collab Stringer {
    toString() tea
}

squad Person {
    name tea
}

slay (p Person) toString() tea {
    yolo p.name
}

sus person = Person{name: "Alice"}
sus stringer Stringer = person       // Automatic conversion to interface
```

## Complex Type Conversions

### Complex Number Conversions

```cursed
// Real number to complex
sus real meal = 3.14
sus complex extra = extra(real)      // 3.14+0i

// Complex to real (loses imaginary part)
sus comp extra = 3.14 + 2.71i
sus real_part meal = meal(comp)      // 3.14 (real part only)
```

## Error Handling in Conversions

### Overflow and Underflow

```cursed
// Integer overflow
sus big thicc = 9223372036854775807
sus small smol = smol(big)           // Undefined behavior (implementation-defined)

// Float overflow
sus huge meal = 1.8e308
sus single snack = snack(huge)       // May result in +Inf
```

### Invalid String Conversions

```cursed
// Invalid string to number conversions
sus invalid tea = "not_a_number"
sus num normie = normie(invalid)     // Results in 0
sus float_num meal = meal(invalid)   // Results in 0.0
```

### Runtime Conversion Errors

For conversions that may fail at runtime, CURSED provides two approaches:

1. **Panic-causing conversions**: Fail with runtime panic
2. **Safe conversions**: Return success flag or use error propagation

```cursed
// Panic-causing interface assertion
sus num = interface_val.(normie)     // Panics if not normie

// Safe interface assertion
sus num, ok = interface_val.(normie) // Returns (value, success)
lowkey !ok {
    // Handle conversion failure
}

// Error propagation
sus num = interface_val.(normie)?    // Propagates error if conversion fails
```

## Implementation Requirements

### Compiler Implementation

1. **Type Checking**: Ensure source and target types are compatible
2. **Code Generation**: Generate appropriate conversion instructions
3. **Error Reporting**: Provide clear error messages for invalid conversions
4. **Optimization**: Eliminate no-op conversions

### Runtime Implementation

1. **Overflow Handling**: Define behavior for numeric overflows
2. **Error Propagation**: Support `?` operator for safe conversions
3. **Memory Safety**: Ensure pointer conversions don't violate memory safety
4. **Performance**: Optimize common conversion patterns

### Standard Library Integration

The following stdlib functions support type conversions:

```cursed
// vibez package (formatted I/O)
vibez.spillf("%d", value)            // Format any type to string

// stringz package (string utilities)  
stringz.parse_int(str)               // Safe string to int conversion
stringz.parse_float(str)             // Safe string to float conversion
stringz.format_int(num)              // Int to string conversion
stringz.format_float(num)            // Float to string conversion

// mathz package (math utilities)
mathz.int(float_val)                 // Float to int conversion with rounding
mathz.float(int_val)                 // Int to float conversion

// reflectz package (reflection)
reflectz.type_name(value)            // Get type name as string
reflectz.convert(value, target_type) // Dynamic type conversion
```

## Conversion Compatibility Matrix

| From/To    | smol | mid | normie | thicc | snack | meal | lit | tea | sip | @T | []T | interface{} |
|------------|------|-----|--------|-------|-------|------|-----|-----|-----|----|----|-------------|
| smol       | ✓    | ✓   | ✓      | ✓     | ✓     | ✓    | ✓   | ✓   | ✓   | ✗  | ✗  | ✓           |
| mid        | ⚠    | ✓   | ✓      | ✓     | ✓     | ✓    | ✓   | ✓   | ✓   | ✗  | ✗  | ✓           |
| normie     | ⚠    | ⚠   | ✓      | ✓     | ✓     | ✓    | ✓   | ✓   | ✓   | ✗  | ✗  | ✓           |
| thicc      | ⚠    | ⚠   | ⚠      | ✓     | ⚠     | ✓    | ✓   | ✓   | ✓   | ✗  | ✗  | ✓           |
| snack      | ⚠    | ⚠   | ⚠      | ⚠     | ✓     | ✓    | ✓   | ✓   | ⚠   | ✗  | ✗  | ✓           |
| meal       | ⚠    | ⚠   | ⚠      | ⚠     | ⚠     | ✓    | ✓   | ✓   | ⚠   | ✗  | ✗  | ✓           |
| lit        | ✓    | ✓   | ✓      | ✓     | ✓     | ✓    | ✓   | ✓   | ✗   | ✗  | ✗  | ✓           |
| tea        | ⚠    | ⚠   | ⚠      | ⚠     | ⚠     | ⚠    | ⚠   | ✓   | ⚠   | ✗  | ✓* | ✓           |
| sip        | ✓    | ✓   | ✓      | ✓     | ✓     | ✓    | ✗   | ✓   | ✓   | ✗  | ✗  | ✓           |
| @T         | ✗    | ✗   | ✗      | ✗     | ✗     | ✗    | ✗   | ✗   | ✗   | ⚠  | ✗  | ✓           |
| []T        | ✗    | ✗   | ✗      | ✗     | ✗     | ✗    | ✗   | ✓* | ✗   | ✗  | ⚠  | ✓           |
| interface{}| ⚠    | ⚠   | ⚠      | ⚠     | ⚠     | ⚠    | ⚠   | ⚠   | ⚠   | ⚠  | ⚠  | ✓           |

**Legend:**
- ✓ = Always safe conversion
- ⚠ = May lose precision, overflow, or fail at runtime
- ✗ = Not allowed / compilation error
- ✓* = Special case (e.g., string to []byte, []byte to string)

## Testing Requirements

### Unit Tests Required

1. **Numeric Conversions**:
   - All integer type combinations
   - Integer to floating point conversions  
   - Floating point to integer conversions
   - Overflow and underflow edge cases

2. **String Conversions**:
   - Valid numeric strings to numbers
   - Invalid strings to numbers (should yield zero)
   - Numbers to strings with correct formatting
   - Unicode character handling

3. **Character Conversions**:
   - Character to integer code points
   - Integer code points to characters
   - Unicode character support
   - Character to string conversions

4. **Boolean Conversions**:
   - Zero and non-zero values to boolean
   - Boolean to numeric conversions

5. **Pointer Conversions**:
   - Nil pointer handling
   - Type-safe and unsafe pointer conversions

6. **Interface Conversions**:
   - Type assertions with success and failure cases
   - Error propagation with `?` operator
   - Multiple interface implementations

### Integration Tests Required

1. **Stdlib Integration**: Verify conversions work with standard library functions
2. **LLVM Code Generation**: Test compiled conversion code produces correct results
3. **Error Handling**: Verify proper error propagation and panic behavior
4. **Performance**: Benchmark conversion performance for optimization

## Implementation Phases

### Phase 1: Core Numeric Conversions
- Basic integer-to-integer conversions
- Integer-to-float and float-to-integer conversions
- Boolean conversions
- Parser support for conversion syntax

### Phase 2: String and Character Conversions  
- String to numeric conversions (with error handling)
- Numeric to string conversions
- Character operations and conversions
- Unicode support

### Phase 3: Advanced Conversions
- Interface type assertions
- Pointer conversions (unsafe)
- Complex type conversions
- Error propagation with `?` operator

### Phase 4: Optimization and Testing
- Conversion optimization in LLVM backend
- Comprehensive test suite
- Performance benchmarking
- Documentation and examples

This specification provides the complete blueprint for implementing type conversions in CURSED, ensuring consistency with the language's explicit conversion philosophy while maintaining Go-like semantics where appropriate.
