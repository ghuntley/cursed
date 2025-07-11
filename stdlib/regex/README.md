# Regex Module

Advanced regular expression operations and pattern matching for CURSED.

## Overview

The `regex` module provides comprehensive pattern matching capabilities, including simple patterns, wildcards, and advanced regex operations. It includes validation functions for common patterns like emails, URLs, and phone numbers.

## Features

### Basic Pattern Matching
- **Exact Matching**: `match_pattern()` - Direct string comparison
- **Wildcard Matching**: `match_wildcard()` - Support for `*` and `?` wildcards
- **Simple Patterns**: `simple_pattern_match()` - Basic pattern matching

### Advanced Pattern Operations
- **Find Matches**: `find_matches()`, `find_all_matches()` - Locate pattern occurrences
- **Replace Patterns**: `replace_pattern()`, `replace_all_patterns()` - Pattern replacement
- **Split by Pattern**: `split_by_pattern()` - Split strings using patterns

### Pattern Analysis
- **Match Counting**: `count_matches()`, `contains_pattern()`
- **Position Finding**: `get_match_positions()` - Get all match positions
- **Group Extraction**: `extract_groups()` - Extract capture groups

### Character Classes
- **Digit Validation**: `is_digit()` - Check if character is numeric
- **Letter Validation**: `is_letter()` - Check if character is alphabetic
- **Whitespace**: `is_whitespace()` - Check for whitespace characters
- **Alphanumeric**: `is_alphanumeric()` - Check for letters and numbers

### Common Validations
- **Email**: `is_valid_email()` - RFC-compliant email validation
- **URL**: `is_valid_url()` - HTTP/HTTPS URL validation
- **Phone**: `is_valid_phone()` - Phone number validation
- **IP Address**: `is_valid_ip()` - IPv4 address validation

### Pattern Validation
- **Syntax Check**: `is_valid_pattern()` - Validate pattern syntax
- **Bracket Balancing**: Ensure proper bracket and parenthesis matching

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

## Usage Examples

```cursed
yeet "regex"

// Basic pattern matching
sus pattern tea = "hello"
sus text tea = "hello world"
sus matches lit = match_pattern(text, pattern)

// Wildcard matching
sus wildcard_pattern tea = "h*o"
sus wildcard_matches lit = match_wildcard("hello", wildcard_pattern)

// Find all matches
sus all_matches [tea] = find_matches("test test test", "test")
sus match_count normie = count_matches("hello", "l")

// Pattern replacement
sus replaced tea = replace_pattern("hello world", "world", "CURSED")
sus all_replaced tea = replace_all_patterns("test test", "test", "exam")

// Split by pattern
sus parts [tea] = split_by_pattern("a,b,c", ",")

// Character class validation
sus is_digit_5 lit = is_digit("5")
sus is_letter_a lit = is_letter("a")
sus is_whitespace_space lit = is_whitespace(" ")

// Common validations
sus valid_email lit = is_valid_email("user@example.com")
sus valid_url lit = is_valid_url("https://example.com")
sus valid_phone lit = is_valid_phone("123-456-7890")
sus valid_ip lit = is_valid_ip("192.168.1.1")

// Advanced pattern matching with positions
sus positions [normie] = get_match_positions("hello world hello", "hello")
sus match_results [MatchResult] = find_all_matches("test test", "test")
```

## Pattern Syntax

### Wildcard Patterns
- `*` - Matches zero or more characters
- `?` - Matches exactly one character
- `h*o` - Matches "hello", "hero", "ho", etc.
- `h?llo` - Matches "hello", "hallo", etc.

### Character Classes
- `[a-z]` - Lowercase letters
- `[A-Z]` - Uppercase letters
- `[0-9]` - Digits
- `[a-zA-Z0-9]` - Alphanumeric characters

### Common Patterns
- Email: `\S+@\S+\.\S+`
- URL: `https?://\S+`
- Phone: `\d{3}-\d{3}-\d{4}`
- IP: `\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}`

## Performance

The regex module is optimized for performance:
- Efficient pattern matching algorithms
- Minimal backtracking for simple patterns
- Optimized character class matching
- Fast string search for literal patterns

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/regex/test_regex.csd
```

## Status

✅ **Production Ready**: Core functionality implemented and tested
✅ **Pure CURSED**: No external regex engine dependencies
✅ **Cross-Platform**: Consistent behavior across all platforms
✅ **Extensible**: Easy to add new pattern types and validations
✅ **Fully Tested**: Comprehensive test coverage for all features
