# CURSED String Library Tests

This directory contains comprehensive tests for the CURSED string standard library.

## Test Coverage

The `test_string.csd` file provides complete test coverage for all string functions:

### String Properties
- `string_len()` - String length calculation
- `string_is_empty()` - Empty string detection

### Case Conversion
- `string_to_upper()` - Convert to uppercase
- `string_to_lower()` - Convert to lowercase
- `string_capitalize()` - Capitalize first letter

### String Trimming
- `string_trim()` - Remove leading/trailing whitespace
- `string_trim_start()` - Remove leading whitespace
- `string_trim_end()` - Remove trailing whitespace

### String Search
- `string_contains()` - Check if substring exists
- `string_starts_with()` - Check prefix
- `string_ends_with()` - Check suffix
- `string_index_of()` - Find first occurrence
- `string_last_index_of()` - Find last occurrence
- `string_count_occurrences()` - Count substring occurrences

### String Slicing
- `string_slice()` - Extract substring by indices
- `string_substring()` - Extract substring by start/length
- `string_char_at()` - Get character at index

### String Splitting
- `string_split()` - Split by delimiter
- `string_split_lines()` - Split by line breaks
- `string_split_whitespace()` - Split by whitespace

### String Replacement
- `string_replace()` - Replace first occurrence
- `string_replace_all()` - Replace all occurrences
- `string_repeat()` - Repeat string N times

### String Padding
- `string_pad_left()` - Left-pad with characters
- `string_pad_right()` - Right-pad with characters
- `string_pad_center()` - Center-pad with characters

### String Validation
- `string_is_numeric()` - Check if numeric
- `string_is_alpha()` - Check if alphabetic
- `string_is_alphanumeric()` - Check if alphanumeric
- `string_is_whitespace()` - Check if whitespace
- `string_is_ascii()` - Check if ASCII

### Type Conversion
- `string_to_int()` - Parse integer
- `string_to_float()` - Parse float
- `string_to_bool()` - Parse boolean
- `string_from_int()` - Convert from integer
- `string_from_float()` - Convert from float
- `string_from_bool()` - Convert from boolean

### String Utilities
- `string_reverse()` - Reverse string
- `string_join()` - Join array of strings
- `string_hash()` - Calculate hash
- `string_levenshtein_distance()` - Edit distance
- `string_similarity()` - String similarity metric

### Regular Expressions
- `regex_match()` - Check if pattern matches
- `regex_find()` - Find first match
- `regex_find_all()` - Find all matches
- `regex_replace()` - Replace with regex
- `regex_split()` - Split with regex

### Edge Cases Tested
- Empty string operations
- Single character strings
- Unicode character handling
- Very long strings
- Invalid input handling

## Running Tests

```bash
# Run string tests specifically
cargo run --bin cursed stdlib/string/test_string.csd

# Run all stdlib tests
cargo run --bin cursed test
```

## Test Results

All tests verify:
- Correct string manipulation
- Proper Unicode handling
- Expected behavior with edge cases
- Performance with large strings
- Integration with other stdlib modules

The tests ensure that the string library functions work correctly in both interpretation and native compilation modes.
