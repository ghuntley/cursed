# StringZ Module - Complete String Operations Library

A comprehensive string manipulation library for the CURSED programming language, providing high-performance, memory-efficient string operations with full Unicode awareness.

## Overview

StringZ is a pure CURSED implementation providing enterprise-grade string processing capabilities:

- **Search & Match**: Advanced pattern matching and substring operations
- **Manipulation**: Comprehensive text transformation and modification
- **Validation**: Complete character classification and format checking  
- **Case Conversion**: Support for snake_case, camelCase, PascalCase, kebab-case
- **Splitting & Joining**: Flexible string parsing and concatenation
- **Padding & Alignment**: Text formatting and layout utilities
- **Performance**: Optimized algorithms with minimal memory allocation

## Quick Start

```cursed
yeet "stringz"

# Basic string operations
sus text tea = "Hello, World!"
vibez.spill(stringz.ToLower(text))        # "hello, world!"
vibez.spill(stringz.Contains(text, "World"))  # based (true)

# String manipulation
sus words [tea] = stringz.Split("a,b,c", ",")
sus joined tea = stringz.Join(words, " | ")  # "a | b | c"

# Case conversion
sus snake tea = stringz.ToSnakeCase("XMLHttpRequest")  # "xml_http_request"
sus camel tea = stringz.ToCamelCase(snake)             # "xmlHttpRequest"
```

## Function Reference

### String Search Functions

#### Contains(s, substr tea) lit
Check if string contains substring.
```cursed
stringz.Contains("hello world", "world")  # based
stringz.Contains("test", "")              # based (empty string always contained)
```

#### ContainsAny(s, chars tea) lit  
Check if string contains any character from chars.
```cursed
stringz.ContainsAny("hello", "aeiou")     # based (contains vowels)
stringz.ContainsAny("bcdfg", "aeiou")     # cap (no vowels)
```

#### Count(s, substr tea) normie
Count non-overlapping occurrences of substring.
```cursed
stringz.Count("hello world", "l")         # 3
stringz.Count("aaa", "aa")               # 1 (non-overlapping)
```

#### HasPrefix(s, prefix tea) lit
Check if string starts with prefix.
```cursed
stringz.HasPrefix("hello world", "hello") # based
stringz.HasPrefix("hello world", "world") # cap
```

#### HasSuffix(s, suffix tea) lit
Check if string ends with suffix.
```cursed
stringz.HasSuffix("hello world", "world") # based
stringz.HasSuffix("hello world", "hello") # cap
```

#### Index(s, substr tea) normie
Find first occurrence index (-1 if not found).
```cursed
stringz.Index("hello world", "world")     # 6
stringz.Index("hello world", "xyz")       # -1
```

#### LastIndex(s, substr tea) normie
Find last occurrence index (-1 if not found).
```cursed
stringz.LastIndex("hello world hello", "hello")  # 12
stringz.LastIndex("hello world", "xyz")          # -1
```

### String Manipulation Functions

#### ToLower(s tea) tea
Convert string to lowercase.
```cursed
stringz.ToLower("HELLO World")           # "hello world"
stringz.ToLower("123ABC")                # "123abc"
```

#### ToUpper(s tea) tea
Convert string to uppercase.
```cursed
stringz.ToUpper("hello World")           # "HELLO WORLD"
stringz.ToUpper("123abc")                # "123ABC"
```

#### TrimSpace(s tea) tea
Remove leading and trailing whitespace.
```cursed
stringz.TrimSpace("  hello world  ")     # "hello world"
stringz.TrimSpace("\t\nhello\t\n")       # "hello"
```

#### Trim(s, cutset tea) tea
Remove leading and trailing characters from cutset.
```cursed
stringz.Trim("xyzhelloxyz", "xyz")       # "hello"
stringz.Trim("   hello   ", " ")         # "hello"
```

#### TrimLeft(s, cutset tea) tea / TrimRight(s, cutset tea) tea
Remove characters from one side only.
```cursed
stringz.TrimLeft("xyzhello", "xyz")      # "hello"
stringz.TrimRight("helloxyz", "xyz")     # "hello"
```

#### TrimPrefix(s, prefix tea) tea / TrimSuffix(s, suffix tea) tea
Remove specific prefix or suffix if present.
```cursed
stringz.TrimPrefix("hello world", "hello ")  # "world"
stringz.TrimSuffix("hello world", " world")  # "hello"
```

#### Replace(s, old, new tea) tea
Replace first occurrence of old with new.
```cursed
stringz.Replace("hello hello", "hello", "hi")  # "hi hello"
```

#### ReplaceAll(s, old, new tea) tea
Replace all occurrences of old with new.
```cursed
stringz.ReplaceAll("hello hello", "hello", "hi")  # "hi hi"
```

#### Repeat(s tea, count normie) tea
Repeat string count times.
```cursed
stringz.Repeat("abc", 3)                 # "abcabcabc"
stringz.Repeat("x", 5)                   # "xxxxx"
```

