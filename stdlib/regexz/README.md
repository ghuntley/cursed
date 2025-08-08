# regexz Module

The `regexz` module provides comprehensive regular expression functionality with pattern compilation, matching, searching, and replacement operations. It serves as a standardized alias for the `regex_vibez` module while maintaining the expected "z" suffix naming convention.

## Features

### Pattern Compilation
- Regex pattern compilation with caching
- Flag-based compilation (case-insensitive, multiline, etc.)
- Pattern validation and syntax checking
- Optimized pattern storage for performance

### Matching Operations
- Basic pattern matching (full, partial, start-anchored)
- Group extraction and named capture groups
- Multiple match finding with position information
- Case-sensitive and case-insensitive matching

### String Operations
- Find and replace with regex patterns
- String splitting by regex patterns
- Pattern-based text extraction
- Batch replacement operations

### Character Classes
- Built-in character class support (\d, \w, \s, etc.)
- Custom character class definitions
- Unicode character support
- Character validation functions

### Common Patterns
- Pre-compiled patterns for email, URL, phone, IP addresses
- Validation functions for common data types
- Pattern escaping and literal matching
- Performance-optimized common operations

## Usage Examples

### Basic Pattern Matching
```cursed
yeet "regexz"

// Simple pattern matching
sus matches lit = regexz.match("hello", "hello world")  // true
sus no_match lit = regexz.match("hello", "goodbye")     // false

// Pattern-based matching
sus has_digits lit = regexz.match("\\d+", "abc123def")  // true
sus only_letters lit = regexz.match("^[a-zA-Z]+$", "Hello")  // true
```

### Pattern Compilation
```cursed
// Compile pattern for reuse
sus pattern CompiledPattern = regexz.compile("\\d{3}-\\d{3}-\\d{4}")

// Compile with flags
sus case_insensitive CompiledPattern = regexz.compile_with_flags("hello", "i")
```

### Finding Matches
```cursed
// Find all matches
sus text tea = "Phone: 123-456-7890, Fax: 098-765-4321"
sus phone_matches []Match = regexz.find("\\d{3}-\\d{3}-\\d{4}", text)

// Find first match only
sus first_phone Match = regexz.find_first("\\d{3}-\\d{3}-\\d{4}", text)

// Count matches
sus phone_count normie = regexz.match_count("\\d{3}-\\d{3}-\\d{4}", text)
```

### String Replacement
```cursed
// Replace all occurrences
sus censored tea = regexz.replace("password", "My password is secret", "****")
// Result: "My **** is secret"

// Case-insensitive replacement
sus normalized tea = regexz.replace_ignore_case("HELLO", "hello world", "hi")
// Result: "hi world"
```

### String Splitting
```cursed
// Split by pattern
sus csv_data tea = "apple,banana,cherry"
sus fruits []tea = regexz.split(",", csv_data)
// Result: ["apple", "banana", "cherry"]

// Split with limit
sus limited []tea = regexz.split_limit("\\s+", "one two three four", 2)
// Result: ["one", "two three four"]
```

### Character Class Validation
```cursed
// Built-in character classes
sus is_num lit = regexz.is_digit("5")        // true
sus is_word lit = regexz.is_word_char("_")   // true  
sus is_space lit = regexz.is_whitespace(" ") // true

// Pattern-based validation
sus valid_email lit = regexz.is_email("user@example.com")     // true
sus valid_url lit = regexz.is_url("https://www.example.com")  // true
sus valid_phone lit = regexz.is_phone("+1234567890")          // true
sus valid_ipv4 lit = regexz.is_ipv4("192.168.1.1")          // true
```

### Advanced Operations
```cursed
// Group extraction
sus phone_text tea = "Call me at (555) 123-4567"
sus groups []tea = regexz.match_groups("\\((\\d{3})\\) (\\d{3})-(\\d{4})", phone_text)
// Result: ["555", "123", "4567"]

// Escape special characters
sus literal tea = regexz.escape("Hello. How are you?")
// Result: "Hello\\. How are you\\?"

// Quote for literal matching
sus quoted tea = regexz.quote("user@domain.com")
```

### Common Pattern Validation
```cursed
// Pre-defined validation patterns
sus email_valid lit = regexz.is_email("test@example.com")
sus url_valid lit = regexz.is_url("https://github.com/user/repo")
sus phone_valid lit = regexz.is_phone("+1-555-123-4567")
sus ipv4_valid lit = regexz.is_ipv4("10.0.0.1")
sus ipv6_valid lit = regexz.is_ipv6("2001:db8::1")
```

### Pattern Validation
```cursed
// Check if pattern is valid
sus valid_syntax lit = regexz.is_valid_pattern("\\d+")      // true
sus invalid_syntax lit = regexz.is_valid_pattern("[invalid") // false

// Detailed validation
sus validation ValidationResult = regexz.validate_regex("\\d{3,5}")
ready (validation.is_valid) {
    vibez.spill("Pattern is valid")
} yikes {
    vibez.spill("Error:", validation.error_message)
}
```

## Data Types

### Core Types
- `CompiledPattern` - Compiled regex pattern
- `Match` - Match result with position information
- `ValidationResult` - Pattern validation result
- `NamedGroups` - Named capture group results

### Match Structure
```cursed
Match{
    start: normie,    // Start position of match
    end: normie,      // End position of match  
    text: tea         // Matched text content
}
```

### Validation Result
```cursed
ValidationResult{
    is_valid: lit,         // Whether pattern is valid
    error_message: tea,    // Error description if invalid
    error_position: normie // Position of syntax error
}
```

## Function Reference

### Pattern Compilation
- `compile(pattern)` - Compile regex pattern
- `compile_with_flags(pattern, flags)` - Compile with flags
- `compile_optimized(pattern)` - Compile with performance optimization

