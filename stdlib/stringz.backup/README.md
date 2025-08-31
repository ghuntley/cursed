# StringZ - Core String Processing Module

Pure CURSED implementation of essential string operations for manipulation, formatting, parsing, and validation.

## Overview

The StringZ module provides four core categories of string functionality:

1. **String Manipulation** - Split, join, replace, reverse, and substring operations
2. **String Formatting** - Template formatting, interpolation, and padding operations  
3. **String Parsing** - Parse integers and booleans from strings, convert back to strings
4. **String Validation** - Length, contains, prefix/suffix checks, character type validation

## Usage

```cursed
yeet "stringz"

sus words []tea = split("hello,world,cursed", ",")
sus message tea = join(words, " ")
sus formatted tea = format_template("Hello {}", ["CURSED"])
```

## Function Reference

### String Manipulation

#### `split(s tea, delimiter tea) []tea`
Split a string into array of substrings using delimiter.

```cursed
sus parts []tea = split("a,b,c", ",")  # ["a", "b", "c"]
sus words []tea = split("hello world", " ")  # ["hello", "world"]
```

#### `join(parts []tea, delimiter tea) tea`
Join array of strings with delimiter.

```cursed
sus result tea = join(["a", "b", "c"], ",")  # "a,b,c"
sus sentence tea = join(["hello", "world"], " ")  # "hello world"
```

#### `replace(s tea, find tea, replacement tea) tea`
Replace first occurrence of substring.

```cursed
sus result tea = replace("hello world", "hello", "hi")  # "hi world"
```

#### `replace_all(s tea, find tea, replacement tea) tea`
Replace all occurrences of substring.

```cursed
sus result tea = replace_all("test test test", "test", "exam")  # "exam exam exam"
```

#### `reverse(s tea) tea`
Reverse the characters in a string.

```cursed
sus backwards tea = reverse("hello")  # "olleh"
```

#### `substring(s tea, start drip, length drip) tea`
Extract substring starting at index with given length.

```cursed
sus part tea = substring("hello", 1, 3)  # "ell"
```

### String Formatting

#### `format_template(template tea, replacements []tea) tea`
Format template string with `{}` placeholders.

```cursed
sus msg tea = format_template("Hello {}", ["World"])  # "Hello World"
sus full tea = format_template("{} says {}", ["Alice", "hi"])  # "Alice says hi"
```

#### `interpolate(template tea, key tea, value tea) tea`
Simple string interpolation for single key-value pair.

```cursed
sus greeting tea = interpolate("Hello {name}", "name", "Alice")  # "Hello Alice"
```

#### `pad_left(s tea, length drip, pad_char tea) tea`
Pad string on the left to reach target length.

```cursed
sus padded tea = pad_left("42", 5, "0")  # "00042"
```

#### `pad_right(s tea, length drip, pad_char tea) tea`
Pad string on the right to reach target length.

```cursed
sus padded tea = pad_right("test", 8, "-")  # "test----"
```

#### `center(s tea, length drip, pad_char tea) tea`
Center string with padding to reach target length.

```cursed
sus centered tea = center("hi", 6, " ")  # "  hi  "
```

#### `repeat_char(c tea, count drip) tea`
Repeat character specified number of times.

```cursed
sus line tea = repeat_char("-", 10)  # "----------"
```

### String Parsing

#### `parse_int(s tea) drip`
Parse string to integer, returns 0 for invalid input.

```cursed
sus num drip = parse_int("42")  # 42
sus zero drip = parse_int("invalid")  # 0
```

#### `parse_bool(s tea) lit`
Parse string to boolean, supports multiple formats.

```cursed
sus yes lit = parse_bool("true")  # based
sus no lit = parse_bool("false")  # cringe
sus also_yes lit = parse_bool("YES")  # based
```

#### `to_int(n drip) tea`
Convert integer to string representation.

```cursed
sus text tea = to_int(42)  # "42"
```

#### `to_string(b lit) tea`
Convert boolean to string representation.

```cursed
sus text tea = to_string(based)  # "true"
```

