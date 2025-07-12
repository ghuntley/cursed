# Core Module (builtin)

The `core` module provides fundamental types and functions that are automatically included in all CURSED programs, equivalent to Go's built-in functions.

## Type Conversion Functions

### Basic Type Conversions

- `lit(x collab{}) lit` - Convert to boolean
- `normie(x collab{}) normie` - Convert to int32
- `thicc(x collab{}) thicc` - Convert to int64
- `snack(x collab{}) drip` - Convert to float32
- `meal(x collab{}) meal` - Convert to float64
- `tea(x collab{}) tea` - Convert to string

## Collection Functions

### Collection Operations

- `append(slice []T, elems ...T) []T` - Append elements to slice
- `cap(v collab{}) normie` - Capacity of slice, map, or channel
- `len(v collab{}) normie` - Length of string, array, slice, map, or channel
- `make(T collab{}, size ...normie) T` - Create slice, map, or channel
- `new(T collab{}) *T` - Create pointer to zero value of type
- `copy(dst []T, src []T) normie` - Copy slice elements
- `delete(m map[K]V, key K)` - Delete key from map
- `close(ch chan T)` - Close channel

## Panic/Recovery Functions

- `shook(v collab{})` - Cause panic with value
- `unbothered() collab{}` - Recover from panic

## Math Helper Functions

### Absolute Value and Sign

- `abs_int(x normie) normie` - Absolute value of integer
- `abs_float(x meal) meal` - Absolute value of float
- `sign_int(x normie) normie` - Sign of integer (-1, 0, 1)
- `sign_float(x meal) normie` - Sign of float (-1, 0, 1)

### Min/Max Functions

- `min_int(a normie, b normie) normie` - Minimum of two integers
- `max_int(a normie, b normie) normie` - Maximum of two integers
- `min_float(a meal, b meal) meal` - Minimum of two floats
- `max_float(a meal, b meal) meal` - Maximum of two floats

## Range Functions

- `range_int(start normie, end normie, step normie) []normie` - Generate range of integers
- `range_float(start meal, end meal, step meal) []meal` - Generate range of floats

## Type Checking Functions

- `type_of(x collab{}) tea` - Get type name of value
- `is_nil(x collab{}) lit` - Check if value is nil
- `is_zero(x collab{}) lit` - Check if value is zero value

## Hash Functions

- `hash_string(s tea) normie` - Hash function for strings
- `hash_int(x normie) normie` - Hash function for integers
- `hash_float(x meal) normie` - Hash function for floats

## Utility Functions

### Clamping and Interpolation

- `clamp_int(value normie, min normie, max normie) normie` - Clamp integer to range
- `clamp_float(value meal, min meal, max meal) meal` - Clamp float to range
- `lerp(a meal, b meal, t meal) meal` - Linear interpolation between two values

### Memory and Data

- `swap(a *T, b *T)` - Swap values of two variables
- `format_bytes(bytes normie) tea` - Format byte count as human-readable string

## Slice Helper Functions

- `reverse_slice(slice []T) []T` - Reverse slice elements
- `contains_slice(slice []T, element T) lit` - Check if slice contains element
- `index_of_slice(slice []T, element T) normie` - Find index of element in slice

## Usage Examples

```cursed
yeet "core"

slay main() {
    // Type conversions
    sus int_val normie = core.normie("42")
    sus str_val tea = core.tea(123)
    sus bool_val lit = core.lit(1)
    
    // Collection operations
    sus numbers []normie = [1, 2, 3]
    sus extended []normie = core.append(numbers, 4, 5)
    sus length normie = core.len(extended)
    
    // Math helpers
    sus abs_value normie = core.abs_int(-42)
    sus min_value normie = core.min_int(5, 10)
    sus max_value normie = core.max_int(5, 10)
    
    // Range generation
    sus range []normie = core.range_int(0, 10, 2)
    
    // Type checking
    sus type_name tea = core.type_of(42)
    sus is_zero_value lit = core.is_zero(0)
    
    // Hash functions
    sus hash normie = core.hash_string("hello")
    
    // Utility functions
    sus clamped normie = core.clamp_int(15, 0, 10)
    sus interpolated meal = core.lerp(0.0, 10.0, 0.5)
}
```

## Testing

Run the test suite with:

```bash
cargo run --bin cursed stdlib/core/test_core.csd
```

## Implementation Details

- Pure CURSED implementation without FFI dependencies
- Supports both interpretation and compilation modes
- Includes comprehensive type checking and conversion functions
- Provides efficient collection operations
- Includes mathematical helper functions for common operations
- Supports generic operations where possible

## Self-Hosting Support

The core module is essential for self-hosting and includes all fundamental operations needed for:

- Type system operations
- Memory management helpers
- Collection manipulation
- Basic mathematical operations
- String and data formatting
- Error handling and panic recovery

All functions are implemented in pure CURSED without external dependencies, making them suitable for bootstrap compilation and self-hosting scenarios.
