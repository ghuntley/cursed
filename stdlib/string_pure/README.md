# Pure CURSED String Library

A complete string manipulation library implemented in pure CURSED without any FFI dependencies.

## Overview

This module provides comprehensive string manipulation functions using only native CURSED language features. It eliminates external dependencies and provides a self-contained string processing library.

## Features

### String Manipulation
- `string_len(s tea) normie` - Get string length
- `string_is_empty(s tea) lit` - Check if string is empty
- `string_trim(s tea) tea` - Trim whitespace from both ends
- `string_trim_start(s tea) tea` - Trim whitespace from start
- `string_trim_end(s tea) tea` - Trim whitespace from end
- `string_reverse(s tea) tea` - Reverse string

### Case Conversion
- `string_to_upper(s tea) tea` - Convert to uppercase
- `string_to_lower(s tea) tea` - Convert to lowercase
- `string_capitalize(s tea) tea` - Capitalize first letter

### String Searching
- `string_contains(s tea, substr tea) lit` - Check if string contains substring
- `string_starts_with(s tea, prefix tea) lit` - Check if string starts with prefix
- `string_ends_with(s tea, suffix tea) lit` - Check if string ends with suffix
- `string_index_of(s tea, substr tea) normie` - Find first occurrence index
- `string_last_index_of(s tea, substr tea) normie` - Find last occurrence index
- `string_count_occurrences(s tea, substr tea) normie` - Count occurrences

### String Slicing
- `string_slice(s tea, start normie, end normie) tea` - Extract substring by indices
- `string_substring(s tea, start normie, length normie) tea` - Extract substring by length
- `string_char_at(s tea, index normie) tea` - Get character at index

### String Splitting
- `string_split(s tea, delimiter tea) [tea]` - Split by delimiter
- `string_split_lines(s tea) [tea]` - Split by newlines
- `string_split_whitespace(s tea) [tea]` - Split by whitespace

### String Replacement
- `string_replace(s tea, old tea, new tea) tea` - Replace first occurrence
- `string_replace_all(s tea, old tea, new tea) tea` - Replace all occurrences
- `string_repeat(s tea, count normie) tea` - Repeat string

### String Padding
- `string_pad_left(s tea, length normie, pad_char tea) tea` - Pad left
- `string_pad_right(s tea, length normie, pad_char tea) tea` - Pad right
- `string_pad_center(s tea, length normie, pad_char tea) tea` - Pad center

### String Validation
- `string_is_numeric(s tea) lit` - Check if string is numeric
- `string_is_alpha(s tea) lit` - Check if string is alphabetic
- `string_is_alphanumeric(s tea) lit` - Check if string is alphanumeric
- `string_is_whitespace(s tea) lit` - Check if string is whitespace
- `string_is_ascii(s tea) lit` - Check if string is ASCII

### Type Conversion
- `string_to_int(s tea) normie` - Convert string to integer
- `string_to_float(s tea) meal` - Convert string to float
- `string_to_bool(s tea) lit` - Convert string to boolean
- `string_from_int(i normie) tea` - Convert integer to string
- `string_from_float(f meal) tea` - Convert float to string
- `string_from_bool(b lit) tea` - Convert boolean to string

### String Utilities
- `string_join(strings [tea], separator tea) tea` - Join strings
- `string_hash(s tea) normie` - Calculate string hash
- `string_levenshtein_distance(s1 tea, s2 tea) normie` - Calculate edit distance
- `string_similarity(s1 tea, s2 tea) meal` - Calculate similarity

### Regular Expressions
- `regex_match(pattern tea, text tea) lit` - Match regex pattern
- `regex_find(pattern tea, text tea) tea` - Find first match
- `regex_find_all(pattern tea, text tea) [tea]` - Find all matches
- `regex_replace(pattern tea, text tea, replacement tea) tea` - Replace matches
- `regex_split(pattern tea, text tea) [tea]` - Split by regex

### Encoding/Decoding
- `string_to_bytes(s tea) [byte]` - Convert string to bytes
- `string_from_bytes(bytes [byte]) tea` - Convert bytes to string
- `string_escape(s tea) tea` - Escape special characters
- `string_unescape(s tea) tea` - Unescape special characters

## Usage Examples

```cursed
yeet "string_pure"

// Basic string operations
sus text tea = "Hello, World!"
sus length normie = string_len(text)
sus upper tea = string_to_upper(text)
sus lower tea = string_to_lower(text)

// String searching
sus contains_hello lit = string_contains(text, "Hello")
sus starts_with_hello lit = string_starts_with(text, "Hello")
sus index normie = string_index_of(text, "World")

// String manipulation
sus trimmed tea = string_trim("  spaced  ")
sus reversed tea = string_reverse("hello")
sus repeated tea = string_repeat("abc", 3)

// String splitting and joining
sus words [tea] = string_split("a,b,c", ",")
sus joined tea = string_join(words, " ")

// Type conversion
sus number normie = string_to_int("123")
sus float_num meal = string_to_float("123.45")
sus bool_val lit = string_to_bool("true")

// String validation
sus is_numeric lit = string_is_numeric("123")
sus is_alpha lit = string_is_alpha("hello")
sus is_ascii lit = string_is_ascii("test")
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/string_pure/test_string_pure.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/string_pure/test_string_pure.csd
./test_string_pure
```

## Implementation Notes

### Pure CURSED Design
- No FFI dependencies or external libraries
- Uses only native CURSED language features
- Simplified implementations for core functionality
- Optimized for common use cases

### Performance Considerations
- Basic implementations prioritize correctness over performance
- Suitable for most string processing tasks
- Can be extended with more sophisticated algorithms

### Backward Compatibility
- Maintains identical function signatures to original string module
- Drop-in replacement for FFI-based string functions
- All existing code continues to work unchanged

## Architecture

### FFI Elimination Strategy
1. **Function Mapping**: All FFI functions replaced with pure CURSED implementations
2. **Simplified Algorithms**: Core algorithms implemented using basic CURSED constructs
3. **Test-Driven Development**: Comprehensive test suite ensures correctness
4. **Incremental Migration**: Can be adopted gradually alongside existing modules

### Design Patterns
- **Immutable Strings**: String operations return new strings
- **Null Safety**: Proper handling of empty and null strings
- **Error Handling**: Graceful degradation for edge cases
- **Consistent API**: Uniform function naming and parameter conventions

## Future Enhancements

### Planned Features
- Advanced Unicode support
- More sophisticated regex implementation
- Performance optimizations
- Extended character set support

### Extensibility
- Module can be extended with additional string functions
- Compatible with other pure CURSED modules
- Supports custom string processing algorithms

## Migration Guide

### From FFI String Module
1. Replace `yeet "string"` with `yeet "string_pure"`
2. No code changes required - identical API
3. Test both interpretation and compilation modes
4. Verify performance meets requirements

### Integration Steps
1. Import the string_pure module
2. Run existing tests to verify compatibility
3. Migrate gradually from FFI-based functions
4. Update imports and dependencies

## Status

- **Implementation**: Complete with all core functions
- **Testing**: Comprehensive test suite with 17 test categories
- **Compatibility**: Drop-in replacement for FFI string module
- **Performance**: Optimized for common use cases
- **Documentation**: Complete API documentation and examples
