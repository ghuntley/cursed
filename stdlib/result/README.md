# Result Module

The `result` module provides error handling with `Ok(value)` and `Err(error)` types. This is critical for self-hosting and robust error handling.

## Core Functions

### Construction
- `ok_int(value normie) (lit, normie, tea)` - Create Ok with integer value
- `ok_string(value tea) (lit, tea, tea)` - Create Ok with string value
- `ok_bool(value lit) (lit, lit, tea)` - Create Ok with boolean value
- `err_int(error tea) (lit, normie, tea)` - Create Err with error message
- `err_string(error tea) (lit, tea, tea)` - Create Err with error message
- `err_bool(error tea) (lit, lit, tea)` - Create Err with error message

### Checking
- `is_ok_int(result (lit, normie, tea)) lit` - Check if result is Ok
- `is_err_int(result (lit, normie, tea)) lit` - Check if result is Err
- Similar functions for string and bool types

### Unwrapping
- `unwrap_int(result (lit, normie, tea)) normie` - Get Ok value or panic
- `unwrap_err_int(result (lit, normie, tea)) tea` - Get Err value or panic
- `unwrap_or_int(result (lit, normie, tea), default normie) normie` - Get Ok value or default
- `unwrap_or_else_int(result (lit, normie, tea), f func(tea) normie) normie` - Get Ok value or compute default

### Transformations
- `map_int_to_string(result (lit, normie, tea)) (lit, tea, tea)` - Transform Ok values
- `map_err_int(result (lit, normie, tea), f func(tea) tea) (lit, normie, tea)` - Transform Err values
- `and_then_int(result (lit, normie, tea), f func(normie) (lit, normie, tea)) (lit, normie, tea)` - Chain operations
- `or_else_int(result (lit, normie, tea), f func(tea) (lit, normie, tea)) (lit, normie, tea)` - Handle errors

### Safe Operations
- `safe_divide(a normie, b normie) (lit, normie, tea)` - Division with error handling
- `safe_string_index(s tea, index normie) (lit, tea, tea)` - String indexing with bounds checking
- `safe_int_parse(s tea) (lit, normie, tea)` - Integer parsing with error handling

## Usage Example

```cursed
yeet "result"

# Safe division
sus div_result := result.safe_divide(10, 2)
bestie result.is_ok_int(div_result) {
    vibez.spill("Result: " + core.tea(result.unwrap_int(div_result)))
} else {
    vibez.spill("Error: " + result.unwrap_err_int(div_result))
}

# Error handling with defaults
sus safe_result := result.unwrap_or_int(div_result, 0)
```

## Testing

```bash
cargo run --bin cursed stdlib/result/test_result.csd
```

## Status

✅ **Production Ready** - Fully implemented and tested
- All core Result operations
- Comprehensive error handling patterns
- Safe operations with proper error messages
- Zero FFI dependencies