#### `trim_digits(s tea) tea`
Remove all digit characters from string.

```cursed
sus clean tea = trim_digits("abc123def")  # "abcdef"
```

### String Validation

#### `len_string(s tea) drip`
Get the length of a string.

```cursed
sus length drip = len_string("hello")  # 5
```

#### `is_empty(s tea) lit`
Check if string is empty.

```cursed
sus empty lit = is_empty("")  # based
sus not_empty lit = is_empty("test")  # cringe
```

#### `contains(s tea, search tea) lit`
Check if string contains substring.

```cursed
sus found lit = contains("hello world", "world")  # based
sus missing lit = contains("hello world", "xyz")  # cringe
```

#### `starts_with(s tea, prefix tea) lit`
Check if string starts with prefix.

```cursed
sus starts lit = starts_with("hello world", "hello")  # based
```

#### `ends_with(s tea, suffix tea) lit`
Check if string ends with suffix.

```cursed
sus ends lit = ends_with("test.txt", ".txt")  # based
```

#### `is_numeric(s tea) lit`
Check if string contains only digits.

```cursed
sus is_num lit = is_numeric("123")  # based
sus not_num lit = is_numeric("12a")  # cringe
```

#### `is_alpha(s tea) lit`
Check if string contains only letters.

```cursed
sus letters lit = is_alpha("hello")  # based
sus mixed lit = is_alpha("hello123")  # cringe
```

#### `is_alphanumeric(s tea) lit`
Check if string contains only letters and digits.

```cursed
sus alphanum lit = is_alphanumeric("abc123")  # based
sus special lit = is_alphanumeric("abc!")  # cringe
```

### Utility Functions

#### `to_lowercase(s tea) tea`
Convert string to lowercase.

```cursed
sus lower tea = to_lowercase("HELLO")  # "hello"
```

#### `to_uppercase(s tea) tea`
Convert string to uppercase.

```cursed
sus upper tea = to_uppercase("hello")  # "HELLO"
```

#### `trim(s tea) tea`
Remove leading and trailing whitespace.

```cursed
sus clean tea = trim(" hello ")  # "hello"
```

## Examples

### Basic String Processing

```cursed
yeet "stringz"

fr fr Parse CSV data
sus csv tea = "name,age,city"
sus fields []tea = split(csv, ",")
vibez.spill("Fields:", len_string(join(fields, " | ")))

fr fr Format user data
sus user_template tea = "User: {name}, Age: {age}"
sus formatted tea = format_template("Welcome {}", [interpolate(user_template, "name", "Alice")])
vibez.spill(formatted)
```

### Data Validation

```cursed
yeet "stringz"

slay validate_input(input tea) lit {
    ready (is_empty(input)) {
        vibez.spill("Error: Input is empty")
        damn cringe
    }
    
    ready (is_numeric(input)) {
        vibez.spill("Input is a number:", parse_int(input))
        damn based
    }
    
    ready (is_alpha(input)) {
        vibez.spill("Input is text:", to_uppercase(input))
        damn based
    }
    
    vibez.spill("Input is mixed type")
    damn based
}

validate_input("123")
validate_input("hello")  
validate_input("test123")
```

### String Building

```cursed
yeet "stringz"

slay create_report(title tea, items []tea) tea {
    sus header tea = center(title, 40, "=")
    sus separator tea = repeat_char("-", 40)
    
    sus body tea = ""
    sus i drip = 0
    bestie (i < len(items)) {
        sus line tea = pad_left(to_int(i + 1), 2, "0") + ". " + items[i]
        body = join([body, line], "\n")
        i = i + 1
    }
    
    damn join([header, body, separator], "\n")
}

sus items []tea = ["First item", "Second item", "Third item"]
sus report tea = create_report("My Report", items)
vibez.spill(report)
```

## Implementation Notes

- Pure CURSED implementation with no external dependencies
- Pattern matching used for performance-critical string operations  
- Handles edge cases like empty strings and out-of-bounds indices
- Consistent return types and error handling across all functions
- Optimized for common use cases with literal matching
