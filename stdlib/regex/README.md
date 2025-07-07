# CURSED Regex Module

Pure CURSED implementation for pattern matching and text processing operations.

## Overview

The regex module provides comprehensive pattern matching capabilities without external dependencies. It implements various pattern matching algorithms including wildcard matching, basic regex operations, and common validation patterns.

## Features

### Pattern Matching
- Exact string matching
- Wildcard matching with `*` and `?` support
- Pattern validation and syntax checking
- Case-sensitive matching

### Text Processing
- Find all matches in text
- Replace patterns with text
- Split text by patterns
- Extract pattern groups

### Character Classes
- Digit detection (`is_digit`)
- Letter detection (`is_letter`)
- Whitespace detection (`is_whitespace`)
- Alphanumeric detection (`is_alphanumeric`)

### Validation Functions
- Email address validation
- URL validation
- Phone number validation
- IP address validation

### Utilities
- Count pattern matches
- Get match positions
- Check if text contains pattern
- Complex pattern analysis

## Usage Examples

```cursed
yeet "regex"

// Basic pattern matching
lowkey regex.match_pattern("hello", "hello") {
    vibez.spill("Exact match found!")
}

// Wildcard matching
lowkey regex.match_wildcard("hello", "h*o") {
    vibez.spill("Wildcard match found!")
}

// Find all matches
sus matches [tea] = regex.find_matches("test test test", "test")
vibez.spill("Found " + tea(len(matches)) + " matches")

// Replace patterns
sus result tea = regex.replace_all_patterns("hello world", "hello", "hi")
vibez.spill(result)  // "hi world"

// Email validation
lowkey regex.is_valid_email("user@example.com") {
    vibez.spill("Valid email address")
}

// Character classification
lowkey regex.is_digit("5") {
    vibez.spill("Character is a digit")
}

// Split by pattern
sus parts [tea] = regex.split_by_pattern("a,b,c", ",")
bestie i := 0; i < len(parts); i++ {
    vibez.spill("Part: " + parts[i])
}
```

## API Reference

### Core Functions

#### `match_pattern(text tea, pattern tea) lit`
Check if text exactly matches pattern.

#### `match_wildcard(text tea, pattern tea) lit`
Match pattern with wildcard support (`*` and `?`).

#### `find_matches(text tea, pattern tea) [tea]`
Find all occurrences of pattern in text.

#### `find_all_matches(text tea, pattern tea) [MatchResult]`
Find all matches with detailed position information.

#### `replace_pattern(text tea, pattern tea, replacement tea) tea`
Replace first occurrence of pattern with replacement.

#### `replace_all_patterns(text tea, pattern tea, replacement tea) tea`
Replace all occurrences of pattern with replacement.

#### `split_by_pattern(text tea, pattern tea) [tea]`
Split text into array using pattern as delimiter.

### Character Classes

#### `is_digit(char tea) lit`
Check if character is a digit (0-9).

#### `is_letter(char tea) lit`
Check if character is a letter (a-z, A-Z).

#### `is_whitespace(char tea) lit`
Check if character is whitespace (space, tab, newline).

#### `is_alphanumeric(char tea) lit`
Check if character is alphanumeric (letter or digit).

### Validation Functions

#### `is_valid_email(email tea) lit`
Validate email address format.

#### `is_valid_url(url tea) lit`
Validate URL format (http/https).

#### `is_valid_phone(phone tea) lit`
Validate phone number format.

#### `is_valid_ip(ip tea) lit`
Validate IPv4 address format.

### Utility Functions

#### `count_matches(text tea, pattern tea) normie`
Count number of pattern matches in text.

#### `contains_pattern(text tea, pattern tea) lit`
Check if text contains pattern.

#### `get_match_positions(text tea, pattern tea) [normie]`
Get starting positions of all matches.

#### `extract_groups(text tea, pattern tea) [tea]`
Extract capture groups from pattern match.

#### `is_valid_pattern(pattern tea) lit`
Validate pattern syntax.

## Data Structures

### MatchResult
```cursed
be_like MatchResult squad {
    text tea        // Matched text
    start normie    // Start position
    end normie      // End position
    length normie   // Match length
}
```

## Pattern Syntax

### Wildcards
- `*` - Matches zero or more characters
- `?` - Matches exactly one character

### Examples
- `h*` - Matches "h", "hello", "hi"
- `h?llo` - Matches "hello", "hallo" but not "hllo"
- `test*` - Matches "test", "testing", "test123"

## Performance Considerations

- Pattern matching uses simple algorithms optimized for readability
- For large texts, consider pre-processing patterns
- Wildcard matching has O(n*m) complexity in worst case
- Character class functions are O(1) operations

## Dependencies

- `string` module for string manipulation functions

## Testing

Run the test suite:
```bash
cargo run --bin cursed stdlib/regex/test_regex.csd
```

The test suite includes:
- Basic pattern matching tests
- Wildcard matching tests
- Text processing tests
- Character class tests
- Validation function tests
- Utility function tests

## Implementation Notes

This is a pure CURSED implementation that doesn't rely on external regex libraries. It provides essential pattern matching functionality suitable for most text processing needs while maintaining simplicity and readability.

The implementation focuses on:
- Correctness over performance
- Simplicity over advanced features
- Pure CURSED code without FFI dependencies
- Comprehensive test coverage

For complex regex patterns, consider using this module as a foundation and extending it with additional pattern matching capabilities as needed.
