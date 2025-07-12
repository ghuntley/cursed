# stringz Module

The `stringz` module provides comprehensive string manipulation functions for the CURSED programming language. This module implements string searching, matching, transformation, splitting, joining, and various utility functions without external dependencies.

## Features

- **String Search & Matching**: Find substrings, check prefixes/suffixes, count occurrences
- **String Transformation**: Case conversion, trimming, padding, reversing
- **String Splitting & Joining**: Split strings by delimiters, join arrays with separators
- **String Replacement**: Replace first or all occurrences of substrings
- **Helper Functions**: Length calculation, substring extraction, whitespace detection

## Import

```cursed
yeet "stringz"
```

## Functions

### String Search and Matching

#### `Contains(s tea, substr tea) lit`
Check if string `s` contains substring `substr`.

```cursed
assert_true(stringz.Contains("hello world", "world"))
assert_false(stringz.Contains("hello world", "xyz"))
```

#### `Count(s tea, substr tea) normie`
Count the number of non-overlapping occurrences of `substr` in `s`.

```cursed
assert_eq_int(stringz.Count("hello world", "l"), 3)
assert_eq_int(stringz.Count("hello world", "o"), 2)
```

#### `HasPrefix(s tea, prefix tea) lit`
Check if string `s` starts with `prefix`.

```cursed
assert_true(stringz.HasPrefix("hello world", "hello"))
assert_false(stringz.HasPrefix("hello world", "world"))
```

#### `HasSuffix(s tea, suffix tea) lit`
Check if string `s` ends with `suffix`.

```cursed
assert_true(stringz.HasSuffix("hello world", "world"))
assert_false(stringz.HasSuffix("hello world", "hello"))
```

#### `IndexOf(s tea, substr tea) normie`
Find the first index of `substr` in `s`, returns -1 if not found.

```cursed
assert_eq_int(stringz.IndexOf("hello world", "world"), 6)
assert_eq_int(stringz.IndexOf("hello world", "xyz"), -1)
```

#### `LastIndexOf(s tea, substr tea) normie`
Find the last index of `substr` in `s`, returns -1 if not found.

```cursed
assert_eq_int(stringz.LastIndexOf("hello hello", "hello"), 6)
```

### String Transformation

#### `ToLower(s tea) tea`
Convert string to lowercase.

```cursed
assert_eq_string(stringz.ToLower("HELLO WORLD"), "hello world")
assert_eq_string(stringz.ToLower("Hello World"), "hello world")
```

#### `ToUpper(s tea) tea`
Convert string to uppercase.

```cursed
assert_eq_string(stringz.ToUpper("hello world"), "HELLO WORLD")
assert_eq_string(stringz.ToUpper("Hello World"), "HELLO WORLD")
```

#### `Trim(s tea) tea`
Remove leading and trailing whitespace.

```cursed
assert_eq_string(stringz.Trim("  hello world  "), "hello world")
assert_eq_string(stringz.Trim("\t\nhello\t\n"), "hello")
```

#### `TrimLeft(s tea) tea`
Remove leading whitespace.

```cursed
assert_eq_string(stringz.TrimLeft("  hello world  "), "hello world  ")
```

#### `TrimRight(s tea) tea`
Remove trailing whitespace.

```cursed
assert_eq_string(stringz.TrimRight("  hello world  "), "  hello world")
```

#### `Reverse(s tea) tea`
Reverse the string.

```cursed
assert_eq_string(stringz.Reverse("hello"), "olleh")
assert_eq_string(stringz.Reverse("world"), "dlrow")
```

#### `PadLeft(s tea, width normie, pad sip) tea`
Pad string on the left with the specified character until it reaches the target width.

```cursed
assert_eq_string(stringz.PadLeft("hello", 8, ' '), "   hello")
```

#### `PadRight(s tea, width normie, pad sip) tea`
Pad string on the right with the specified character until it reaches the target width.

```cursed
assert_eq_string(stringz.PadRight("hello", 8, ' '), "hello   ")
```

### String Splitting and Joining

#### `Split(s tea, sep tea) [tea]`
Split string by separator into an array of strings.

```cursed
sus words [tea] = stringz.Split("hello,world,test", ",")
assert_eq_int(len(words), 3)
assert_eq_string(words[0], "hello")
```

#### `Join(parts [tea], sep tea) tea`
Join array of strings with separator.

```cursed
sus parts [tea] = ["hello", "world", "test"]
assert_eq_string(stringz.Join(parts, ","), "hello,world,test")
```

#### `Repeat(s tea, count normie) tea`
Repeat string `count` times.

