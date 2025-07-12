# Core Module

The core module provides fundamental types and functions that are automatically included in all CURSED programs. This module forms the foundation of the CURSED standard library and provides essential utilities for type conversion, collection manipulation, memory management, and basic operations.

## Features

- **Type Conversion Functions**: Convert between different CURSED types
- **Collection Manipulation**: Basic operations on slices and arrays
- **Memory Management**: Functions for creating and managing memory
- **Panic and Recovery**: Error handling mechanisms
- **Utility Functions**: Common mathematical and string operations

## Type Conversion Functions

### `lit(value normie) lit`
Converts a numeric value to a boolean. Returns `based` (true) for non-zero values, `cap` (false) for zero.

```cursed
sus flag lit = lit(42)  // based (true)
sus flag2 lit = lit(0)  // cap (false)
```

### `normie(value lit) normie`
Converts a boolean to an integer. Returns 1 for `based` (true), 0 for `cap` (false).

```cursed
sus num normie = normie(based)  // 1
sus num2 normie = normie(cap)   // 0
```

### `thicc(value normie) thicc`
Converts a 32-bit integer to a 64-bit integer.

```cursed
sus bigNum thicc = thicc(42)
```

### `snack(value normie) snack`
Converts an integer to a 32-bit float.

```cursed
sus floatVal snack = snack(42)
```

### `meal(value normie) meal`
Converts an integer to a 64-bit float.

```cursed
sus doubleVal meal = meal(42)
```

### `tea(value normie) tea`
Converts an integer to its string representation.

```cursed
sus str tea = tea(42)      // "42"
sus str2 tea = tea(-123)   // "-123"
```

## Collection Manipulation Functions

### `append(slice []normie, element normie) []normie`
Appends an element to a slice and returns a new slice.

```cursed
sus numbers []normie = make(3)
numbers[0] = 1
numbers[1] = 2
numbers[2] = 3
sus newNumbers []normie = append(numbers, 4)
```

### `cap(slice []normie) normie`
Returns the capacity of a slice (length plus buffer space).

```cursed
sus capacity normie = cap(numbers)
```

### `len(slice []normie) normie`
Returns the length of a slice.

```cursed
sus length normie = len(numbers)
```

## Memory Management Functions

### `make(size normie) []normie`
Creates a new slice with the specified size, initialized with zeros.

```cursed
sus newSlice []normie = make(10)
```

### `new(initialValue normie) *normie`
Creates a new pointer to a value with the specified initial value.

```cursed
sus ptr *normie = new(42)
sus value normie = *ptr  // 42
```

## Panic and Recovery Mechanisms

### `shook(message tea)`
Triggers a panic with the specified message. This terminates the program execution.

```cursed
shook("Something went wrong!")
```

### `unbothered() lit`
Recovery mechanism that returns `based` (true) if recovery was successful.

```cursed
sus recovered lit = unbothered()
```

## Utility Functions

### Mathematical Operations

#### `max(a normie, b normie) normie`
Returns the maximum of two values.

```cursed
sus maximum normie = max(5, 10)  // 10
```

#### `min(a normie, b normie) normie`
Returns the minimum of two values.

```cursed
sus minimum normie = min(5, 10)  // 5
```

#### `abs(value normie) normie`
Returns the absolute value of a number.

```cursed
sus absolute normie = abs(-42)  // 42
```

#### `pow(base normie, exponent normie) normie`
Raises a number to a power.

```cursed
sus result normie = pow(2, 3)  // 8
```

#### `sqrt(value normie) normie`
Returns the square root of a number (integer approximation).

```cursed
sus root normie = sqrt(16)  // 4
```

### String Operations

#### `string_len(str tea) normie`
Returns the length of a string.

```cursed
sus length normie = string_len("hello")  // 5
```

#### `string_concat(a tea, b tea) tea`
Concatenates two strings.

```cursed
sus combined tea = string_concat("hello", "world")  // "helloworld"
```

### Boolean Operations

#### `not(value lit) lit`
Returns the logical NOT of a boolean value.

```cursed
sus inverted lit = not(based)  // cap
```

#### `and(a lit, b lit) lit`
Returns the logical AND of two boolean values.

```cursed
sus result lit = and(based, cap)  // cap
```

#### `or(a lit, b lit) lit`
Returns the logical OR of two boolean values.

```cursed
sus result lit = or(based, cap)  // based
```

## Usage

The core module is automatically imported in all CURSED programs. You can use any of these functions without explicitly importing the module:

```cursed
// Core functions are available immediately
sus x normie = 42
sus str tea = tea(x)
sus flag lit = lit(x)

vibez.spill("Number: " + str)
bestie flag {
    vibez.spill("Value is non-zero")
}
```

## Testing

Run the core module tests:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/core/test_core.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/core/test_core.csd
./test_core
```

## Implementation Notes

- All functions are implemented in pure CURSED without FFI dependencies
- Type conversion functions handle edge cases appropriately
- Collection functions include safety limits to prevent infinite loops
- Mathematical functions use integer arithmetic with approximations where needed
- String operations assume null-terminated strings for compatibility

## Examples

### Basic Type Conversion

```cursed
sus number normie = 42
sus text tea = tea(number)
sus flag lit = lit(number)
sus bigNumber thicc = thicc(number)

vibez.spill("Number: " + text)
bestie flag {
    vibez.spill("Number is truthy")
}
```

### Collection Operations

```cursed
sus data []normie = make(5)
data[0] = 10
data[1] = 20
data[2] = 30

sus length normie = len(data)
sus capacity normie = cap(data)
sus extended []normie = append(data, 40)

vibez.spill("Length: " + tea(length))
vibez.spill("Capacity: " + tea(capacity))
```

### Mathematical Operations

```cursed
sus a normie = 10
sus b normie = 5

sus maximum normie = max(a, b)
sus minimum normie = min(a, b)
sus power normie = pow(a, 2)
sus root normie = sqrt(a * a)

vibez.spill("Max: " + tea(maximum))
vibez.spill("Min: " + tea(minimum))
vibez.spill("Power: " + tea(power))
vibez.spill("Root: " + tea(root))
```

This core module provides the essential building blocks for all CURSED programs and serves as the foundation for more complex standard library modules.