### Basic Matching
- `match(pattern, text)` - Test if pattern matches
- `match_start(pattern, text)` - Match at start of string
- `match_full(pattern, text)` - Match entire string
- `match_ignore_case(pattern, text)` - Case-insensitive match

### Finding Operations
- `find(pattern, text)` - Find all matches
- `find_all(pattern, text)` - Alias for find
- `find_first(pattern, text)` - Find first match only
- `match_count(pattern, text)` - Count number of matches
- `contains_pattern(pattern, text)` - Check if pattern exists

### String Operations
- `replace(pattern, text, replacement)` - Replace all matches
- `replace_all(pattern, text, replacement)` - Alias for replace
- `replace_ignore_case(pattern, text, replacement)` - Case-insensitive replace
- `split(pattern, text)` - Split string by pattern
- `split_limit(pattern, text, max_splits)` - Split with maximum splits

### Group Operations
- `match_groups(pattern, text)` - Extract capture groups
- `named_groups(pattern, text)` - Extract named groups

### Character Classes
- `is_digit(char)` - Test if character is digit
- `is_word_char(char)` - Test if character is word character
- `is_whitespace(char)` - Test if character is whitespace

### Validation Functions
- `is_email(text)` - Validate email address
- `is_url(text)` - Validate URL
- `is_phone(text)` - Validate phone number
- `is_ipv4(text)` - Validate IPv4 address
- `is_ipv6(text)` - Validate IPv6 address

### Pattern Utilities
- `escape(text)` - Escape regex special characters
- `quote(text)` - Quote text for literal matching
- `is_valid_pattern(pattern)` - Check pattern syntax
- `validate_regex(pattern)` - Detailed pattern validation

### Pre-defined Patterns
- `email_pattern()` - Email validation pattern
- `url_pattern()` - URL validation pattern
- `phone_pattern()` - Phone number pattern
- `ipv4_pattern()` - IPv4 address pattern
- `ipv6_pattern()` - IPv6 address pattern

### Performance Operations
- `benchmark_pattern(pattern, text, iterations)` - Benchmark pattern performance

## Pattern Syntax

### Basic Patterns
- `.` - Any character except newline
- `*` - Zero or more of preceding
- `+` - One or more of preceding
- `?` - Zero or one of preceding
- `^` - Start of string
- `$` - End of string

### Character Classes
- `\d` - Digit character [0-9]
- `\D` - Non-digit character
- `\w` - Word character [a-zA-Z0-9_]
- `\W` - Non-word character
- `\s` - Whitespace character
- `\S` - Non-whitespace character

### Quantifiers
- `{n}` - Exactly n occurrences
- `{n,}` - n or more occurrences
- `{n,m}` - Between n and m occurrences

### Groups
- `(pattern)` - Capture group
- `(?:pattern)` - Non-capturing group
- `(?P<name>pattern)` - Named capture group

### Character Sets
- `[abc]` - Any of a, b, or c
- `[a-z]` - Any lowercase letter
- `[^abc]` - Any character except a, b, or c

## Flags

### Compilation Flags
- `i` - Case insensitive matching
- `m` - Multiline mode (^ and $ match line boundaries)
- `s` - Dot matches newline
- `x` - Extended syntax (ignore whitespace)
- `g` - Global matching (find all matches)

## Performance Considerations

### Pattern Compilation
- Compile patterns once and reuse for multiple operations
- Use optimized compilation for frequently used patterns
- Cache compiled patterns for better performance
- Consider pattern complexity and optimization

### Memory Usage
- Compiled patterns use memory for state machines
- Large text processing may require streaming approaches
- Group extraction allocates memory for results
- Consider memory limits for very large operations

### Optimization Tips
- Use non-capturing groups when capture is not needed
- Anchor patterns when possible (^ and $)
- Use character classes instead of alternation when possible
- Benchmark patterns for performance-critical applications

## Error Handling

### Pattern Errors
- Invalid regex syntax
- Unsupported regex features
- Pattern compilation failures
- Resource exhaustion errors

### Runtime Errors
- Text encoding issues
- Memory allocation failures
- Timeout for complex patterns
- Invalid flag combinations

## Testing
Run the comprehensive test suite:
```bash
./zig-out/bin/cursed stdlib/regexz/test_regexz.csd
```

The test suite covers:
- Pattern compilation and validation
- Basic and advanced matching operations
- String replacement and splitting
- Character class validation
- Common pattern validation
- Group extraction and named groups
- Error handling scenarios
- Performance benchmarking

## Implementation Notes

### Module Relationship
The `regexz` module is an alias wrapper around `regex_vibez`, providing:
- Standardized naming convention (z suffix)
- Backward compatibility
- Consistent API surface
- All functionality from `regex_vibez`

### Engine Features
- DFA/NFA hybrid engine for optimal performance
- Unicode support for international text
- Backtracking prevention for security
- Memory-safe pattern compilation

### Compatibility
- PCRE-compatible syntax where possible
- JavaScript regex compatibility for common patterns
- Cross-platform consistent behavior
- UTF-8 text processing support

## Best Practices

### Pattern Design
- Use specific patterns instead of overly broad ones
- Anchor patterns when you know the position
- Use non-capturing groups to improve performance
- Test patterns with edge cases and malicious input

### Security Considerations
- Validate input before regex processing
- Set timeouts for complex pattern matching
- Use pattern compilation validation
- Avoid user-controlled regex patterns when possible

### Performance Optimization
- Compile patterns once and reuse
- Use character classes instead of alternation
- Consider pattern anchoring for better performance
- Profile regex performance in critical paths

### Error Handling
- Always validate patterns before compilation
- Handle pattern compilation errors gracefully
- Provide meaningful error messages to users
- Use fallback patterns for non-critical validations