```cursed
assert_eq_string(stringz.Repeat("hello", 3), "hellohellohello")
assert_eq_string(stringz.Repeat("a", 5), "aaaaa")
```

### String Replacement

#### `Replace(s tea, old tea, new tea) tea`
Replace the first occurrence of `old` with `new` in string `s`.

```cursed
assert_eq_string(stringz.Replace("hello world", "world", "CURSED"), "hello CURSED")
assert_eq_string(stringz.Replace("hello hello", "hello", "hi"), "hi hello")
```

#### `ReplaceAll(s tea, old tea, new tea) tea`
Replace all occurrences of `old` with `new` in string `s`.

```cursed
assert_eq_string(stringz.ReplaceAll("hello hello", "hello", "hi"), "hi hi")
```

### Helper Functions

#### `Length(s tea) normie`
Get the length of string `s`.

```cursed
assert_eq_int(stringz.Length("hello"), 5)
assert_eq_int(stringz.Length(""), 0)
```

#### `Substring(s tea, start normie, length normie) tea`
Extract substring starting at `start` with specified `length`.

```cursed
assert_eq_string(stringz.Substring("hello world", 0, 5), "hello")
assert_eq_string(stringz.Substring("hello world", 6, 5), "world")
```

#### `IsWhitespace(ch sip) lit`
Check if character is whitespace (space, tab, newline, carriage return).

```cursed
assert_true(stringz.IsWhitespace(' '))
assert_true(stringz.IsWhitespace('\t'))
assert_false(stringz.IsWhitespace('a'))
```

## Usage Examples

### Basic String Operations

```cursed
yeet "stringz"
yeet "testz"

# String searching
sus text tea = "The quick brown fox jumps over the lazy dog"
assert_true(stringz.Contains(text, "quick"))
assert_eq_int(stringz.Count(text, "the"), 2)
assert_true(stringz.HasPrefix(text, "The"))
assert_true(stringz.HasSuffix(text, "dog"))

# String transformation
sus upper tea = stringz.ToUpper(text)
sus lower tea = stringz.ToLower(text)
sus trimmed tea = stringz.Trim("  " + text + "  ")
```

### String Splitting and Processing

```cursed
# Split CSV data
sus csv_data tea = "name,age,city"
sus fields [tea] = stringz.Split(csv_data, ",")
assert_eq_int(len(fields), 3)
assert_eq_string(fields[0], "name")

# Join with different separator
sus pipe_separated tea = stringz.Join(fields, "|")
assert_eq_string(pipe_separated, "name|age|city")
```

### Complex String Processing

```cursed
# Process user input
sus user_input tea = "  Hello, World! This is a Test.  "
sus processed tea = stringz.Trim(user_input)
processed = stringz.ToLower(processed)
processed = stringz.ReplaceAll(processed, "!", "")
processed = stringz.ReplaceAll(processed, ",", "")

sus words [tea] = stringz.Split(processed, " ")
sus clean_text tea = stringz.Join(words, "_")
# Result: "hello_world_this_is_a_test"
```

### String Validation

```cursed
# Check string properties
sus email tea = "user@example.com"
assert_true(stringz.Contains(email, "@"))
assert_true(stringz.HasSuffix(email, ".com"))
assert_eq_int(stringz.Count(email, "@"), 1)

# Validate format
sus is_valid lit = stringz.Contains(email, "@") && 
                  stringz.Contains(email, ".") &&
                  stringz.Count(email, "@") == 1
```

## Testing

Run the comprehensive test suite:

```bash
# Test in interpretation mode
cargo run --bin cursed stdlib/stringz/test_stringz.csd

# Test in compilation mode
cargo run --bin cursed -- compile stdlib/stringz/test_stringz.csd
./test_stringz

# Test with stdlib test runner
cargo run --bin cursed test --filter stringz
```

## Implementation Details

- **Pure CURSED**: No external dependencies or FFI calls
- **Performance**: Efficient string algorithms with minimal memory allocation
- **Unicode Support**: Basic ASCII character handling with extensible design
- **Memory Safe**: Proper bounds checking and memory management
- **Test Coverage**: Comprehensive test suite with edge cases and performance tests

## Compatibility

- **Interpretation Mode**: Full compatibility with CURSED interpreter
- **Compilation Mode**: Full compatibility with LLVM native compilation
- **Cross-Platform**: Works on all supported CURSED platforms
- **Thread-Safe**: All functions are stateless and thread-safe

## Contributing

When adding new string functions:

1. Follow the existing naming conventions
2. Add comprehensive tests in `test_stringz.csd`
3. Update this README with documentation
4. Ensure pure CURSED implementation (no FFI)
5. Test both interpretation and compilation modes

## License

This module is part of the CURSED programming language standard library.
