# NoCap (strconv package)

## Overview
NoCap provides string conversion utilities for Cursed, inspired by Go's strconv package. It handles conversions between strings and other basic data types with no cap (without limitations).

## String to Value Conversions

### `FactsCheck`
Parse a string as a boolean value (equivalent to strconv.ParseBool).

```go
func FactsCheck(s string) (bool, error)
```

Values for true: "1", "t", "T", "true", "TRUE", "True", "facts", "FACTS", "Facts", "no cap", "fr fr"
Values for false: "0", "f", "F", "false", "FALSE", "False", "cap", "CAP", "idk"

### `YoinkInt`
Parse a string as an integer (equivalent to strconv.ParseInt).

```go
func YoinkInt(s string, base int, bitSize int) (i int64, err error)
```

### `YoinkUint`
Parse a string as an unsigned integer (equivalent to strconv.ParseUint).

```go
func YoinkUint(s string, base int, bitSize int) (uint64, error)
```

### `YoinkFloat`
Parse a string as a floating-point number (equivalent to strconv.ParseFloat).

```go
func YoinkFloat(s string, bitSize int) (float64, error)
```

## Value to String Conversions

### `YeetBool`
Convert a boolean to a string (equivalent to strconv.FormatBool).

```go
func YeetBool(b bool) string
```

Returns "facts" for true, "cap" for false.

### `YeetInt`
Convert an integer to a string (equivalent to strconv.FormatInt).

```go
func YeetInt(i int64, base int) string
```

### `YeetUint`
Convert an unsigned integer to a string (equivalent to strconv.FormatUint).

```go
func YeetUint(i uint64, base int) string
```

### `YeetFloat`
Convert a floating-point number to a string (equivalent to strconv.FormatFloat).

```go
func YeetFloat(f float64, fmt byte, prec, bitSize int) string
```

## Convenience Functions

### `Atoi`
Convert a string to an int (equivalent to strconv.Atoi).

```go
func Atoi(s string) (int, error)
```

### `Itoa`
Convert an int to a string (equivalent to strconv.Itoa).

```go
func Itoa(i int) string
```

## Special Features

### `SussyFloat`
Specialized formatter for floating-point numbers that identify "sus" values like NaN and Inf.

```go
func SussyFloat(f float64) string
```

Returns:
- "sus" for NaN
- "bussin" for +Inf
- "busted" for -Inf
- Regular string representation otherwise

## Error Handling

```go
var ErrSyntax = errors.New("sus conversion, invalid syntax")
var ErrRange = errors.New("too extra, value out of range")
```

## Implementation Guidelines
1. All functions must be thoroughly tested against edge cases
2. Performance should be prioritized for common use cases
3. Error messages should be clear and helpful
4. Maintain compatibility with Go's strconv semantics where possible