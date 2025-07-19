# Enhanced StringZ Module

**Pure CURSED String Processing Library**

This module provides a comprehensive string manipulation library implemented entirely in CURSED, eliminating all FFI dependencies. It's migrated from the Rust stdlib modules (`stringz.rs`, `string/`, `glyph_gang/string_ops.rs`) and provides enhanced functionality with performance optimizations.

## Overview

The Enhanced StringZ module offers 50+ string operations across 8 functional categories:

- **Core Operations**: Length, concatenation, reversal, emptiness checks
- **Search & Matching**: Contains, index finding, occurrence counting
- **Case Conversion**: Upper, lower, title case transformations  
- **Trimming & Whitespace**: Leading/trailing whitespace removal
- **Slicing & Substrings**: Substring extraction with bounds checking
- **Splitting & Joining**: String splitting and array joining
- **Replacement**: Pattern replacement with single/all occurrences
- **Validation**: Type checking (numeric, alphabetic, alphanumeric)
- **Advanced Operations**: Common prefix/suffix, distance calculation
- **Format & Encoding**: Special character escaping/unescaping

## Migration Details

### Migrated From:
- `src/stdlib/stringz.rs` - Main string operations
- `src/stdlib/string/mod.rs` - String module coordination
- `src/stdlib/string/core.rs` - Core string processing
- `src/stdlib/string/search.rs` - Search functionality
- `src/stdlib/string/transform.rs` - Transformation operations
- `src/stdlib/string/split_join.rs` - Split/join utilities
- `src/stdlib/string/validation.rs` - Validation functions
- `src/stdlib/string/format.rs` - Format operations
- `src/stdlib/string/regex.rs` - Regular expression support
- `src/stdlib/glyph_gang/string_ops.rs` - Advanced string operations

### FFI Elimination Benefits:
- ✅ **Zero External Dependencies**: No C library calls
- ✅ **Full Self-Hosting**: Enables complete CURSED self-compilation
- ✅ **Enhanced Portability**: Works on any platform supporting CURSED
- ✅ **Memory Safety**: CURSED's type system prevents buffer overflows
- ✅ **Performance**: Optimized algorithms with CURSED language features

## Core Functions

### String Properties
```cursed
string_length(s tea) normie                    # Get string length
string_is_empty(s tea) lit                     # Check if empty
string_is_numeric(s tea) lit                   # Check if all digits
string_is_alpha(s tea) lit                     # Check if all letters
string_is_alphanumeric(s tea) lit              # Check if letters/digits
string_is_lower(s tea) lit                     # Check if lowercase
string_is_upper(s tea) lit                     # Check if uppercase
```

### String Manipulation
```cursed
string_concat(a tea, b tea) tea                # Concatenate strings
string_reverse(s tea) tea                      # Reverse string
string_repeat(s tea, count normie) tea         # Repeat string
string_to_lower(s tea) tea                     # Convert to lowercase
string_to_upper(s tea) tea                     # Convert to uppercase
string_to_title_case(s tea) tea                # Convert to title case
```

### Search Operations
```cursed
string_contains(s tea, substr tea) lit         # Check if contains substring
string_index_of(s tea, substr tea) normie      # Find first occurrence index
string_last_index_of(s tea, substr tea) normie # Find last occurrence index
string_count_occurrences(s tea, substr tea) normie # Count occurrences
string_has_prefix(s tea, prefix tea) lit       # Check if starts with
string_has_suffix(s tea, suffix tea) lit       # Check if ends with
```

### Substring Operations
```cursed
string_substring(s tea, start normie, length normie) tea  # Extract substring
string_slice(s tea, start normie, end normie) tea         # Extract slice
string_trim(s tea) tea                                     # Trim whitespace
string_trim_left(s tea) tea                               # Trim leading
string_trim_right(s tea) tea                              # Trim trailing
```

### Split and Join
```cursed
string_split(s tea, separator tea) [tea]       # Split into array
string_split_lines(s tea) [tea]                # Split by line endings
string_join(parts [tea], separator tea) tea    # Join array with separator
```

