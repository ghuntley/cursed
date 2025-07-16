# CURSED String Library v2.0

Comprehensive string manipulation library for the CURSED programming language, providing full string processing capabilities equivalent to Go's strings package.

## Overview

The CURSED String Library provides a complete set of string manipulation functions including concatenation, searching, replacement, case conversion, validation, formatting, and Unicode handling. All functions are implemented in pure CURSED without external dependencies.

## Features

- ✅ **Basic Operations**: Length, concatenation, reversal
- ✅ **Case Conversion**: Upper/lower case transformations
- ✅ **String Searching**: Contains, index, starts/ends with
- ✅ **Validation**: Numeric, alphabetic, alphanumeric checks
- ✅ **Trimming**: Left, right, and full whitespace removal
- ✅ **Replacement**: First match and global replacement
- ✅ **Comparison**: Case-sensitive and case-insensitive
- ✅ **Encoding**: String to/from byte arrays
- ✅ **Substrings**: Extraction by position and length
- ✅ **Splitting/Joining**: Array conversion and reconstruction
- ✅ **Formatting**: Template-based string formatting
- ✅ **Padding**: Left and right padding with custom characters
- ✅ **Regular Expressions**: Basic pattern matching
- ✅ **Unicode Support**: Character access and manipulation
- ✅ **HTML Escaping**: Safe HTML string handling

## Quick Start

```cursed
yeet "string"

# Basic string operations
sus text tea = "Hello, World!"
sus length normie = string_length(text)
sus upper tea = string_to_upper(text)

# String searching
vibes string_contains(text, "World") {
    vibez.spill("Found 'World' in text!")
}

# String splitting and joining
sus words [tea] = string_split("apple,banana,cherry", ",")
sus joined tea = string_join(words, " | ")

# String formatting
sus args [tea] = ["CURSED", "2025"]
sus formatted tea = string_format("Welcome to {} in {}!", args)
```

## Function Reference

### Basic Operations

#### `string_length(s tea) normie`
Returns the length of a string in characters.

```cursed
string_length("hello") # Returns: 5
string_length("") # Returns: 0
```

#### `string_concat(s1 tea, s2 tea) tea`
Concatenates two strings together.

```cursed
string_concat("hello", " world") # Returns: "hello world"
```

#### `string_reverse(s tea) tea`
Reverses the characters in a string.

```cursed
string_reverse("hello") # Returns: "olleh"
```

### Case Conversion

#### `string_to_upper(s tea) tea`
Converts string to uppercase.

```cursed
string_to_upper("hello") # Returns: "HELLO"
```

#### `string_to_lower(s tea) tea`
Converts string to lowercase.

```cursed
string_to_lower("HELLO") # Returns: "hello"
```

### String Searching

#### `string_contains(haystack tea, needle tea) lit`
Checks if a string contains a substring.

```cursed
string_contains("hello world", "world") # Returns: based
string_contains("hello", "xyz") # Returns: cap
```

#### `string_index_of(haystack tea, needle tea) normie`
Returns the index of the first occurrence of a substring (-1 if not found).

```cursed
string_index_of("hello world", "world") # Returns: 6
string_index_of("hello", "xyz") # Returns: -1
```

#### `string_starts_with(s tea, prefix tea) lit`
Checks if string starts with a prefix.

```cursed
string_starts_with("hello world", "hello") # Returns: based
```

#### `string_ends_with(s tea, suffix tea) lit`
Checks if string ends with a suffix.

```cursed
string_ends_with("hello world", "world") # Returns: based
```

### String Validation

#### `string_is_numeric(s tea) lit`
Checks if string contains only numeric characters.

```cursed
string_is_numeric("123") # Returns: based
string_is_numeric("12a3") # Returns: cap
```

#### `string_is_alpha(s tea) lit`
Checks if string contains only alphabetic characters.

