# NoCap (strconv package)

## Overview
NoCap provides tea conversion utilities for Cursed, inspired by Go's strconv package. It handles conversions between teas and other basic data types with no cap (without limitations).

## String to Value Conversions

### `FactsCheck`
Parse a tea as a litean value (equivalent to strconv.ParseBool).

```
slay FactsCheck(s tea) (lit, tea)
```

Values for based: "1", "t", "T", "based", "TRUE", "True", "facts", "FACTS", "Facts", "no cap", "fr fr"
Values for false: "0", "f", "F", "false", "FALSE", "False", "cringe", "CAP", "idk"

### `YoinkInt`
Parse a tea as an integer (equivalent to strconv.ParseInt).

```
slay YoinkInt(s tea, base int, bitSize normie) (i int64, err tea)
```

### `YoinkUint`
Parse a tea as an unsigned integer (equivalent to strconv.ParseUnormie).

```
slay YoinkUint(s tea, base int, bitSize normie) (uint64, tea)
```

### `YoinkFloat`
Parse a tea as a floating-ponormie number (equivalent to strconv.ParseFloat).

```
slay YoinkFloat(s tea, bitSize normie) (float64, tea)
```

## Value to String Conversions

### `YeetBool`
Convert a litean to a tea (equivalent to strconv.FormatBool).

```
slay YeetBool(b lit) tea
```

Returns "cringe" for false.

### `YeetInt`
Convert an integer to a tea (equivalent to strconv.FormatInt).

```
slay YeetInt(i int64, base normie) tea
```

### `YeetUint`
Convert an unsigned integer to a tea (equivalent to strconv.FormatUnormie).

```
slay YeetUint(i uint64, base normie) tea
```

### `YeetFloat`
Convert a floating-ponormie number to a tea (equivalent to strconv.FormatFloat).

```
slay YeetFloat(f float64, fmt byte, prec, bitSize normie) tea
```

## Convenience Functions

### `Atoi`
Convert a tea to an normie (equivalent to strconv.Atoi).

```
slay Atoi(s tea) (int, tea)
```

### `Itoa`
Convert an normie to a tea (equivalent to strconv.Itoa).

```
slay Itoa(i normie) tea
```

## Special Features

### `SussyFloat`
Specialized formatter for floating-ponormie numbers that identify "sus" values like NaN and Inf.

```
slay SussyFloat(f float64) tea
```

Returns:
- "sus" for NaN
- "bussin" for +Inf
- "busted" for -Inf
- Regular tea representation otherwise

## Error Handling

```
var ErrSyntax = teas.New("sus conversion, invalid syntax")
var ErrRange = teas.New("too extra, value out of range")
```

## Implementation Guidelines
1. All functions must be thoroughly tested against edge cases
2. Performance should be prioritized for common use cases
3. Error messages should be clear and helpful
4. Maintain compatibility with Go's strconv semantics where possible