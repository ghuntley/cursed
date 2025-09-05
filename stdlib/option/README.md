# Option Module

The `option` module provides optional values that can be `Some(value)` or `None`. This is critical for self-hosting and safe error handling.

## Core Functions

### Construction
- `some_int(value normie) (lit, normie)` - Create Some with integer value
- `some_string(value tea) (lit, tea)` - Create Some with string value
- `some_bool(value lit) (lit, lit)` - Create Some with boolean value
- `none_int() (lit, normie)` - Create None for integer type
- `none_string() (lit, tea)` - Create None for string type
- `none_bool() (lit, lit)` - Create None for boolean type

### Checking
- `is_some_int(option (lit, normie)) lit` - Check if option is Some
- `is_none_int(option (lit, normie)) lit` - Check if option is None
- Similar functions for string and bool types

### Unwrapping
- `unwrap_int(option (lit, normie)) normie` - Get value or panic
- `unwrap_or_int(option (lit, normie), default normie) normie` - Get value or default
- `unwrap_or_string(option (lit, tea), default tea) tea` - Get value or default

### Transformations
- `map_int_to_string(option (lit, normie)) (lit, tea)` - Transform Some values
- `filter_int(option (lit, normie), condition func(normie) lit) (lit, normie)` - Filter values
- `and_then_int(option (lit, normie), f func(normie) (lit, normie)) (lit, normie)` - Chain operations

## Usage Example

```cursed
yeet "option"

# Create some values
sus maybe_val := option.some_int(42)
sus empty_val := option.none_int()

# Check and unwrap
bestie option.is_some_int(maybe_val) {
    vibez.spill("Value: " + core.tea(option.unwrap_int(maybe_val)))
}

# Use defaults
sus result := option.unwrap_or_int(empty_val, 0)
```

## Testing

```bash
cargo run --bin cursed stdlib/option/test_option.💀
```

## Status

✅ **Production Ready** - Fully implemented and tested
- All core Option operations
- Type-safe construction and destruction
- Comprehensive test coverage
- Zero FFI dependencies
