# String Module

Enhanced string manipulation and processing module for CURSED.

## Overview

The `string` module provides comprehensive string manipulation, validation, and processing capabilities. It includes functions for string transformation, searching, slicing, formatting, and advanced operations like regex matching and distance calculations.

## Features

### Core String Operations
- **Length & Validation**: `string_len()`, `string_is_empty()`
- **Case Conversion**: `string_to_upper()`, `string_to_lower()`, `string_capitalize()`
- **Trimming**: `string_trim()`, `string_trim_start()`, `string_trim_end()`
- **Reversal**: `string_reverse()`

### String Searching & Matching
- **Search**: `string_contains()`, `string_starts_with()`, `string_ends_with()`
- **Indexing**: `string_index_of()`, `string_last_index_of()`, `string_count_occurrences()`

### String Slicing & Splitting
- **Slicing**: `string_slice()`, `string_substring()`, `string_char_at()`
- **Splitting**: `string_split()`, `string_split_lines()`, `string_split_whitespace()`

### String Formatting & Transformation
- **Replacement**: `string_replace()`, `string_replace_all()`
- **Repetition**: `string_repeat()`
- **Padding**: `string_pad_left()`, `string_pad_right()`, `string_pad_center()`
- **Formatting**: `string_format()`

### String Validation
- **Type Checking**: `string_is_numeric()`, `string_is_alpha()`, `string_is_alphanumeric()`
- **Content Validation**: `string_is_whitespace()`, `string_is_ascii()`

### String Conversion
- **To Types**: `string_to_int()`, `string_to_float()`, `string_to_bool()`
- **From Types**: `string_from_int()`, `string_from_float()`, `string_from_bool()`

### String Encoding
- **Byte Operations**: `string_to_bytes()`, `string_from_bytes()`
- **Escaping**: `string_escape()`, `string_unescape()`

### Regular Expressions
- **Pattern Matching**: `regex_match()`, `regex_find()`, `regex_find_all()`
- **Replacement**: `regex_replace()`, `regex_split()`

### String Utilities
- **Joining**: `string_join()`
- **Distance**: `string_levenshtein_distance()`, `string_similarity()`
- **Hashing**: `string_hash()`

## Usage Examples

```cursed
yeet "string"

// Basic operations
sus text tea = "  Hello, World!  "
sus length normie = string_len(text)
sus trimmed tea = string_trim(text)
sus upper tea = string_to_upper(trimmed)

// Search and replace
sus contains_hello lit = string_contains(text, "Hello")
sus replaced tea = string_replace(text, "World", "CURSED")

// Splitting and joining
sus words [tea] = string_split_whitespace(text)
sus rejoined tea = string_join(words, "-")

// Validation
sus is_numeric lit = string_is_numeric("123")
sus is_email lit = regex_match("\\S+@\\S+\\.\\S+", "user@example.com")
```

## Performance

The string module uses efficient algorithms for all operations:
- Linear time complexity for most operations
- Optimized pattern matching for regex operations
- Memory-efficient string building for concatenation

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/string/test_string.csd
```

## Status

✅ **Production Ready**: All functions implemented and tested
✅ **Pure CURSED**: No external dependencies
✅ **Cross-Platform**: Works on all supported platforms
✅ **Fully Tested**: Comprehensive test coverage