### Replacement
```cursed
string_replace_first(s tea, old tea, new tea) tea     # Replace first match
string_replace_all(s tea, old tea, new tea) tea       # Replace all matches  
string_replace_at_index(s tea, index normie, replacement tea) tea # Replace at index
```

### Padding Operations
```cursed
string_pad_left(s tea, width normie, pad_char sip) tea    # Pad left
string_pad_right(s tea, width normie, pad_char sip) tea   # Pad right
string_center(s tea, width normie, pad_char sip) tea      # Center string
```

### Advanced Operations
```cursed
string_common_prefix(a tea, b tea) tea         # Find common prefix
string_common_suffix(a tea, b tea) tea         # Find common suffix
string_distance_levenshtein(a tea, b tea) normie # Calculate edit distance
string_escape_special_chars(s tea) tea         # Escape special characters
string_unescape_special_chars(s tea) tea       # Unescape special characters
```

## Compatibility Layer

All legacy function names are preserved for backward compatibility:

```cursed
Contains(s tea, substr tea) lit                # Alias for string_contains
HasPrefix(s tea, prefix tea) lit               # Alias for string_has_prefix
HasSuffix(s tea, suffix tea) lit               # Alias for string_has_suffix
ToLower(s tea) tea                             # Alias for string_to_lower
ToUpper(s tea) tea                             # Alias for string_to_upper
Trim(s tea) tea                                # Alias for string_trim
Replace(s tea, old tea, new tea) tea           # Alias for string_replace_first
ReplaceAll(s tea, old tea, new tea) tea        # Alias for string_replace_all
Split(s tea, sep tea) [tea]                    # Alias for string_split
Join(parts [tea], sep tea) tea                 # Alias for string_join
Length(s tea) normie                           # Alias for string_length
IndexOf(s tea, substr tea) normie              # Alias for string_index_of
# ... and 20+ more compatibility aliases
```

## Usage Examples

### Basic String Operations
```cursed
yeet "stringz"

# String properties
sus text tea = "Hello, CURSED!"
vibez.spill(string_length(text))               # Output: 14
vibez.spill(string_is_empty(""))               # Output: based (true)
vibez.spill(string_is_alpha("Hello"))          # Output: cap (false - punctuation)

# Case conversion
vibez.spill(string_to_upper("hello"))          # Output: HELLO
vibez.spill(string_to_lower("WORLD"))          # Output: world
vibez.spill(string_to_title_case("hello world")) # Output: Hello World
```

### Search and Matching
```cursed
yeet "stringz"

sus text tea = "The quick brown fox jumps over the lazy dog"

# Search operations
vibez.spill(string_contains(text, "fox"))      # Output: based (true)
vibez.spill(string_index_of(text, "fox"))      # Output: 16
vibez.spill(string_count_occurrences(text, "the")) # Output: 2

# Prefix/suffix checks
vibez.spill(string_has_prefix(text, "The"))    # Output: based (true)
vibez.spill(string_has_suffix(text, "dog"))    # Output: based (true)
```

### String Manipulation
```cursed
yeet "stringz"

# Trimming and cleaning
sus messy tea = "  \t  hello world  \n  "
vibez.spill("'" + string_trim(messy) + "'")    # Output: 'hello world'

# Substring extraction
sus text tea = "Hello, CURSED!"
vibez.spill(string_substring(text, 7, 6))      # Output: CURSED
vibez.spill(string_slice(text, 0, 5))          # Output: Hello

# Replacement
sus original tea = "Hello, World!"
vibez.spill(string_replace_all(original, "l", "X")) # Output: HeXXo, WorXd!
```

### Split and Join Operations
```cursed
yeet "stringz"

# Splitting strings
sus csv tea = "apple,banana,cherry,date"
sus fruits [tea] = string_split(csv, ",")
vibez.spill(len(fruits))                       # Output: 4
vibez.spill(fruits[0])                         # Output: apple

# Joining arrays
sus words [tea] = ["Hello", "beautiful", "world"]
vibez.spill(string_join(words, " "))           # Output: Hello beautiful world
vibez.spill(string_join(words, "-"))           # Output: Hello-beautiful-world
```