```cursed
string_is_alpha("hello") # Returns: based
string_is_alpha("hello123") # Returns: cap
```

#### `string_is_alphanumeric(s tea) lit`
Checks if string contains only alphanumeric characters.

```cursed
string_is_alphanumeric("hello123") # Returns: based
string_is_alphanumeric("hello!") # Returns: cap
```

### String Trimming

#### `string_trim(s tea) tea`
Removes whitespace from both ends of string.

```cursed
string_trim("  hello  ") # Returns: "hello"
```

#### `string_trim_left(s tea) tea`
Removes whitespace from the left end of string.

```cursed
string_trim_left("  hello") # Returns: "hello"
```

#### `string_trim_right(s tea) tea`
Removes whitespace from the right end of string.

```cursed
string_trim_right("hello  ") # Returns: "hello"
```

### String Replacement

#### `string_replace_first(s tea, old tea, new tea) tea`
Replaces the first occurrence of a substring.

```cursed
string_replace_first("hello world hello", "hello", "hi") 
# Returns: "hi world hello"
```

#### `string_replace_all(s tea, old tea, new tea) tea`
Replaces all occurrences of a substring.

```cursed
string_replace_all("hello world hello", "hello", "hi") 
# Returns: "hi world hi"
```

### String Comparison

#### `string_compare(s1 tea, s2 tea) normie`
Compares two strings lexicographically (returns -1, 0, or 1).

```cursed
string_compare("a", "b") # Returns: -1
string_compare("hello", "hello") # Returns: 0
```

#### `string_compare_ignore_case(s1 tea, s2 tea) normie`
Case-insensitive string comparison.

```cursed
string_compare_ignore_case("Hello", "hello") # Returns: 0
```

### String Encoding

#### `string_to_bytes(s tea) [normie]`
Converts string to array of byte values.

```cursed
string_to_bytes("hello") # Returns: [104, 101, 108, 108, 111]
```

#### `string_from_bytes(bytes [normie]) tea`
Converts array of byte values to string.

```cursed
string_from_bytes([116, 101, 115, 116]) # Returns: "test"
```

### Substring Operations

#### `string_substring(s tea, start normie, end normie) tea`
Extracts substring by start and end positions.

```cursed
string_substring("hello world", 0, 5) # Returns: "hello"
```

#### `string_substr(s tea, start normie, length normie) tea`
Extracts substring by start position and length.

```cursed
string_substr("hello world", 6, 5) # Returns: "world"
```

### String Splitting and Joining

#### `string_split(s tea, delimiter tea) [tea]`
Splits string into array by delimiter.

```cursed
string_split("a,b,c", ",") # Returns: ["a", "b", "c"]
```

#### `string_join(arr [tea], delimiter tea) tea`
Joins array of strings with delimiter.

```cursed
string_join(["hello", "world"], " ") # Returns: "hello world"
```

### String Formatting

#### `string_format(template tea, args [tea]) tea`
Formats string using template with placeholders.

```cursed
sus args [tea] = ["World"]
string_format("Hello, {}!", args) # Returns: "Hello, World!"
```

### String Padding

#### `string_pad_left(s tea, width normie, pad_char tea) tea`
Pads string on the left to specified width.

```cursed
string_pad_left("test", 8, "0") # Returns: "0000test"
```

#### `string_pad_right(s tea, width normie, pad_char tea) tea`
Pads string on the right to specified width.

```cursed
string_pad_right("test", 8, "0") # Returns: "test0000"
```

### Regular Expression Support

#### `string_match_pattern(s tea, pattern tea) lit`
Basic pattern matching support.

```cursed
string_match_pattern("123", "\\d+") # Returns: based (numeric pattern)
string_match_pattern("hello", "[a-zA-Z]+") # Returns: based (alpha pattern)
```

### Unicode Support

#### `string_char_at(s tea, index normie) sip`
Returns character at specified index.

```cursed
string_char_at("hello", 0) # Returns: 'h'
```

