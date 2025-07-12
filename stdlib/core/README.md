# Core Module - Essential Language Primitives for CURSED Self-Hosting

The core module provides fundamental types, utilities, and primitives essential for CURSED programs and self-hosting compiler support. This module forms the foundation of the CURSED standard library with pure CURSED implementations without FFI dependencies.

## Features

- **Type Conversion Functions**: Comprehensive type conversion between all CURSED types
- **Option Type**: Nullable value handling with Option<T> pattern
- **Result Type**: Error handling with Result<T, E> pattern  
- **Memory Allocation Utilities**: Memory management abstractions
- **Panic and Error Handling**: Program termination and error reporting
- **Array and Slice Utilities**: Collection manipulation and bounds checking
- **String Utilities**: Comprehensive string processing functions
- **Mathematical Utilities**: Essential math operations
- **Boolean Utilities**: Logical operations and boolean algebra
- **Compiler Utilities**: Support functions for self-hosting compiler

## Type Conversion Functions

### Integer Conversions

```cursed
# Convert integer to boolean
sus flag lit = lit_from_int(42)     # based (true)
sus flag2 lit = lit_from_int(0)     # cap (false)

# Convert boolean to integer  
sus num normie = int_from_bool(based)  # 1
sus num2 normie = int_from_bool(cap)   # 0

# Convert string to integer
sus value normie = int_from_string("42")   # 42
sus value2 normie = int_from_string("-5")  # -5
```

### String Conversions

```cursed
# Convert integer to string
sus str tea = string_from_int(42)      # "42"
sus str2 tea = string_from_int(-123)   # "-123"

# Convert boolean to string
sus bool_str tea = string_from_bool(based)  # "true"
sus bool_str2 tea = string_from_bool(cap)   # "false"

# Convert float to string
sus float_str tea = string_from_float(3.14)  # "3.14"
```

### Float Conversions

```cursed
# Convert integer to float
sus float_val meal = float_from_int(42)      # 42.0

# Convert string to float
sus parsed_float meal = float_from_string("3.14")  # 3.14
```

### Legacy Compatibility

```cursed
# Original core functions (for backward compatibility)
sus flag lit = lit(42)              # lit_from_int(42)
sus num normie = normie(based)      # int_from_bool(based)
sus str tea = tea(42)               # string_from_int(42)
sus big_int thicc = thicc(42)       # Type cast to 64-bit
sus float_val meal = meal(42)       # float_from_int(42)
```

## Option Type Implementation

The Option type represents nullable values safely, preventing null pointer exceptions.

```cursed
# Create Option values
sus some_value (lit, normie) = option_some(42)
sus none_value (lit, normie) = option_none()

# Check Option type
bestie option_is_some(some_value) {
    vibez.spill("Has value!")
}

bestie option_is_none(none_value) {
    vibez.spill("No value")
}

# Unwrap Option values
sus value normie = option_unwrap(some_value)              # 42
sus safe_value normie = option_unwrap_or(none_value, 100) # 100
sus default_value normie = option_unwrap_or_else(none_value, 200) # 200
```

## Result Type Implementation

The Result type handles errors without exceptions, providing safe error propagation.

```cursed
# Create Result values
sus ok_result (lit, normie, normie) = result_ok(123)
sus err_result (lit, normie, normie) = result_err(404)

# Check Result type
bestie result_is_ok(ok_result) {
    vibez.spill("Success!")
}

bestie result_is_err(err_result) {
    vibez.spill("Error occurred")
}

# Unwrap Result values
sus value normie = result_unwrap(ok_result)              # 123
sus safe_value normie = result_unwrap_or(err_result, 999) # 999
sus error_code normie = result_get_error(err_result)     # 404
```

## Memory Allocation Utilities

Simplified memory management abstractions for compiler support.

```cursed
# Allocate memory
sus address normie = memory_allocate(1024)

# Memory operations
memory_copy(dest_addr, src_addr, size)
memory_set(address, value, size)
memory_deallocate(address)

# Memory comparison
sus cmp_result normie = memory_compare(addr1, addr2, size)
# Returns: 0 (equal), -1 (less), 1 (greater)
```

## Panic and Error Handling

Robust error handling and program termination mechanisms.

```cursed
# Panic with message (terminates program)
panic("Critical error occurred!")

# Unreachable code marker
unreachable()  # Panics if reached

# TODO marker
todo("Implement this feature")

# Assertions
assert(condition, "Assertion failed message")
debug_assert(condition, "Debug assertion failed")

# Recovery mechanism
sus recovered lit = unbothered()  # Always returns based
```

## Array and Slice Utilities

Collection manipulation and bounds checking functions.

```cursed
# Array length and access
sus length normie = array_len(5)
sus element normie = array_get(base_addr, index)
array_set(base_addr, index, value)

# Bounds checking
sus in_bounds lit = array_bounds_check(index, length)

# Array operations
array_copy(dest_addr, src_addr, length)
array_fill(arr_addr, value, length)
```

## String Utilities

Comprehensive string processing functions.

```cursed
# String length and concatenation
sus len normie = string_len("hello")              # 5
sus combined tea = string_concat("hello", "world") # "helloworld"

# String comparison and searching
sus equal lit = string_eq("hello", "hello")       # based
sus contains lit = string_contains("hello world", "world") # based
sus starts lit = string_starts_with("hello world", "hello") # based
sus ends lit = string_ends_with("hello world", "world")     # based

# String transformation
sus trimmed tea = string_trim("  hello  ")        # "hello"
sus upper tea = string_to_upper("hello")          # "HELLO"
sus lower tea = string_to_lower("HELLO")          # "hello"

# String splitting and replacement
sus parts (tea, tea) = string_split_first("hello,world", ",")  # ("hello", "world")
sus replaced tea = string_replace("hello world", "world", "CURSED") # "hello CURSED"
```