### String Splitting and Joining

#### Split(s, sep tea) [tea]
Split string by separator.
```cursed
stringz.Split("a,b,c,d", ",")           # ["a", "b", "c", "d"]
stringz.Split("abc", "")                # ["a", "b", "c"] (split into chars)
```

#### SplitN(s, sep tea, n normie) [tea]
Split string with maximum number of parts.
```cursed
stringz.SplitN("a,b,c,d", ",", 2)       # ["a", "b,c,d"]
```

#### Join(parts [tea], sep tea) tea
Join string array with separator.
```cursed
stringz.Join(["hello", "world"], " ")   # "hello world"
stringz.Join(["a", "b", "c"], ",")      # "a,b,c"
```

#### Fields(s tea) [tea]
Split string by whitespace.
```cursed
stringz.Fields("  hello   world  ")     # ["hello", "world"]
```

### String Utility Functions

#### Reverse(s tea) tea
Reverse string.
```cursed
stringz.Reverse("hello")                 # "olleh"
stringz.Reverse("abc")                   # "cba"
```

#### PadLeft(s tea, width normie, pad tea) tea
Pad string on the left.
```cursed
stringz.PadLeft("hello", 10, "0")        # "00000hello"
stringz.PadLeft("hello", 8, "xy")        # "xyxhello"
```

#### PadRight(s tea, width normie, pad tea) tea
Pad string on the right.
```cursed
stringz.PadRight("hello", 10, "0")       # "hello00000"
stringz.PadRight("hello", 8, "xy")       # "helloXYX"
```

#### Center(s tea, width normie, pad tea) tea
Center string in field of given width.
```cursed
stringz.Center("hello", 11, " ")         # "   hello   "
stringz.Center("test", 10, "-")          # "---test---"
```

### String Validation Functions

#### IsEmpty(s tea) lit
Check if string is empty.
```cursed
stringz.IsEmpty("")                      # based
stringz.IsEmpty(" ")                     # cap
```

#### IsBlank(s tea) lit
Check if string is empty or contains only whitespace.
```cursed
stringz.IsBlank("")                      # based
stringz.IsBlank("   ")                   # based
stringz.IsBlank(" hello ")               # cap
```

#### IsNumeric(s tea) lit
Check if string contains only numeric characters.
```cursed
stringz.IsNumeric("12345")               # based
stringz.IsNumeric("123a")                # cap
```

#### IsAlpha(s tea) lit
Check if string contains only alphabetic characters.
```cursed
stringz.IsAlpha("hello")                 # based
stringz.IsAlpha("hello123")              # cap
```

#### IsAlphanumeric(s tea) lit
Check if string contains only alphanumeric characters.
```cursed
stringz.IsAlphanumeric("hello123")       # based
stringz.IsAlphanumeric("hello world")    # cap
```

### Advanced String Functions

#### Before(s, sep tea) tea / After(s, sep tea) tea
Get portion before/after first separator.
```cursed
stringz.Before("name:value", ":")        # "name"
stringz.After("name:value", ":")         # "value"
```

#### BeforeLast(s, sep tea) tea / AfterLast(s, sep tea) tea
Get portion before/after last separator.
```cursed
stringz.BeforeLast("a:b:c", ":")         # "a:b"
stringz.AfterLast("a:b:c", ":")          # "c"
```

#### Truncate(s tea, length normie) tea
Truncate string to specified length.
```cursed
stringz.Truncate("hello world", 5)       # "hello"
```

#### TruncateWithEllipsis(s tea, length normie) tea
Truncate string with ellipsis.
```cursed
stringz.TruncateWithEllipsis("hello world", 8)  # "hello..."
```

### Case Conversion Functions

#### ToSnakeCase(s tea) tea
Convert to snake_case.
```cursed
stringz.ToSnakeCase("HelloWorld")        # "hello_world"
stringz.ToSnakeCase("XMLHttpRequest")    # "xml_http_request"
```

#### ToCamelCase(s tea) tea
Convert to camelCase.
```cursed
stringz.ToCamelCase("hello_world")       # "helloWorld"
stringz.ToCamelCase("XML_HTTP_REQUEST")  # "xmlHttpRequest"
```

#### ToPascalCase(s tea) tea
Convert to PascalCase.
```cursed
stringz.ToPascalCase("hello_world")      # "HelloWorld"
stringz.ToPascalCase("xml_http_request") # "XmlHttpRequest"
```

#### ToKebabCase(s tea) tea
Convert to kebab-case.
```cursed
stringz.ToKebabCase("HelloWorld")        # "hello-world"
stringz.ToKebabCase("XMLHttpRequest")    # "xml-http-request"
```

### Helper Functions

#### Length(s tea) normie
Get string length.
```cursed
stringz.Length("hello")                  # 5
stringz.Length("")                       # 0
```