#### `string_char_code_at(s tea, index normie) normie`
Returns character code at specified index.

```cursed
string_char_code_at("hello", 0) # Returns: 104
```

### Advanced Utilities

#### `string_repeat(s tea, count normie) tea`
Repeats string specified number of times.

```cursed
string_repeat("ha", 3) # Returns: "hahaha"
```

#### `string_escape_html(s tea) tea`
Escapes HTML special characters.

```cursed
string_escape_html("<script>") # Returns: "&lt;script&gt;"
```

#### `string_unescape_html(s tea) tea`
Unescapes HTML entities.

```cursed
string_unescape_html("&lt;script&gt;") # Returns: "<script>"
```

## Performance Characteristics

- **Memory Efficient**: All operations use minimal memory allocation
- **Pure CURSED**: No external dependencies or FFI calls
- **Fast Execution**: Optimized for both interpretation and compilation modes
- **Unicode Ready**: Full Unicode character support
- **Thread Safe**: All functions are stateless and thread-safe

## Testing

The library includes comprehensive test coverage with 40+ test cases covering all functionality:

```bash
# Run interpretation mode tests
cargo run --bin cursed stdlib/string/test_string.csd

# Run compilation mode tests
cargo run --bin cursed -- compile stdlib/string/test_string.csd
./test_string

# Verify both modes produce identical output
test_both_modes() {
    cargo run --bin cursed stdlib/string/test_string.csd > interp_output.txt
    cargo run --bin cursed -- compile stdlib/string/test_string.csd
    ./test_string > comp_output.txt
    diff interp_output.txt comp_output.txt
}
```

## Examples

### Text Processing Pipeline

```cursed
yeet "string"

# Process user input
sus user_input tea = "  Hello, CURSED World!  "
sus cleaned tea = string_trim(user_input)
sus normalized tea = string_to_lower(cleaned)
sus words [tea] = string_split(normalized, " ")

# Format output
sus formatted tea = string_join(words, "-")
vibez.spill("Processed: " + formatted)
```

### Data Validation

```cursed
yeet "string"

slay validate_email(email tea) lit {
    vibes string_contains(email, "@") && string_contains(email, ".") {
        damn based
    }
    damn cap
}

slay validate_phone(phone tea) lit {
    sus digits tea = string_replace_all(phone, "-", "")
    digits = string_replace_all(digits, " ", "")
    damn string_is_numeric(digits) && string_length(digits) == 10
}
```

### Template Engine

```cursed
yeet "string"

slay render_template(template tea, data [tea]) tea {
    sus result tea = template
    # Replace placeholders with actual data
    bestie i := 0; i < string_length(template); i++ {
        vibes string_contains(result, "{}") {
            result = string_replace_first(result, "{}", data[i])
        }
    }
    damn result
}
```

## Best Practices

1. **Use appropriate functions**: Choose the most specific function for your use case
2. **Validate input**: Always check string validity before processing
3. **Handle edge cases**: Test with empty strings and special characters
4. **Combine operations**: Chain functions for complex string processing
5. **Consider performance**: Use substring operations for large strings efficiently

## Compatibility

- ✅ **CURSED Language**: Full compatibility with CURSED v2.0+
- ✅ **Interpretation Mode**: All functions work in interpretation mode
- ✅ **Compilation Mode**: All functions work in native compilation
- ✅ **Cross-Platform**: Works on all CURSED-supported platforms
- ✅ **Thread-Safe**: All functions are stateless and thread-safe

## Contributing

The CURSED String Library is part of the core CURSED standard library. All functions are implemented in pure CURSED for maximum portability and self-hosting capability.

For bug reports or feature requests, please refer to the main CURSED compiler repository.

## License

Licensed under the same terms as the CURSED programming language.

---

**CURSED String Library v2.0** - *Comprehensive string manipulation for the modern era* 🚀