## Mathematical Utilities

Essential mathematical operations and functions.

```cursed
# Basic arithmetic
sus maximum normie = max(10, 5)     # 10
sus minimum normie = min(10, 5)     # 5
sus absolute normie = abs(-42)      # 42

# Power and square root
sus power normie = pow(2, 3)        # 8
sus root normie = sqrt(16)          # 4

# Utility functions
sus clamped normie = clamp(15, 0, 10)  # 10
sus in_range lit = in_range(5, 0, 10)  # based
```

## Boolean Utilities

Logical operations and boolean algebra.

```cursed
# Logical operations
sus inverted lit = not(based)           # cap
sus and_result lit = and(based, cap)    # cap
sus or_result lit = or(based, cap)      # based
sus xor_result lit = xor(based, cap)    # based

# Comparison utilities
sus int_cmp normie = compare_int(5, 10)     # -1
sus str_cmp normie = compare_string("a", "b") # -1

# Swap utility
sus swapped (normie, normie) = swap_int(10, 20) # (20, 10)
```

## Compiler Utilities for Self-Hosting

Support functions for the Stage 2 self-hosting compiler.

```cursed
# Token type constants
sus token_id normie = token_type_identifier()  # 1
sus token_num normie = token_type_number()     # 2
sus token_str normie = token_type_string()     # 3
sus token_kw normie = token_type_keyword()     # 4
sus token_op normie = token_type_operator()    # 5

# Error codes
sus syntax_err normie = error_code_syntax()    # 1000
sus type_err normie = error_code_type()        # 2000
sus runtime_err normie = error_code_runtime()  # 3000

# Hash function for symbol tables
sus hash_val normie = hash_string("variable")  # Hash for symbol table
```

## Usage Examples

### Basic Type Conversion Pipeline

```cursed
yeet "core"

# String to number conversion
sus input tea = "42"
sus number normie = int_from_string(input)
sus output tea = string_from_int(number)
vibez.spill("Converted: " + output)  # "42"
```

### Option Type Error Handling

```cursed
yeet "core"

slay safe_divide(a normie, b normie) (lit, normie) {
    bestie b == 0 {
        damn option_none()
    }
    damn option_some(a / b)
}

sus result (lit, normie) = safe_divide(10, 2)
sus value normie = option_unwrap_or(result, 0)
vibez.spill("Result: " + string_from_int(value))
```

### Result Type Error Propagation

```cursed
yeet "core"

slay parse_number(input tea) (lit, normie, normie) {
    bestie input == "42" {
        damn result_ok(42)
    }
    damn result_err(1001)  # Parse error
}

sus parse_result (lit, normie, normie) = parse_number("42")
bestie result_is_ok(parse_result) {
    sus number normie = result_unwrap(parse_result)
    vibez.spill("Parsed: " + string_from_int(number))
} else {
    sus error normie = result_get_error(parse_result)
    vibez.spill("Parse error: " + string_from_int(error))
}
```

### String Processing Pipeline

```cursed
yeet "core"

sus text tea = "  hello,world  "
sus trimmed tea = string_trim(text)              # "hello,world"
sus parts (tea, tea) = string_split_first(trimmed, ",")  # ("hello", "world")
sus greeting tea = string_to_upper(parts.0)     # "HELLO"
sus target tea = string_to_upper(parts.1)       # "WORLD"
sus result tea = string_concat(greeting, " " + target)  # "HELLO WORLD"
vibez.spill(result)
```

### Self-Hosting Compiler Support

```cursed
yeet "core"

# Token processing for lexer
slay process_token(token_text tea) normie {
    bestie string_starts_with(token_text, "sus") {
        damn token_type_keyword()
    }
    bestie string_contains(token_text, "0123456789") {
        damn token_type_number()
    }
    damn token_type_identifier()
}

# Symbol table with hash
slay add_symbol(name tea, value normie) {
    sus hash normie = hash_string(name)
    vibez.spill("Adding symbol: " + name + " hash: " + string_from_int(hash))
}

# Error reporting
slay report_error(message tea, code normie) {
    vibez.spill("Error " + string_from_int(code) + ": " + message)
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/core/test_core.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/core/test_core.csd
./test_core

# Test both modes for compatibility
cargo run --bin cursed stdlib/core/test_core.csd > interp_output.txt
cargo run --bin cursed -- compile stdlib/core/test_core.csd
./test_core > comp_output.txt
diff interp_output.txt comp_output.txt
```

## Implementation Notes

- **Pure CURSED**: All functions implemented without FFI dependencies
- **Self-Hosting Ready**: Includes essential primitives for Stage 2 compiler
- **Memory Safe**: Option and Result types prevent null pointer exceptions
- **Performance**: Optimized for both interpretation and compilation modes
- **Compatibility**: Maintains backward compatibility with existing core functions
- **Comprehensive**: Covers all essential language primitives and utilities

## Self-Hosting Integration

The core module provides critical support for the Stage 2 self-hosting compiler:

1. **Type System**: Complete type conversion functions for compiler type checking
2. **Error Handling**: Result and Option types for safe error propagation
3. **Memory Management**: Allocation utilities for compiler memory management
4. **String Processing**: Text processing for source code manipulation
5. **Symbol Tables**: Hash functions and comparison utilities
6. **Token Processing**: Token type constants and classification functions

This comprehensive core module enables the CURSED compiler to compile itself, providing all essential language primitives needed for self-hosting capability.
