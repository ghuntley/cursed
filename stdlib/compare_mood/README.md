# compare_mood Module

The `compare_mood` module provides comprehensive comparison utilities for CURSED programs. It offers a consistent interface for comparing values, checking equality, and performing ordering operations across different data types.

## Overview

This module implements comparison functions for basic types including integers (`normie`), floats (`meal`), strings (`tea`), and booleans (`lit`). It follows the standard comparison convention where:
- `-1` (LessThan): First value is less than second
- `0` (Equal): Values are equal
- `1` (GreaterThan): First value is greater than second

## Core Constants

```cursed
sus LessThan normie = -1
sus Equal normie = 0
sus GreaterThan normie = 1
```

## Functions

### Basic Comparison Functions

#### Integer Comparisons
- `CompareInt(a normie, b normie) normie` - Compare two integers
- `EqualInt(a normie, b normie) lit` - Check if integers are equal
- `LessInt(a normie, b normie) lit` - Check if first integer is less than second
- `GreaterInt(a normie, b normie) lit` - Check if first integer is greater than second
- `LessEqualInt(a normie, b normie) lit` - Check if first integer is less than or equal to second
- `GreaterEqualInt(a normie, b normie) lit` - Check if first integer is greater than or equal to second

#### String Comparisons
- `CompareString(a tea, b tea) normie` - Compare two strings lexicographically
- `EqualString(a tea, b tea) lit` - Check if strings are equal
- `LessString(a tea, b tea) lit` - Check if first string is less than second
- `GreaterString(a tea, b tea) lit` - Check if first string is greater than second
- `LessEqualString(a tea, b tea) lit` - Check if first string is less than or equal to second
- `GreaterEqualString(a tea, b tea) lit` - Check if first string is greater than or equal to second

#### Float Comparisons
- `CompareFloat(a meal, b meal) normie` - Compare two floats
- `EqualFloat(a meal, b meal) lit` - Check if floats are equal
- `LessFloat(a meal, b meal) lit` - Check if first float is less than second
- `GreaterFloat(a meal, b meal) lit` - Check if first float is greater than second
- `LessEqualFloat(a meal, b meal) lit` - Check if first float is less than or equal to second
- `GreaterEqualFloat(a meal, b meal) lit` - Check if first float is greater than or equal to second

#### Boolean Comparisons
- `CompareBool(a lit, b lit) lit` - Compare two booleans (cap < based)
- `EqualBool(a lit, b lit) lit` - Check if booleans are equal

### Utility Functions

#### Min/Max Functions
- `MinInt(a normie, b normie) normie` - Return smaller integer
- `MaxInt(a normie, b normie) normie` - Return larger integer
- `MinFloat(a meal, b meal) meal` - Return smaller float
- `MaxFloat(a meal, b meal) meal` - Return larger float
- `MinString(a tea, b tea) tea` - Return lexicographically smaller string
- `MaxString(a tea, b tea) tea` - Return lexicographically larger string

#### Clamp Functions
- `ClampInt(value normie, min normie, max normie) normie` - Clamp integer to range
- `ClampFloat(value meal, min meal, max meal) meal` - Clamp float to range

#### Sign Functions
- `SignInt(value normie) normie` - Return sign of integer (-1, 0, or 1)
- `SignFloat(value meal) normie` - Return sign of float (-1, 0, or 1)

#### Absolute Value Functions
- `AbsInt(value normie) normie` - Return absolute value of integer
- `AbsFloat(value meal) meal` - Return absolute value of float

#### Distance Functions
- `DistanceInt(a normie, b normie) normie` - Return absolute difference between integers
- `DistanceFloat(a meal, b meal) meal` - Return absolute difference between floats

#### Range Check Functions
- `BetweenInt(value normie, min normie, max normie) lit` - Check if integer is in range (inclusive)
- `BetweenFloat(value meal, min meal, max meal) lit` - Check if float is in range (inclusive)
- `BetweenExclusiveInt(value normie, min normie, max normie) lit` - Check if integer is in range (exclusive)
- `BetweenExclusiveFloat(value meal, min meal, max meal) lit` - Check if float is in range (exclusive)

#### Helper Functions
- `ThreeWay(less_condition lit, greater_condition lit) normie` - Three-way comparison helper

## Usage Examples

### Basic Comparisons

```cursed
yeet "compare_mood"

# Compare integers
sus result normie = CompareInt(5, 10)
spill result == LessThan {
    vibez.spill("5 is less than 10")
}

# Compare strings
sus str_result normie = CompareString("apple", "banana")
spill str_result == LessThan {
    vibez.spill("apple comes before banana")
}

# Check equality
spill EqualInt(42, 42) {
    vibez.spill("Numbers are equal")
}
```

### Using Min/Max Functions

```cursed
yeet "compare_mood"

sus smaller normie = MinInt(15, 8)  # Returns 8
sus larger normie = MaxInt(15, 8)   # Returns 15

sus min_str tea = MinString("zebra", "apple")  # Returns "apple"
```

### Clamping Values

```cursed
yeet "compare_mood"

sus clamped normie = ClampInt(150, 0, 100)  # Returns 100
sus clamped_float meal = ClampFloat(-5.5, 0.0, 10.0)  # Returns 0.0
```

### Range Checking

```cursed
yeet "compare_mood"

spill BetweenInt(5, 1, 10) {
    vibez.spill("5 is between 1 and 10")
}

spill BetweenExclusiveFloat(2.5, 1.0, 10.0) {
    vibez.spill("2.5 is between 1.0 and 10.0 (exclusive)")
}
```

### Using Three-Way Comparison

```cursed
yeet "compare_mood"

sus a normie = 10
sus b normie = 5
sus comparison normie = ThreeWay(a < b, a > b)

spill comparison == GreaterThan {
    vibez.spill("a is greater than b")
}
```

## Implementation Details

This module provides a pure CURSED implementation without external dependencies. All comparison functions follow consistent semantics:

- Integer comparisons use natural ordering
- String comparisons use lexicographical ordering
- Float comparisons use IEEE 754 ordering
- Boolean comparisons treat `cap` as less than `based`

The module is designed for performance and thread safety, making it suitable for use in concurrent CURSED programs.

## Testing

Run the comprehensive test suite with:

```bash
cargo run --bin cursed stdlib/compare_mood/test_compare_mood.csd
```

The test suite covers all functions, edge cases, and error conditions to ensure reliable behavior across all comparison operations.