#### Substring(s tea, start normie, length normie) tea
Extract substring with bounds checking.
```cursed
stringz.Substring("hello world", 0, 5)   # "hello"
stringz.Substring("hello world", 6, 5)   # "world"
```

### Alias Functions

For compatibility with other string libraries:

- `StartsWith(s, prefix tea) lit` - alias for `HasPrefix`
- `EndsWith(s, suffix tea) lit` - alias for `HasSuffix`  
- `IndexOf(s, substr tea) normie` - alias for `Index`
- `LastIndexOf(s, substr tea) normie` - alias for `LastIndex`

## Performance Characteristics

### Time Complexity
- **Search Operations**: O(n*m) for pattern matching (n = string length, m = pattern length)
- **Case Conversion**: O(n) linear scan
- **Splitting**: O(n) with additional O(k) for k splits
- **Joining**: O(n) where n is total character count
- **Padding**: O(n + p) where p is padding length

### Memory Usage
- **Immutable Operations**: All functions return new strings, preserving input
- **Minimal Allocation**: Efficient string building with single allocation where possible
- **Bounds Checking**: Safe substring operations prevent buffer overflows

### Optimization Features
- **Early Exit**: Search functions exit immediately when match found/impossible
- **Non-overlapping Counting**: Count function skips matched portions for efficiency
- **Bounds Validation**: Substring operations include comprehensive bounds checking
- **Character Classification**: Fast ASCII character classification

## Usage Examples

### Text Processing Pipeline
```cursed
yeet "stringz"

# Process user input
sus input tea = "  Hello, World!  "
sus cleaned tea = stringz.TrimSpace(input)           # "Hello, World!"
sus lower tea = stringz.ToLower(cleaned)             # "hello, world!"
sus snake tea = stringz.ToSnakeCase(lower)           # "hello_world"

# Extract parts
sus parts [tea] = stringz.Split(snake, "_")          # ["hello", "world"]
sus first tea = parts[0]                             # "hello"
sus capitalized tea = stringz.ToPascalCase(first)    # "Hello"
```

### Data Validation
```cursed
slay ValidateEmail(email tea) lit {
    # Basic email validation
    highkey stringz.IsEmpty(email) {
        damn cap
    }
    
    sus at_index normie = stringz.Index(email, "@")
    highkey at_index == -1 || at_index == 0 {
        damn cap
    }
    
    sus local tea = stringz.Before(email, "@")
    sus domain tea = stringz.After(email, "@")
    
    damn !stringz.IsEmpty(local) && 
         !stringz.IsEmpty(domain) && 
         stringz.Contains(domain, ".")
}
```

### Text Formatting
```cursed
slay FormatTitle(title tea, width normie) tea {
    sus trimmed tea = stringz.TrimSpace(title)
    sus upper tea = stringz.ToUpper(trimmed)
    
    highkey stringz.Length(upper) > width {
        damn stringz.TruncateWithEllipsis(upper, width)
    }
    
    damn stringz.Center(upper, width, " ")
}
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/stringz/test_stringz.csd
```

The test suite includes:
- **Unit Tests**: All individual functions tested with edge cases
- **Integration Tests**: Complex operations combining multiple functions  
- **Performance Tests**: Large string operations and stress testing
- **Edge Case Coverage**: Empty strings, single characters, special cases
- **Unicode Support**: Extended ASCII and Unicode-like character handling

## Implementation Notes

### Design Principles
- **Pure CURSED**: No FFI dependencies, fully self-contained
- **Memory Safety**: Comprehensive bounds checking and validation
- **Performance**: Optimized algorithms with minimal allocations
- **Compatibility**: Familiar API similar to Go/Python string libraries
- **Consistency**: Uniform error handling and edge case behavior

### Character Encoding
- **ASCII Focus**: Optimized for ASCII character operations
- **Unicode Aware**: Handles extended ASCII and basic Unicode operations
- **Null Termination**: Compatible with C-style string handling
- **Bounds Safety**: All character access includes bounds validation

### Error Handling
- **Graceful Degradation**: Functions handle edge cases gracefully
- **Consistent Behavior**: Empty strings and invalid inputs handled uniformly
- **Bounds Checking**: All array/string access operations are bounds-checked
- **Safe Defaults**: Functions return safe default values for invalid inputs

## Contributing

When extending StringZ:

1. **Follow Patterns**: Use existing function patterns for consistency
2. **Add Tests**: Include comprehensive tests for new functions
3. **Document**: Add clear documentation with examples
4. **Validate**: Ensure proper bounds checking and edge case handling
5. **Performance**: Consider algorithmic complexity and memory usage

## Version History

- **v2.0**: Complete rewrite with advanced features
  - Added case conversion functions
  - Enhanced splitting and joining
  - Comprehensive validation functions
  - Advanced string utilities
  - Full test coverage

- **v1.0**: Initial implementation
  - Basic string operations
  - Simple search and replace
  - Minimal test coverage