### Advanced Operations
```cursed
yeet "stringz"

# Common prefix/suffix
vibez.spill(string_common_prefix("testing", "tester"))    # Output: test
vibez.spill(string_common_suffix("running", "jumping"))   # Output: ing

# String distance (simplified Levenshtein)
vibez.spill(string_distance_levenshtein("hello", "hallo")) # Output: 1

# Padding operations
vibez.spill("'" + string_pad_left("test", 8, ' ') + "'")  # Output: '    test'
vibez.spill("'" + string_center("hi", 6, 'X') + "'")      # Output: 'XXhiXX'
```

## Performance Considerations

### Optimization Features:
- **Early Exit**: Search operations exit immediately when matches are found
- **Bounds Checking**: Safe array access prevents runtime errors
- **Memory Efficient**: Minimal string copying in operations
- **Algorithm Selection**: Optimal algorithms for each operation type

### Performance Tips:
1. **Batch Operations**: Combine multiple operations when possible
2. **Avoid Repeated Searches**: Cache index results for multiple operations
3. **Use Appropriate Functions**: Choose specific functions over generic ones
4. **String Building**: Use concatenation carefully for large strings

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/stringz/test_stringz_comprehensive.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/stringz/test_stringz_comprehensive.csd
./test_stringz_comprehensive

# Both-mode verification
test_both_modes() {
    cargo run --bin cursed stdlib/stringz/test_stringz_comprehensive.csd > interp.txt
    cargo run --bin cursed -- compile stdlib/stringz/test_stringz_comprehensive.csd
    ./test_stringz_comprehensive > comp.txt
    diff interp.txt comp.txt
}
```

## Test Coverage

The comprehensive test suite covers:

- ✅ **50+ Core Functions**: All string operations tested
- ✅ **Edge Cases**: Empty strings, boundary conditions, invalid inputs
- ✅ **Unicode Support**: Basic Unicode character handling
- ✅ **Performance**: Large string operations and stress testing
- ✅ **Compatibility**: All legacy API functions verified
- ✅ **Both Modes**: Interpretation and compilation mode verification

## Integration

Import the enhanced stringz module in your CURSED programs:

```cursed
# Import the enhanced string module
yeet "stringz"

# Use enhanced functions
sus result tea = string_to_upper("hello world")
vibez.spill(result)  # Output: HELLO WORLD

# Or use compatibility aliases
sus result2 tea = ToUpper("hello world") 
vibez.spill(result2) # Output: HELLO WORLD
```

## Migration Notes

### From Rust Implementation:
1. **Function Signatures**: All function signatures preserved with CURSED types
2. **Error Handling**: Converted from Result<T, E> to direct value returns with bounds checking
3. **Memory Management**: CURSED's GC eliminates manual memory management
4. **Unicode Support**: Basic Unicode support maintained, full support planned for future versions

### Breaking Changes:
- **None**: Full backward compatibility maintained
- **Enhanced**: Additional functions provide more comprehensive string processing
- **Performance**: Optimized algorithms for better performance in interpretation mode

## Future Enhancements

Planned improvements for next versions:

1. **Full Unicode Support**: Complete Unicode normalization and codepoint handling
2. **Regular Expressions**: Pure CURSED regex engine implementation
3. **Localization**: Locale-aware string operations
4. **Pattern Matching**: Advanced pattern matching capabilities
5. **Stream Processing**: Large string processing with streaming operations

## Contributing

To contribute to the StringZ module:

1. Add new functions following the `string_*` naming convention
2. Include comprehensive error checking and bounds validation
3. Add corresponding test cases in `test_stringz_comprehensive.csd`
4. Update this documentation with function descriptions and examples
5. Ensure both interpretation and compilation modes work correctly

---

**Enhanced StringZ Module** - Pure CURSED String Processing
*Migrated from Rust stdlib modules with FFI elimination and performance optimizations*
